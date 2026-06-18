use sqlx::PgPool;
use uuid::Uuid;
use crate::errors::{AppError, AppResult};
use crate::models::Banner;

pub struct BannerService;

impl BannerService {
    pub async fn list(db: &PgPool, r#type: Option<&str>, limit: i64) -> AppResult<Vec<Banner>> {
        let mut query = String::from("SELECT * FROM banners");
        let limit_with_one = limit + 1;
        if let Some(t) = r#type {
            query.push_str(&format!(" WHERE type = '{}'", t.replace("'", "''")));
        }
        query.push_str(&format!(" ORDER BY created_at DESC LIMIT {limit_with_one}"));
        sqlx::query_as::<_, Banner>(&query)
            .fetch_all(db).await.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn create(db: &PgPool, image: &str, r#type: Option<&str>) -> AppResult<Banner> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Banner>(
            r#"INSERT INTO banners (id, image, type) VALUES ($1,$2,$3) RETURNING *"#
        )
        .bind(id).bind(image).bind(r#type)
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))
    }
}
