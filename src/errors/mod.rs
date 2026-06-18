use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("validation_error")]
    Validation(Vec<FieldError>),

    #[error("not_found: {0}")]
    NotFound(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("rate_limited")]
    RateLimited,

    #[error("internal: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub path: String,
    pub message: String,
}

/// Matches original TypeScript contract:
/// - Single error:  {statusCode, message, error}
/// - Validation:    {statusCode, message, error: null, content: [...]}
#[derive(Serialize)]
struct ErrorResponse {
    #[serde(rename = "statusCode")]
    status_code: u16,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Vec<FieldError>>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(d) => {
                let body = ErrorResponse {
                    status_code: 400,
                    message: "Bad Request".to_string(),
                    error: None,
                    content: Some(d),
                };
                (StatusCode::BAD_REQUEST, axum::Json(body)).into_response()
            }
            AppError::NotFound(m) => {
                let body = ErrorResponse {
                    status_code: 404,
                    message: "Not Found".to_string(),
                    error: Some(m),
                    content: None,
                };
                (StatusCode::NOT_FOUND, axum::Json(body)).into_response()
            }
            AppError::Unauthorized => {
                let body = ErrorResponse {
                    status_code: 401,
                    message: "Unauthorized".to_string(),
                    error: Some("Token not found".to_string()),
                    content: None,
                };
                (StatusCode::UNAUTHORIZED, axum::Json(body)).into_response()
            }
            AppError::Forbidden(m) => {
                let body = ErrorResponse {
                    status_code: 403,
                    message: "Forbidden".to_string(),
                    error: Some(m),
                    content: None,
                };
                (StatusCode::FORBIDDEN, axum::Json(body)).into_response()
            }
            AppError::Conflict(m) => {
                let body = ErrorResponse {
                    status_code: 409,
                    message: "Conflict".to_string(),
                    error: Some(m),
                    content: None,
                };
                (StatusCode::CONFLICT, axum::Json(body)).into_response()
            }
            AppError::RateLimited => {
                let body = ErrorResponse {
                    status_code: 429,
                    message: "Too Many Requests".to_string(),
                    error: Some("Rate limit exceeded".to_string()),
                    content: None,
                };
                (StatusCode::TOO_MANY_REQUESTS, axum::Json(body)).into_response()
            }
            AppError::Internal(e) => {
                tracing::error!(error = %e, "internal_error");
                let body = ErrorResponse {
                    status_code: 500,
                    message: "Internal Server Error".to_string(),
                    error: Some(e.to_string()),
                    content: None,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(body)).into_response()
            }
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
