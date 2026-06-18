pub fn parse_id(s: &str) -> Result<uuid::Uuid, crate::errors::AppError> {
    uuid::Uuid::parse_str(s).map_err(|_| {
        crate::errors::AppError::Validation(vec![crate::errors::FieldError {
            path: "id".into(),
            message: "Invalid ID format".into(),
        }])
    })
}

pub fn extract_bearer_token(
    headers: &axum::http::HeaderMap,
) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}
