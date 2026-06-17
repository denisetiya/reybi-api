use sqlx::PgPool;
use uuid::Uuid;
use crate::dto::{CreateOrderRequest, PaginationQuery};
use crate::errors::{AppError, AppResult};
use crate::models::Order;

pub struct OrderService;

impl OrderService {
    pub async fn create(
        db: &PgPool,
        user_id: Uuid,
        data: CreateOrderRequest,
    ) -> AppResult<Order> {
        let id = Uuid::new_v4();
        let order = sqlx::query_as::<_, Order>(
            r#"INSERT INTO orders (id, user_id, product_id, quantity, coin)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#
        )
        .bind(id)
        .bind(user_id)
        .bind(data.product_id)
        .bind(data.quantity as i32)
        .bind(data.coin.map(|c| c as i32))
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        // Create payment record
        let pay_id = Uuid::new_v4();
        sqlx::query(
            r#"INSERT INTO payment_histories (id, order_id, method, type, amount)
               VALUES ($1, $2, $3, $4, $5)"#
        )
        .bind(pay_id)
        .bind(id)
        .bind(&data.payment.method)
        .bind(&data.payment.r#type)
        .bind(data.payment.amount)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(order)
    }

    pub async fn get_by_user(
        db: &PgPool,
        user_id: Uuid,
        pq: &PaginationQuery,
    ) -> AppResult<Vec<Order>> {
        let limit = pq.take();
        let rows = sqlx::query_as::<_, Order>(
            "SELECT * FROM orders WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2"
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
        Ok(rows)
    }

    pub async fn get_all(db: &PgPool, pq: &PaginationQuery) -> AppResult<Vec<Order>> {
        let limit = pq.take();
        let rows = sqlx::query_as::<_, Order>(
            "SELECT * FROM orders ORDER BY created_at DESC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
        Ok(rows)
    }

    pub async fn delete(db: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE orders SET ... WHERE id = $1")
            .bind(id)
            .execute(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        Ok(())
    }
}
