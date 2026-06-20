use super::dto::CreateLandfillRequest;
use crate::errors::{AppError, AppResult};
use crate::models::Landfill;
use sqlx::PgPool;

pub struct LandfillService;

impl LandfillService {
    pub async fn list(db: &PgPool, cursor: Option<&str>, limit: i64) -> AppResult<Vec<Landfill>> {
        let lim = limit + 1;
        if let Some(c) = cursor {
            sqlx::query_as::<_, Landfill>(
                "SELECT * FROM landfills WHERE id < $1 ORDER BY name ASC LIMIT $2",
            )
            .bind(c)
            .bind(lim)
            .fetch_all(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))
        } else {
            sqlx::query_as::<_, Landfill>("SELECT * FROM landfills ORDER BY name ASC LIMIT $1")
                .bind(lim)
                .fetch_all(db)
                .await
                .map_err(|e| AppError::Internal(e.into()))
        }
    }

    pub async fn create(db: &PgPool, data: CreateLandfillRequest) -> AppResult<Landfill> {
        let id = cuid2::create_id();
        sqlx::query_as::<_, Landfill>(
            r#"INSERT INTO landfills (id, name, address) VALUES ($1,$2,$3) RETURNING *"#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.address)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn update(
        db: &PgPool,
        id: String,
        data: CreateLandfillRequest,
    ) -> AppResult<Landfill> {
        sqlx::query_as::<_, Landfill>(
            r#"UPDATE landfills SET name=$2, address=$3, updated_at=NOW() WHERE id=$1 RETURNING *"#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.address)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Landfill not found".into()))
    }

    pub async fn delete(db: &PgPool, id: String) -> AppResult<()> {
        let r = sqlx::query("DELETE FROM landfills WHERE id=$1")
            .bind(&id)
            .execute(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        if r.rows_affected() == 0 {
            return Err(AppError::NotFound("Landfill not found".into()));
        }
        Ok(())
    }
}
