use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use crate::config::AppState;
use crate::dto::{CreateReviewRequest, UpdateReviewRequest};
use crate::errors::AppResult;
use crate::services::review::ReviewService;

pub async fn create_review(
    State(state): State<AppState>,
    axum::extract::Extension(claims): axum::extract::Extension<crate::services::auth::Claims>,
    Json(body): Json<CreateReviewRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let user_id = Uuid::parse_str(&claims.id).map_err(|_| crate::errors::AppError::Unauthorized)?;
    let review = ReviewService::create(&state.db, user_id, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": review,
        "meta": { "locale": "en" }
    })))
}

pub async fn update_review(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateReviewRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let review = ReviewService::update(&state.db, id, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": review,
        "meta": { "locale": "en" }
    })))
}
