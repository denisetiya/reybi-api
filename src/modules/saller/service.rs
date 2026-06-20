use sqlx::PgPool;
use crate::errors::{AppError, AppResult};
use crate::models::Product;

pub struct SallerService;

impl SallerService {
    pub async fn get_products(db: &PgPool, saller_id: String, limit: i64) -> AppResult<Vec<Product>> {
        sqlx::query_as::<_, Product>(
            "SELECT * FROM products WHERE saller_id=$1 ORDER BY created_at DESC LIMIT $2"
        ).bind(saller_id).bind(limit + 1).fetch_all(db).await.map_err(|e| AppError::Internal(e.into()))
    }
}
