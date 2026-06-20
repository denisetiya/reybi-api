use axum::{extract::{Path, State}, Json};
use std::time::Duration;
use crate::config::AppState;
use crate::common::response::ok;
use crate::errors::AppResult;
use crate::models::User;
use crate::utils::cache::keys;
use super::dto::UpdateProfileRequest;
use super::service::ProfileService;

pub async fn get(State(state): State<AppState>, Path(email): Path<String>) -> AppResult<Json<serde_json::Value>> {
    let cache_key = keys::profile(&email);
    let profile: User = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(600), || async {
            ProfileService::get_by_email(&state.db, &email).await
        })
        .await?;
    Ok(Json(ok(profile, "en")))
}

pub async fn update(State(state): State<AppState>, Path(email): Path<String>, Json(body): Json<UpdateProfileRequest>) -> AppResult<Json<serde_json::Value>> {
    let profile = ProfileService::update(&state.db, &email, body).await?;
    state.cache.invalidate(&keys::profile(&email)).await;
    state.cache.invalidate_pattern(keys::profile_pattern()).await;
    Ok(Json(ok(profile, "en")))
}
