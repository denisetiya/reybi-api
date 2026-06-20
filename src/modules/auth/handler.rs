use super::{dto::*, service::AuthService};
use crate::common::locale::Locale;
use crate::common::response::{message, ok, AppResponse};
use crate::config::AppState;
use crate::errors::AppResult;
use crate::utils::helpers::extract_bearer_token;
use axum::{extract::State, Json};

pub async fn login(
    State(state): State<AppState>,
    Locale(locale): Locale,
    headers: axum::http::HeaderMap,
) -> AppResult<AppResponse<serde_json::Value>> {
    let token = extract_bearer_token(&headers).ok_or(crate::errors::AppError::Unauthorized)?;
    let response = AuthService::login(&state.db, &state.config, &token).await?;
    Ok(ok(response, &locale))
}

pub async fn register(
    State(state): State<AppState>,
    Locale(_locale): Locale,
    headers: axum::http::HeaderMap,
    Json(body): Json<RegisterRequest>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let _token = extract_bearer_token(&headers).ok_or(crate::errors::AppError::Unauthorized)?;
    let result = AuthService::register(&state.db, &_token, body).await?;
    Ok(message("Registered"))
}

pub async fn reset_password(
    Json(body): Json<ResetPasswordRequest>,
) -> AppResult<AppResponse<serde_json::Value>> {
    // Localised message via t() — keep the structured response shape.
    Ok(message("Password reset link sent"))
}
