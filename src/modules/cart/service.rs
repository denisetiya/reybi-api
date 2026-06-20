use super::dto::AddCartRequest;
use crate::errors::{AppError, AppResult};
use crate::models::Cart;
use sqlx::PgPool;

pub struct CartService;

impl CartService {
    pub async fn get(db: &PgPool, user_id: String, limit: i64) -> AppResult<Vec<Cart>> {
        sqlx::query_as::<_, Cart>(
            "SELECT * FROM carts WHERE user_id=$1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(&user_id)
        .bind(limit + 1)
        .fetch_all(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn add(db: &PgPool, user_id: String, data: AddCartRequest) -> AppResult<Cart> {
        let existing =
            sqlx::query_as::<_, Cart>("SELECT * FROM carts WHERE user_id=$1 AND product_id=$2")
                .bind(&user_id)
                .bind(&data.product_id)
                .fetch_optional(db)
                .await
                .map_err(|e| AppError::Internal(e.into()))?;

        if let Some(item) = existing {
            let new_qty = item.quantity + data.quantity;
            if new_qty <= 0 {
                sqlx::query("DELETE FROM carts WHERE id=$1")
                    .bind(&item.id)
                    .execute(db)
                    .await
                    .map_err(|e| AppError::Internal(e.into()))?;
                return Ok(item);
            }
            return sqlx::query_as::<_, Cart>(
                "UPDATE carts SET quantity=$2, updated_at=NOW() WHERE id=$1 RETURNING *",
            )
            .bind(&item.id)
            .bind(new_qty)
            .fetch_one(db)
            .await
            .map_err(|e| AppError::Internal(e.into()));
        }

        let id = cuid2::create_id();
        sqlx::query_as::<_, Cart>(
            r#"INSERT INTO carts (id, user_id, product_id, quantity, variant_id) VALUES ($1,$2,$3,$4,$5) RETURNING *"#
        ).bind(id).bind(&user_id).bind(&data.product_id).bind(data.quantity).bind(data.variant_id)
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn delete(db: &PgPool, id: String) -> AppResult<()> {
        let r = sqlx::query("DELETE FROM carts WHERE id=$1")
            .bind(&id)
            .execute(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        if r.rows_affected() == 0 {
            return Err(AppError::NotFound("Cart item not found".into()));
        }
        Ok(())
    }
}
