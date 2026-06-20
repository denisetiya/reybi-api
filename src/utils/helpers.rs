pub fn parse_id(s: &str) -> Result<String, crate::errors::AppError> {
    Ok(s.to_string())
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
