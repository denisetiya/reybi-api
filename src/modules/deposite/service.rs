use sqlx::PgPool;
use crate::errors::{AppError, AppResult};
use crate::models::Deposite;
use super::dto::CreateDepositeRequest;

pub struct DepositeService;

impl DepositeService {
    pub async fn create(db: &PgPool, user_id: String, data: CreateDepositeRequest) -> AppResult<Deposite> {
        let id = cuid2::create_id();
        let deposite = sqlx::query_as::<_, Deposite>(
            r#"INSERT INTO deposites (id, user_id, address_id, type, pickup_date, pickup_time, coin, images, landfill_id)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING *"#
        ).bind(&id).bind(&user_id).bind(&data.address_id).bind(&data.r#type)
        .bind(&data.pickup_date).bind(&data.pickup_time).bind(data.coin)
        .bind(data.images.unwrap_or(serde_json::json!({})))
        .bind(data.landfill_id)
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))?;

        for g in &data.garbage_type {
            let gid = cuid2::create_id();
            sqlx::query(
                r#"INSERT INTO garbage_details (id, trash_type_id, deposite_id, amount) VALUES ($1,$2,$3,$4)"#
            ).bind(gid).bind(&g.trash_type_id).bind(&id).bind(g.amount)
            .execute(db).await.map_err(|e| AppError::Internal(e.into()))?;
        }
        let sid = cuid2::create_id();
        sqlx::query(r#"INSERT INTO deposite_statuses (id, deposit_id, ongoing) VALUES ($1,$2,true)"#)
            .bind(sid).bind(id).execute(db).await.map_err(|e| AppError::Internal(e.into()))?;
        Ok(deposite)
    }

    pub async fn get_by_user(db: &PgPool, user_id: Option<String>, limit: i64) -> AppResult<Vec<Deposite>> {
        if let Some(uid) = user_id {
            sqlx::query_as::<_, Deposite>(
                "SELECT * FROM deposites WHERE user_id=$1 ORDER BY created_at DESC LIMIT $2"
            ).bind(uid).bind(limit + 1).fetch_all(db).await.map_err(|e| AppError::Internal(e.into()))
        } else {
            sqlx::query_as::<_, Deposite>(
                "SELECT * FROM deposites ORDER BY created_at DESC LIMIT $1"
            ).bind(limit + 1).fetch_all(db).await.map_err(|e| AppError::Internal(e.into()))
        }
    }
}
