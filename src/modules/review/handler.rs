use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use crate::config::AppState;
use crate::common::response::ok;
use crate::errors::AppResult;
use super::{dto::*, service::ReviewService};

pub async fn create(State(state): State<AppState>, Json(body): Json<CreateReviewRequest>) -> AppResult<Json<serde_json::Value>> {
    let review = ReviewService::create(&state.db, uuid::Uuid::default(), body).await?;
    Ok(Json(ok(review, "en")))
}

pub async fn update(State(state): State<AppState>, Path(id): Path<Uuid>, Json(body): Json<UpdateReviewRequest>) -> AppResult<Json<serde_json::Value>> {
    let review = ReviewService::update(&state.db, id, body).await?;
    Ok(Json(ok(review, "en")))
}
