use super::dto::UpdateProfileRequest;
use super::service::ProfileService;
use crate::common::locale::Locale;
use crate::common::response::ok;
use crate::common::response::AppResponse;
use crate::config::AppState;
use crate::errors::AppResult;
use crate::models::User;
use crate::utils::cache::keys;
use axum::{
    extract::{Path, State},
    Json,
};
use std::time::Duration;

pub async fn get(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(email): Path<String>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let cache_key = keys::profile(&email);
    let profile: User = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(600), || async {
            ProfileService::get_by_email(&state.db, &email).await
        })
        .await?;
    Ok(ok(profile, &locale))
}

pub async fn update(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(email): Path<String>,
    Json(body): Json<UpdateProfileRequest>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let profile = ProfileService::update(&state.db, &email, body).await?;
    state.cache.invalidate(&keys::profile(&email)).await;
    state
        .cache
        .invalidate_pattern(keys::profile_pattern())
        .await;
    Ok(ok(profile, &locale))
}
