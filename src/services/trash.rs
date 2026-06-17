use sqlx::PgPool;
use uuid::Uuid;
use crate::dto::{CreateTrashTypeRequest, PaginationQuery};
use crate::errors::{AppError, AppResult};
use crate::models::TrashType;

pub struct TrashService;

impl TrashService {
    pub async fn list(db: &PgPool, pq: &PaginationQuery) -> AppResult<Vec<TrashType>> {
        let limit = pq.take();
        let rows = sqlx::query_as::<_, TrashType>(
            "SELECT * FROM trash_types ORDER BY name ASC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
        Ok(rows)
    }

    pub async fn create(db: &PgPool, data: CreateTrashTypeRequest) -> AppResult<TrashType> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, TrashType>(
            r#"INSERT INTO trash_types (id, name, image) VALUES ($1, $2, $3) RETURNING *"#
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.image.unwrap_or_default())
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn update(db: &PgPool, id: Uuid, data: CreateTrashTypeRequest) -> AppResult<TrashType> {
        sqlx::query_as::<_, TrashType>(
            r#"UPDATE trash_types SET name = $2, image = $3, updated_at = NOW() WHERE id = $1 RETURNING *"#
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.image.unwrap_or_default())
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Trash type not found".into()))
    }

    pub async fn delete(db: &PgPool, id: Uuid) -> AppResult<()> {
        let r = sqlx::query("DELETE FROM trash_types WHERE id = $1")
            .bind(id)
            .execute(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        if r.rows_affected() == 0 {
            return Err(AppError::NotFound("Trash type not found".into()));
        }
        Ok(())
    }
}
