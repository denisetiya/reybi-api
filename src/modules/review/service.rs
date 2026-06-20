use super::dto::{CreateReviewRequest, UpdateReviewRequest};
use crate::errors::{AppError, AppResult};
use crate::models::ReviewProduct;
use sqlx::PgPool;

pub struct ReviewService;

impl ReviewService {
    pub async fn create(
        db: &PgPool,
        user_id: String,
        data: CreateReviewRequest,
    ) -> AppResult<ReviewProduct> {
        let id = cuid2::create_id();
        sqlx::query_as::<_, ReviewProduct>(
            r#"INSERT INTO review_products (id, product_id, user_id, comment, rating, images) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *"#
        ).bind(id).bind(&data.product_id).bind(&user_id).bind(&data.comment).bind(data.rating).bind(data.images)
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn update(
        db: &PgPool,
        id: String,
        data: UpdateReviewRequest,
    ) -> AppResult<ReviewProduct> {
        sqlx::query_as::<_, ReviewProduct>(
            r#"UPDATE review_products SET comment=COALESCE($2,comment), rating=COALESCE($3,rating), updated_at=NOW() WHERE id=$1 RETURNING *"#
        ).bind(id).bind(&data.comment).bind(data.rating)
        .fetch_optional(db).await.map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Review not found".into()))
    }
}
