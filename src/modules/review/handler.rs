use super::{dto::*, service::ReviewService};
use crate::common::locale::Locale;
use crate::common::response::ok;
use crate::config::AppState;
use crate::errors::AppResult;
use crate::utils::cache::keys;
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn create(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Json(body): Json<CreateReviewRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let review = ReviewService::create(&state.db, cuid2::create_id(), body).await?;
    // New review affects the parent product's review set — drop product cache too.
    state
        .cache
        .invalidate_pattern(keys::reviews_pattern())
        .await;
    state
        .cache
        .invalidate_pattern(crate::utils::cache::keys::products_pattern())
        .await;
    Ok(Json(ok(review, &locale)))
}

pub async fn update(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(id): Path<String>,
    Json(body): Json<UpdateReviewRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let review = ReviewService::update(&state.db, id.clone(), body).await?;
    state.cache.invalidate(&keys::review(&id)).await;
    state
        .cache
        .invalidate_pattern(keys::reviews_pattern())
        .await;
    state
        .cache
        .invalidate_pattern(crate::utils::cache::keys::products_pattern())
        .await;
    Ok(Json(ok(review, &locale)))
}
