use super::dto::CreateOrderRequest;
use crate::errors::{AppError, AppResult};
use crate::models::Order;
use sqlx::PgPool;

pub struct OrderService;

impl OrderService {
    pub async fn create(
        db: &PgPool,
        user_id: String,
        data: CreateOrderRequest,
    ) -> AppResult<Order> {
        let id = cuid2::create_id();
        let order = sqlx::query_as::<_, Order>(
            r#"INSERT INTO orders (id, user_id, product_id, quantity, coin) VALUES ($1,$2,$3,$4,$5) RETURNING *"#
        ).bind(&id).bind(&user_id).bind(&data.product_id).bind(data.quantity).bind(data.coin)
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))?;

        let pay_id = cuid2::create_id();
        sqlx::query(
            r#"INSERT INTO payment_histories (id, order_id, method, type, amount) VALUES ($1,$2,$3,$4,$5)"#
        ).bind(pay_id).bind(id).bind(&data.payment.method).bind(&data.payment.r#type).bind(data.payment.amount)
        .execute(db).await.map_err(|e| AppError::Internal(e.into()))?;
        Ok(order)
    }

    pub async fn get_by_user(db: &PgPool, user_id: String, limit: i64) -> AppResult<Vec<Order>> {
        sqlx::query_as::<_, Order>(
            "SELECT * FROM orders WHERE user_id=$1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(&user_id)
        .bind(limit + 1)
        .fetch_all(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn get_all(db: &PgPool, limit: i64) -> AppResult<Vec<Order>> {
        sqlx::query_as::<_, Order>("SELECT * FROM orders ORDER BY created_at DESC LIMIT $1")
            .bind(limit + 1)
            .fetch_all(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn delete(db: &PgPool, id: String) -> AppResult<()> {
        let r = sqlx::query("DELETE FROM orders WHERE id=$1")
            .bind(&id)
            .execute(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        if r.rows_affected() == 0 {
            return Err(AppError::NotFound("Order not found".into()));
        }
        Ok(())
    }
}
