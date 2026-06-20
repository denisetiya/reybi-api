use crate::errors::{AppError, AppResult};
use crate::models::Banner;
use sqlx::PgPool;

pub struct BannerService;

impl BannerService {
    pub async fn list(
        db: &PgPool,
        r#type: Option<&str>,
        cursor: Option<&str>,
        limit: i64,
    ) -> AppResult<Vec<Banner>> {
        // CUIDs are time-sortable lexicographically; `id < $cursor` gives the prev page
        // when results are ordered by `id DESC` (newest first).
        let limit_with_one = limit + 1;
        let rows = match (r#type, cursor) {
            (Some(t), Some(c)) => {
                sqlx::query_as::<_, Banner>(
                    "SELECT * FROM banners WHERE type = $1 AND id < $2 \
                     ORDER BY id DESC LIMIT $3",
                )
                .bind(t)
                .bind(c)
                .bind(limit_with_one)
                .fetch_all(db)
                .await
            }
            (Some(t), None) => {
                sqlx::query_as::<_, Banner>(
                    "SELECT * FROM banners WHERE type = $1 \
                     ORDER BY id DESC LIMIT $2",
                )
                .bind(t)
                .bind(limit_with_one)
                .fetch_all(db)
                .await
            }
            (None, Some(c)) => {
                sqlx::query_as::<_, Banner>(
                    "SELECT * FROM banners WHERE id < $1 \
                     ORDER BY id DESC LIMIT $2",
                )
                .bind(c)
                .bind(limit_with_one)
                .fetch_all(db)
                .await
            }
            (None, None) => {
                sqlx::query_as::<_, Banner>("SELECT * FROM banners ORDER BY id DESC LIMIT $1")
                    .bind(limit_with_one)
                    .fetch_all(db)
                    .await
            }
        };
        rows.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn create(db: &PgPool, image: &str, r#type: Option<&str>) -> AppResult<Banner> {
        let id = cuid2::create_id();
        sqlx::query_as::<_, Banner>(
            r#"INSERT INTO banners (id, image, type) VALUES ($1,$2,$3) RETURNING *"#,
        )
        .bind(id)
        .bind(image)
        .bind(r#type)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn update(
        db: &PgPool,
        id: &str,
        image: Option<&str>,
        r#type: Option<&str>,
    ) -> AppResult<Banner> {
        sqlx::query_as::<_, Banner>(
            r#"UPDATE banners
               SET image = COALESCE($2, image),
                   type  = COALESCE($3, type)
               WHERE id = $1
               RETURNING *"#,
        )
        .bind(id)
        .bind(image)
        .bind(r#type)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or(AppError::NotFound(format!("banner {id} not found")))
    }

    pub async fn delete(db: &PgPool, id: &str) -> AppResult<u64> {
        let res = sqlx::query("DELETE FROM banners WHERE id = $1")
            .bind(id)
            .execute(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        Ok(res.rows_affected())
    }
}
