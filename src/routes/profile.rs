use axum::{
    extract::{Path, State},
    Json,
};
use crate::config::AppState;
use crate::dto::UpdateProfileRequest;
use crate::errors::AppResult;
use crate::services::profile::ProfileService;

pub async fn get_profile(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let profile = ProfileService::get_by_email(&state.db, &email).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": profile,
        "meta": { "locale": "en" }
    })))
}

pub async fn update_profile(
    State(state): State<AppState>,
    Path(email): Path<String>,
    Json(body): Json<UpdateProfileRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let profile = ProfileService::update(&state.db, &email, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": profile,
        "meta": { "locale": "en" }
    })))
}
