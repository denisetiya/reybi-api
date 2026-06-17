use axum::{extract::State, Json};
use crate::config::AppState;
use crate::dto::{RegisterRequest, ResetPasswordRequest};
use crate::errors::AppResult;
use crate::services::auth::AuthService;
use crate::utils::helpers::extract_bearer_token;

pub async fn login(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> AppResult<Json<serde_json::Value>> {
    let token = extract_bearer_token(&headers).ok_or_else(|| crate::errors::AppError::Unauthorized)?;
    let response = AuthService::login(&state.db, &state.config, &token).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": response,
        "meta": { "locale": "en" }
    })))
}

pub async fn register(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(body): Json<RegisterRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let token = extract_bearer_token(&headers).ok_or_else(|| crate::errors::AppError::Unauthorized)?;
    let response = AuthService::register(&state.db, &token, body).await?;
    Ok(Json(response))
}

pub async fn reset_password(
    _state: State<AppState>,
    Json(body): Json<ResetPasswordRequest>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Password reset link sent",
        "data": { "email": body.email }
    })))
}
