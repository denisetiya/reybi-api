use sqlx::PgPool;
use uuid::Uuid;
use crate::dto::{CreateLandfillRequest, PaginationQuery};
use crate::errors::{AppError, AppResult};
use crate::models::Landfill;

pub struct LandfillService;

impl LandfillService {
    pub async fn list(db: &PgPool, pq: &PaginationQuery) -> AppResult<Vec<Landfill>> {
        let limit = pq.take();
        let rows = sqlx::query_as::<_, Landfill>(
            "SELECT * FROM landfills ORDER BY name ASC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
        Ok(rows)
    }

    pub async fn create(db: &PgPool, data: CreateLandfillRequest) -> AppResult<Landfill> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Landfill>(
            r#"INSERT INTO landfills (id, name, address) VALUES ($1, $2, $3) RETURNING *"#
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.address)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn update(db: &PgPool, id: Uuid, data: CreateLandfillRequest) -> AppResult<Landfill> {
        sqlx::query_as::<_, Landfill>(
            r#"UPDATE landfills SET name = $2, address = $3 WHERE id = $1 RETURNING *"#
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.address)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Landfill not found".into()))
    }

    pub async fn delete(db: &PgPool, id: Uuid) -> AppResult<()> {
        let r = sqlx::query("DELETE FROM landfills WHERE id = $1")
            .bind(id)
            .execute(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        if r.rows_affected() == 0 {
            return Err(AppError::NotFound("Landfill not found".into()));
        }
        Ok(())
    }
}
