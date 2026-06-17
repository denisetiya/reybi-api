use sqlx::PgPool;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::errors::{AppError, AppResult};
use crate::dto::{AuthResponse, RegisterRequest};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct AuthService;

impl AuthService {
    /// Validate a Firebase ID token via Firebase REST API.
    /// In production, replace this with the firebase-rs or google-authz crate.
    pub async fn validate_firebase_token(
        token: &str,
    ) -> AppResult<ValidatedUser> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=AIzaSyD_placeholder"
        );
        let resp = client
            .post(&url)
            .json(&serde_json::json!({ "idToken": token }))
            .send()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        let users = body["users"]
            .as_array()
            .ok_or_else(|| AppError::Unauthorized)?;

        let user = users.first().ok_or_else(|| AppError::Unauthorized)?;

        Ok(ValidatedUser {
            uid: user["localId"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            email: user["email"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            display_name: user["displayName"]
                .as_str()
                .unwrap_or("User")
                .to_string(),
            photo_url: user["photoUrl"]
                .as_str()
                .map(String::from),
        })
    }

    pub fn generate_tokens(
        config: &AppConfig,
        user_id: &str,
        email: &str,
    ) -> AppResult<(String, String)> {
        let now = chrono::Utc::now().timestamp() as usize;

        let access_claims = Claims {
            id: user_id.to_string(),
            email: email.to_string(),
            iat: now,
            exp: now + 10800, // 3h
        };

        let refresh_claims = Claims {
            id: user_id.to_string(),
            email: email.to_string(),
            iat: now,
            exp: now + 604800, // 7d
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(config.jwt_access_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(e.into()))?;

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(config.jwt_refresh_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok((access_token, refresh_token))
    }

    pub async fn login(
        db: &PgPool,
        config: &AppConfig,
        firebase_token: &str,
    ) -> AppResult<AuthResponse> {
        let user_info = Self::validate_firebase_token(firebase_token).await?;

        // Upsert user
        sqlx::query(
            r#"INSERT INTO users (fb_id, email, name, photo_url)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (fb_id)
               DO UPDATE SET email = EXCLUDED.email, name = EXCLUDED.name, photo_url = COALESCE(EXCLUDED.photo_url, users.photo_url)"#
        )
        .bind(&user_info.uid)
        .bind(&user_info.email)
        .bind(&user_info.display_name)
        .bind(&user_info.photo_url)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let user = sqlx::query_as::<_, crate::models::User>(
            "SELECT * FROM users WHERE fb_id = $1"
        )
        .bind(&user_info.uid)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        let (access_token, refresh_token) =
            Self::generate_tokens(config, &user.id.to_string(), &user.email)?;

        // Upsert refresh token
        sqlx::query(
            r#"INSERT INTO tokens (user_id, refresh_token)
               VALUES ($1, $2)
               ON CONFLICT (user_id)
               DO UPDATE SET refresh_token = EXCLUDED.refresh_token"#
        )
        .bind(user.id)
        .bind(&refresh_token)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(AuthResponse {
            email: user.email,
            name: user.name.unwrap_or_default(),
            role: user.role.unwrap_or_else(|| "user".into()),
            photo_url: user.photo_url,
            phone_number: user.phone_number,
            access_token,
            refresh_token,
        })
    }

    pub async fn register(
        db: &PgPool,
        firebase_token: &str,
        data: RegisterRequest,
    ) -> AppResult<serde_json::Value> {
        let user_info = Self::validate_firebase_token(firebase_token).await?;

        sqlx::query(
            r#"INSERT INTO users (fb_id, email, name, phone_number)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (fb_id) DO NOTHING"#
        )
        .bind(&user_info.uid)
        .bind(&user_info.email)
        .bind(&data.name)
        .bind(&data.phone_number)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(serde_json::json!({
            "success": true,
            "message": "Registration successful",
            "data": { "email": user_info.email, "name": data.name }
        }))
    }
}

pub struct ValidatedUser {
    pub uid: String,
    pub email: String,
    pub display_name: String,
    pub photo_url: Option<String>,
}
