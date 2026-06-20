use axum::{extract::{Path, State}, Json};
use crate::config::AppState;
use crate::common::response::ok;
use crate::errors::AppResult;
use crate::utils::cache::keys;
use super::{dto::*, service::ReviewService};

pub async fn create(State(state): State<AppState>, Json(body): Json<CreateReviewRequest>) -> AppResult<Json<serde_json::Value>> {
    let review = ReviewService::create(&state.db, cuid2::create_id(), body).await?;
    // New review affects the parent product's review set — drop product cache too.
    state.cache.invalidate_pattern(keys::reviews_pattern()).await;
    state.cache.invalidate_pattern(crate::utils::cache::keys::products_pattern()).await;
    Ok(Json(ok(review, "en")))
}

pub async fn update(State(state): State<AppState>, Path(id): Path<String>, Json(body): Json<UpdateReviewRequest>) -> AppResult<Json<serde_json::Value>> {
    let review = ReviewService::update(&state.db, id.clone(), body).await?;
    state.cache.invalidate(&keys::review(&id)).await;
    state.cache.invalidate_pattern(keys::reviews_pattern()).await;
    state.cache.invalidate_pattern(crate::utils::cache::keys::products_pattern()).await;
    Ok(Json(ok(review, "en")))
}
