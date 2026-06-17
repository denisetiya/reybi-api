use sqlx::PgPool;
use uuid::Uuid;
use crate::dto::PaginationQuery;
use crate::errors::{AppError, AppResult};
use crate::models::Product;

pub struct SallerService;

impl SallerService {
    pub async fn get_products(
        db: &PgPool,
        saller_id: Uuid,
        pq: &PaginationQuery,
    ) -> AppResult<Vec<Product>> {
        let limit = pq.take();
        let rows = sqlx::query_as::<_, Product>(
            "SELECT * FROM products WHERE saller_id = $1 ORDER BY created_at DESC LIMIT $2"
        )
        .bind(saller_id)
        .bind(limit)
        .fetch_all(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
        Ok(rows)
    }
}
