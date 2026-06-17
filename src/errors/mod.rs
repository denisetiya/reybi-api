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
    pub field: String,
    pub rule: String,
    pub message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: ErrorBody,
    meta: ResponseMeta,
}

#[derive(Serialize)]
struct ErrorBody {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    details: Vec<FieldError>,
}

#[derive(Serialize)]
struct ResponseMeta {
    locale: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message, details) = match self {
            AppError::Validation(d) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                "Validation failed".to_string(),
                d,
            ),
            AppError::NotFound(m) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                m,
                vec![],
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                "Authentication required".to_string(),
                vec![],
            ),
            AppError::Forbidden(m) => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                m,
                vec![],
            ),
            AppError::Conflict(m) => (
                StatusCode::CONFLICT,
                "CONFLICT",
                m,
                vec![],
            ),
            AppError::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS,
                "RATE_LIMITED",
                "Too many requests".to_string(),
                vec![],
            ),
            AppError::Internal(e) => {
                tracing::error!(error = %e, "internal_error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "Internal server error".to_string(),
                    vec![],
                )
            }
        };

        let body = ErrorResponse {
            success: false,
            error: ErrorBody {
                code: code.to_string(),
                message,
                details,
            },
            meta: ResponseMeta {
                locale: "en".to_string(),
            },
        };

        (status, axum::Json(body)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
