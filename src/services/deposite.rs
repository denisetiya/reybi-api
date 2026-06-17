use sqlx::PgPool;
use uuid::Uuid;
use crate::dto::{CreateDepositeRequest, PaginationQuery};
use crate::errors::{AppError, AppResult};
use crate::models::Deposite;

pub struct DepositeService;

impl DepositeService {
    pub async fn create(
        db: &PgPool,
        user_id: Uuid,
        data: CreateDepositeRequest,
    ) -> AppResult<Deposite> {
        let id = Uuid::new_v4();
        let deposite = sqlx::query_as::<_, Deposite>(
            r#"INSERT INTO deposites (id, user_id, address_id, type, pickup_date, pickup_time, coin, images, landfill_id)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"#
        )
        .bind(id)
        .bind(user_id)
        .bind(data.address_id)
        .bind(&data.r#type)
        .bind(&data.pickup_date)
        .bind(&data.pickup_time)
        .bind(data.coins.map(|c| c as i32))
        .bind(data.images.unwrap_or(serde_json::json!({})))
        .bind(data.landfill_id)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        // Create garbage details
        for g in &data.garbage_type {
            let gid = Uuid::new_v4();
            sqlx::query(
                r#"INSERT INTO garbage_details (id, trash_type_id, deposite_id, amount) VALUES ($1, $2, $3, $4)"#
            )
            .bind(gid)
            .bind(g.trash_type_id)
            .bind(id)
            .bind(g.amount as i32)
            .execute(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        // Create deposite status
        let sid = Uuid::new_v4();
        sqlx::query(
            r#"INSERT INTO deposite_statuses (id, deposit_id, ongoing) VALUES ($1, $2, true)"#
        )
        .bind(sid)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(deposite)
    }

    pub async fn get_by_user(
        db: &PgPool,
        user_id: Option<Uuid>,
        pq: &PaginationQuery,
    ) -> AppResult<Vec<Deposite>> {
        let limit = pq.take();
        if let Some(uid) = user_id {
            let rows = sqlx::query_as::<_, Deposite>(
                "SELECT * FROM deposites WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2"
            )
            .bind(uid)
            .bind(limit)
            .fetch_all(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
            Ok(rows)
        } else {
            let rows = sqlx::query_as::<_, Deposite>(
                "SELECT * FROM deposites ORDER BY created_at DESC LIMIT $1"
            )
            .bind(limit)
            .fetch_all(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
            Ok(rows)
        }
    }
}
