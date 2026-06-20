use sqlx::PgPool;
use crate::errors::{AppError, AppResult};
use crate::models::TrashType;
use super::dto::CreateTrashTypeRequest;

pub struct TrashService;

impl TrashService {
    pub async fn list(db: &PgPool, cursor: Option<&str>, limit: i64) -> AppResult<Vec<TrashType>> {
        let lim = limit + 1;
        if let Some(c) = cursor {
            sqlx::query_as::<_, TrashType>(
                "SELECT * FROM trash_types WHERE id < $1 ORDER BY name ASC LIMIT $2"
            ).bind(c).bind(lim).fetch_all(db).await.map_err(|e| AppError::Internal(e.into()))
        } else {
            sqlx::query_as::<_, TrashType>(
                "SELECT * FROM trash_types ORDER BY name ASC LIMIT $1"
            ).bind(lim).fetch_all(db).await.map_err(|e| AppError::Internal(e.into()))
        }
    }

    pub async fn create(db: &PgPool, data: CreateTrashTypeRequest) -> AppResult<TrashType> {
        let id = cuid2::create_id();
        sqlx::query_as::<_, TrashType>(
            r#"INSERT INTO trash_types (id, name, image) VALUES ($1,$2,$3) RETURNING *"#
        ).bind(id).bind(&data.name).bind(data.image.unwrap_or_default())
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn update(db: &PgPool, id: String, data: CreateTrashTypeRequest) -> AppResult<TrashType> {
        sqlx::query_as::<_, TrashType>(
            r#"UPDATE trash_types SET name=$2, image=$3, updated_at=NOW() WHERE id=$1 RETURNING *"#
        ).bind(id).bind(&data.name).bind(data.image.unwrap_or_default())
        .fetch_optional(db).await.map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Trash type not found".into()))
    }

    pub async fn delete(db: &PgPool, id: String) -> AppResult<()> {
        let r = sqlx::query("DELETE FROM trash_types WHERE id=$1").bind(&id)
            .execute(db).await.map_err(|e| AppError::Internal(e.into()))?;
        if r.rows_affected() == 0 { return Err(AppError::NotFound("Trash type not found".into())); }
        Ok(())
    }
}
