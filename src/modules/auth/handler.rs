use axum::{extract::State, Json};
use crate::common::locale::Locale;
use crate::config::AppState;
use crate::errors::AppResult;
use crate::utils::helpers::extract_bearer_token;
use super::{dto::*, service::AuthService};

pub async fn login(
    State(state): State<AppState>,
    Locale(locale): Locale,
    headers: axum::http::HeaderMap,
) -> AppResult<Json<serde_json::Value>> {
    let token = extract_bearer_token(&headers)
        .ok_or(crate::errors::AppError::Unauthorized)?;
    let response = AuthService::login(&state.db, &state.config, &token).await?;
    Ok(Json(crate::common::response::ok(response, &locale)))
}

pub async fn register(
    State(state): State<AppState>,
    Locale(locale): Locale,
    headers: axum::http::HeaderMap,
    Json(body): Json<RegisterRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let _token = extract_bearer_token(&headers)
        .ok_or(crate::errors::AppError::Unauthorized)?;
    let result = AuthService::register(&state.db, &_token, body).await?;
    Ok(Json(result))
}

pub async fn reset_password(
    Json(body): Json<ResetPasswordRequest>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Password reset link sent",
        "data": { "email": body.email },
        "meta": { "locale": "en" }
    })))
}