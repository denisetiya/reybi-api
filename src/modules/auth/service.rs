use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::config::AppConfig;
use crate::errors::{AppError, AppResult};
use super::dto::{RegisterRequest, AuthResponse};

#[derive(Debug, serde::Serialize)]
pub struct UserSummary {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: String,
    pub role: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    id: String,
    email: String,
    exp: usize,
    iat: usize,
}

pub struct AuthService;

impl AuthService {
    pub async fn login(
        db: &PgPool,
        config: &AppConfig,
        firebase_token: &str,
    ) -> AppResult<AuthResponse> {
        let user_info = validate_firebase_token(config, firebase_token).await?;
        let user = find_or_create_user(db, &user_info).await?;
        let tokens = generate_tokens(&user, config)?;
        Ok(AuthResponse {
            token: tokens.0,
            refresh_token: tokens.1,
            user: UserSummary {
                id: user.id,
                name: user.name,
                email: user.email,
                role: user.role.unwrap_or_else(|| "user".into()),
            },
        })
    }

    pub async fn register(
        db: &PgPool,
        _firebase_token: &str,
        data: RegisterRequest,
    ) -> AppResult<serde_json::Value> {
        let id = Uuid::new_v4();
        sqlx::query(
            r#"INSERT INTO users (id, name, email, role) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"#
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.email)
        .bind("user")
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(serde_json::json!({
            "success": true,
            "data": {
                "id": id,
                "name": data.name,
                "email": data.email
            },
            "meta": { "locale": "en" }
        }))
    }
}

async fn validate_firebase_token(
    config: &AppConfig,
    token: &str,
) -> AppResult<FirebaseUser> {
    use reqwest::Client;
    let client = Client::new();
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
        config.key_server
    );
    let resp = client.post(&url)
        .json(&serde_json::json!({ "idToken": token }))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let body: serde_json::Value = resp.json().await
        .map_err(|e| AppError::Internal(e.into()))?;

    let email = body["users"][0]["email"].as_str()
        .ok_or_else(|| AppError::Validation(vec![crate::errors::FieldError {
            path: "token".into(),
            message: "Invalid Firebase token".into(),
        }]))?;

    let name = body["users"][0]["displayName"].as_str().map(String::from);

    Ok(FirebaseUser {
        email: email.to_string(),
        name,
    })
}

struct FirebaseUser {
    email: String,
    name: Option<String>,
}

async fn find_or_create_user(
    db: &PgPool,
    firebase: &FirebaseUser,
) -> AppResult<crate::models::User> {
    

    let existing = sqlx::query_as::<_, crate::models::User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&firebase.email)
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    if let Some(user) = existing {
        return Ok(user);
    }

    let id = Uuid::new_v4();
    sqlx::query_as::<_, crate::models::User>(
        r#"INSERT INTO users (id, email, name, role) VALUES ($1, $2, $3, $4) RETURNING *"#
    )
    .bind(id)
    .bind(&firebase.email)
    .bind(&firebase.name)
    .bind("user")
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}

fn generate_tokens(
    user: &crate::models::User,
    config: &AppConfig,
) -> AppResult<(String, String)> {
    let now = Utc::now();
    let access_claims = Claims {
        id: user.id.to_string(),
        email: user.email.clone(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::hours(3)).timestamp() as usize,
    };
    let refresh_claims = Claims {
        id: user.id.to_string(),
        email: user.email.clone(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::days(7)).timestamp() as usize,
    };
    let access = encode(&Header::default(), &access_claims,
        &EncodingKey::from_secret(config.jwt_access_secret.as_bytes()))
        .map_err(|_| AppError::Internal(anyhow::anyhow!("token failed")))?;
    let refresh = encode(&Header::default(), &refresh_claims,
        &EncodingKey::from_secret(config.jwt_refresh_secret.as_bytes()))
        .map_err(|_| AppError::Internal(anyhow::anyhow!("refresh token failed")))?;
    Ok((access, refresh))
}
