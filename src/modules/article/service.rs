use sqlx::PgPool;
use crate::errors::{AppError, AppResult};
use crate::models::Article;
use super::dto::CreateArticleRequest;

pub struct ArticleService;

impl ArticleService {
    pub async fn list(db: &PgPool, limit: i64) -> AppResult<Vec<Article>> {
        sqlx::query_as::<_, Article>(
            "SELECT * FROM articles ORDER BY created_at DESC LIMIT $1"
        ).bind(limit + 1)
        .fetch_all(db).await.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn get_by_id(db: &PgPool, id: String) -> AppResult<Article> {
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = $1")
            .bind(id).fetch_optional(db).await
            .map_err(|e| AppError::Internal(e.into()))?
            .ok_or_else(|| AppError::NotFound("Article not found".into()))
    }

    pub async fn create(db: &PgPool, data: CreateArticleRequest) -> AppResult<Article> {
        let id = cuid2::create_id();
        sqlx::query_as::<_, Article>(
            r#"INSERT INTO articles (id, thumbnail, header, content) VALUES ($1,$2,$3,$4) RETURNING *"#
        ).bind(id).bind(&data.thumbnail).bind(&data.header).bind(&data.content)
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn update(db: &PgPool, id: String, data: CreateArticleRequest) -> AppResult<Article> {
        sqlx::query_as::<_, Article>(
            r#"UPDATE articles SET thumbnail=$2, header=$3, content=$4, updated_at=NOW() WHERE id=$1 RETURNING *"#
        ).bind(id).bind(&data.thumbnail).bind(&data.header).bind(&data.content)
        .fetch_optional(db).await.map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Article not found".into()))
    }

    pub async fn delete(db: &PgPool, id: String) -> AppResult<()> {
        let r = sqlx::query("DELETE FROM articles WHERE id=$1").bind(&id)
            .execute(db).await.map_err(|e| AppError::Internal(e.into()))?;
        if r.rows_affected() == 0 { return Err(AppError::NotFound("Article not found".into())); }
        Ok(())
    }
}
