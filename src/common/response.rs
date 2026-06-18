use serde::Serialize;

/// Paginated list response — matches original TypeScript contract + locale feature.
pub fn ok_paginated<T: Serialize>(
    content: Vec<T>,
    next_cursor: Option<String>,
    has_more: bool,
    locale: &str,
) -> serde_json::Value {
    serde_json::json!({
        "statusCode": 200,
        "message": "Success",
        "content": content,
        "meta": {
            "nextCursor": next_cursor,
            "hasMore": has_more,
            "locale": locale,
        }
    })
}

/// Single item response.
pub fn ok<T: Serialize>(content: T, locale: &str) -> serde_json::Value {
    serde_json::json!({
        "statusCode": 200,
        "message": "Success",
        "content": content,
        "meta": {
            "locale": locale,
        }
    })
}

/// Created response.
pub fn created<T: Serialize>(content: T, locale: &str) -> serde_json::Value {
    serde_json::json!({
        "statusCode": 201,
        "message": "Success",
        "content": content,
        "meta": {
            "locale": locale,
        }
    })
}

/// Action response with message (delete, etc.)
pub fn message(msg: &str) -> serde_json::Value {
    serde_json::json!({
        "statusCode": 200,
        "message": msg,
    })
}
