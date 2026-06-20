use super::dto::UpdateProfileRequest;
use crate::errors::{AppError, AppResult};
use crate::models::User;
use sqlx::PgPool;

pub struct ProfileService;

impl ProfileService {
    pub async fn get_by_email(db: &PgPool, email: &str) -> AppResult<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?
            .ok_or_else(|| AppError::NotFound("User not found".into()))
    }

    pub async fn update(db: &PgPool, email: &str, data: UpdateProfileRequest) -> AppResult<User> {
        let existing = Self::get_by_email(db, email).await?;
        sqlx::query_as::<_, User>(
            r#"UPDATE users SET name=COALESCE($2,name), photo_url=COALESCE($3,photo_url),
               role=COALESCE($4,role), phone_number=COALESCE($5,phone_number), updated_at=NOW()
               WHERE email=$1 RETURNING *"#,
        )
        .bind(email)
        .bind(
            data.name
                .as_deref()
                .unwrap_or(existing.name.as_deref().unwrap_or("")),
        )
        .bind(data.photo_url.as_deref().or(existing.photo_url.as_deref()))
        .bind(
            data.role
                .as_deref()
                .unwrap_or(existing.role.as_deref().unwrap_or("user")),
        )
        .bind(
            data.phone_number
                .as_deref()
                .unwrap_or(existing.phone_number.as_deref().unwrap_or("")),
        )
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))
    }
}
