pub mod helpers;

pub fn ok<T: serde::Serialize>(data: T, locale: &str) -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": locale }
    })
}

pub fn ok_paginated<T: serde::Serialize>(
    data: Vec<T>,
    cursor: Option<String>,
    has_more: bool,
    locale: &str,
) -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "data": data,
        "meta": {
            "locale": locale,
            "pagination": { "cursor": cursor, "has_more": has_more, "count": data.len() }
        }
    })
}
