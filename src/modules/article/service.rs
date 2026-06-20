use super::dto::CreateArticleRequest;
use crate::errors::{AppError, AppResult};
use crate::models::Article;
use sqlx::PgPool;

pub struct ArticleService;

impl ArticleService {
    pub async fn list(db: &PgPool, cursor: Option<&str>, limit: i64) -> AppResult<Vec<Article>> {
        let limit_with_one = limit + 1;
        if let Some(c) = cursor {
            sqlx::query_as::<_, Article>(
                "SELECT * FROM articles WHERE id < $1 ORDER BY id DESC LIMIT $2",
            )
            .bind(c)
            .bind(limit_with_one)
            .fetch_all(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))
        } else {
            sqlx::query_as::<_, Article>("SELECT * FROM articles ORDER BY id DESC LIMIT $1")
                .bind(limit_with_one)
                .fetch_all(db)
                .await
                .map_err(|e| AppError::Internal(e.into()))
        }
    }

    pub async fn get_by_id(db: &PgPool, id: String) -> AppResult<Article> {
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await
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
        let r = sqlx::query("DELETE FROM articles WHERE id=$1")
            .bind(&id)
            .execute(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        if r.rows_affected() == 0 {
            return Err(AppError::NotFound("Article not found".into()));
        }
        Ok(())
    }
}
