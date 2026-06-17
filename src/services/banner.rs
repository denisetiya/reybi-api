use sqlx::PgPool;
use uuid::Uuid;
use crate::dto::PaginationQuery;
use crate::errors::{AppError, AppResult};

pub struct BannerService;

impl BannerService {
    pub async fn list(
        db: &PgPool,
        r#type: Option<&str>,
        pq: &PaginationQuery,
    ) -> AppResult<Vec<crate::models::Banner>> {
        let limit = pq.take();
        let mut query = String::from("SELECT * FROM banners");
        if let Some(t) = r#type {
            query.push_str(&format!(" WHERE type = '{}'", t.replace('\'', "''")));
        }
        query.push_str(" ORDER BY created_at DESC");
        query.push_str(&format!(" LIMIT {limit}"));
        let rows = sqlx::query_as::<_, crate::models::Banner>(&query)
            .fetch_all(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        Ok(rows)
    }

    pub async fn create(
        db: &PgPool,
        image: &str,
        r#type: Option<&str>,
    ) -> AppResult<crate::models::Banner> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, crate::models::Banner>(
            r#"INSERT INTO banners (id, image, type) VALUES ($1, $2, $3) RETURNING *"#
        )
        .bind(id)
        .bind(image)
        .bind(r#type)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))
    }
}
