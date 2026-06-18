use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
    pub meta: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub cursor: Option<String>,
    pub has_more: bool,
    pub count: usize,
}

pub fn ok<T: Serialize>(data: T, locale: &str) -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": locale }
    })
}

pub fn ok_paginated<T: Serialize>(
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
            "pagination": {
                "cursor": cursor,
                "has_more": has_more,
                "count": data.len()
            }
        }
    })
}

pub fn message(msg: &str, locale: &str) -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": msg,
        "meta": { "locale": locale }
    })
}
