use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;

use super::dto::{AuthResponse, RegisterRequest};
use crate::config::AppConfig;
use crate::errors::{AppError, AppResult};

#[derive(Debug, serde::Serialize)]
pub struct UserSummary {
    pub id: String,
    pub name: Option<String>,
    pub email: String,
    pub role: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    id: String,
    email: String,
    role: String,
    exp: usize,
    iat: usize,
}

pub struct AuthService;

impl AuthService {
    pub async fn login(
        db: &PgPool,
        config: &AppConfig,
        firebase: &crate::utils::firebase::FirebaseVerifier,
        firebase_token: &str,
    ) -> AppResult<AuthResponse> {
        let user_info = validate_firebase_token(firebase, firebase_token)?;
        let user = find_or_create_user(db, &user_info, RegisterRequest::default()).await?;
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
        config: &AppConfig,
        firebase: &crate::utils::firebase::FirebaseVerifier,
        firebase_token: &str,
        overrides: RegisterRequest,
    ) -> AppResult<AuthResponse> {
        let firebase_user = validate_firebase_token(firebase, firebase_token)?;
        let user = find_or_create_user(db, &firebase_user, overrides).await?;
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
}

/// Verify a Firebase ID token server-side against Google's public keys
/// (scoped to our project_id).  No API key required — the signature +
/// audience/issuer claims are validated locally using cached JWKS.
fn validate_firebase_token(
    firebase: &crate::utils::firebase::FirebaseVerifier,
    token: &str,
) -> AppResult<FirebaseUser> {
    let user = firebase.verify(token).ok_or_else(|| {
        AppError::Validation(vec![crate::errors::FieldError {
            path: "token".into(),
            message: "Invalid or expired Firebase token".into(),
        }])
    })?;

    let email = user.email.clone().ok_or_else(|| {
        AppError::Validation(vec![crate::errors::FieldError {
            path: "token".into(),
            message: "Firebase token has no email".into(),
        }])
    })?;

    Ok(FirebaseUser {
        email,
        name: user.name.clone(),
    })
}

struct FirebaseUser {
    email: String,
    name: Option<String>,
}

async fn find_or_create_user(
    db: &PgPool,
    firebase: &FirebaseUser,
    overrides: RegisterRequest,
) -> AppResult<crate::models::User> {
    let existing = sqlx::query_as::<_, crate::models::User>("SELECT * FROM users WHERE email = $1")
        .bind(&firebase.email)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    if let Some(user) = existing {
        return Ok(user);
    }

    // New user: prefer Firebase profile fields; allow client overrides.
    let id = cuid2::create_id();
    let name = overrides.name.or(firebase.name.clone());
    let photo = overrides.photo_url;
    let row: crate::models::User = sqlx::query_as::<_, crate::models::User>(
        r#"INSERT INTO users (id, email, name, role, "photoUrl")
           VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
    )
    .bind(&id)
    .bind(&firebase.email)
    .bind(&name)
    .bind("user")
    .bind(&photo)
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;
    Ok(row)
}

fn generate_tokens(user: &crate::models::User, config: &AppConfig) -> AppResult<(String, String)> {
    let now = Utc::now();
    let role = user.role.clone().unwrap_or_else(|| "user".into());
    let access_claims = Claims {
        id: user.id.clone(),
        email: user.email.clone(),
        role: role.clone(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::hours(3)).timestamp() as usize,
    };
    let refresh_claims = Claims {
        id: user.id.clone(),
        email: user.email.clone(),
        role,
        iat: now.timestamp() as usize,
        exp: (now + Duration::days(7)).timestamp() as usize,
    };
    let access = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(config.jwt_access_secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal(anyhow::anyhow!("token failed")))?;
    let refresh = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(config.jwt_refresh_secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal(anyhow::anyhow!("refresh token failed")))?;
    Ok((access, refresh))
}
