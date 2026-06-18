use axum::{extract::{Path, State}, Json};
use crate::config::AppState;
use crate::common::response::ok;
use super::dto::UpdateProfileRequest;
use crate::errors::AppResult;
use super::service::ProfileService;

pub async fn get(State(state): State<AppState>, Path(email): Path<String>) -> AppResult<Json<serde_json::Value>> {
    let profile = ProfileService::get_by_email(&state.db, &email).await?;
    Ok(Json(ok(profile)))
}

pub async fn update(State(state): State<AppState>, Path(email): Path<String>, Json(body): Json<UpdateProfileRequest>) -> AppResult<Json<serde_json::Value>> {
    let profile = ProfileService::update(&state.db, &email, body).await?;
    Ok(Json(ok(profile)))
}
