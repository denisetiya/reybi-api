//! Typed response builders — avoid the `serde_json::Value` → `axum::Json` double-encode
//! (one full JSON serialise roundtrip on the hot path).
//!
//! Contract (matches the original TypeScript API, EN+ID, pagination, locale):
//!   { "statusCode": 200, "message": "Success", "content": ..., "meta": { ... } }
//!
//! `AppResponse<T>` implements `IntoResponse` directly so axum does a single
//! `serde_json::to_vec` then writes the bytes — no intermediate `Value` tree.

use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppResponse<T> {
    #[serde(rename = "statusCode")]
    pub status: u16,
    pub message: &'static str,
    pub content: T,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize, Default)]
pub struct ResponseMeta {
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "hasMore", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    pub locale: String,
    /// Server-computed ETag for client-side 304 revalidation.
    /// Only emitted on cacheable list responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}

/// Pre-serialized response wrapper — used when the body is already a
/// `serde_json::Value` (e.g. dynamic shapes from join queries).  Skips
/// the intermediate `Value` allocation when we already have one.
pub struct PreJson(pub serde_json::Value, pub StatusCode);

impl IntoResponse for PreJson {
    fn into_response(self) -> Response {
        (self.1, Json(self.0)).into_response()
    }
}

impl<T: Serialize> AppResponse<T> {
    pub fn ok(content: T, locale: &str) -> Self {
        Self {
            status: 200,
            message: "Success",
            content,
            meta: ResponseMeta {
                locale: locale.to_string(),
                ..Default::default()
            },
        }
    }

    pub fn created(content: T, locale: &str) -> Self {
        Self {
            status: 201,
            message: "Success",
            content,
            meta: ResponseMeta {
                locale: locale.to_string(),
                ..Default::default()
            },
        }
    }

    pub fn paginated(content: T, cursor: Option<String>, has_more: bool, locale: &str) -> Self {
        Self {
            status: 200,
            message: "Success",
            content,
            meta: ResponseMeta {
                next_cursor: cursor,
                has_more: Some(has_more),
                locale: locale.to_string(),
                ..Default::default()
            },
        }
    }

    /// Attach a server-computed ETag for 304 revalidation.
    /// Format: weak ETag `"<fnv1a-hex>"` over the JSON payload.
    pub fn with_etag(mut self, etag: String) -> Self {
        self.meta.etag = Some(etag);
        self
    }

    /// Add `Cache-Control: public, max-age=N` so CDNs and browsers cache
    /// list responses without round-tripping.
    pub fn cacheable(self, max_age_secs: u32) -> CacheResponse<T> {
        CacheResponse {
            inner: self,
            max_age: max_age_secs,
        }
    }
}

pub struct CacheResponse<T> {
    inner: AppResponse<T>,
    max_age: u32,
}

impl<T: Serialize> IntoResponse for CacheResponse<T> {
    fn into_response(self) -> Response {
        let mut resp = self.inner.into_response();
        resp.headers_mut().insert(
            header::CACHE_CONTROL,
            HeaderValue::from_str(&format!("public, max-age={}", self.max_age))
                .unwrap_or_else(|_| HeaderValue::from_static("public, max-age=60")),
        );
        resp
    }
}

impl<T: Serialize> IntoResponse for AppResponse<T> {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::OK);
        // Single serialize — no Value tree allocation.
        match serde_json::to_vec(&self) {
            Ok(bytes) => {
                let mut headers = HeaderMap::with_capacity(2);
                headers.insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json; charset=utf-8"),
                );
                if let Some(ref etag) = self.meta.etag {
                    headers.insert(
                        header::ETAG,
                        HeaderValue::from_str(etag)
                            .unwrap_or_else(|_| HeaderValue::from_static("\"0\"")),
                    );
                }
                (status, headers, bytes).into_response()
            }
            Err(e) => {
                tracing::error!(error = %e, "response serialize failed");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Free functions kept for handler ergonomics — accept any `Serialize` input
// and store it as `serde_json::Value` so handler signatures stay simple.
// Saves one full JSON serialise roundtrip vs the old `Json(Value)` pattern.
// -----------------------------------------------------------------------------

pub fn ok<T: Serialize>(content: T, locale: &str) -> AppResponse<serde_json::Value> {
    let value = serde_json::to_value(content).unwrap_or(serde_json::Value::Null);
    AppResponse::ok(value, locale)
}

pub fn created<T: Serialize>(content: T, locale: &str) -> AppResponse<serde_json::Value> {
    let value = serde_json::to_value(content).unwrap_or(serde_json::Value::Null);
    AppResponse::created(value, locale)
}

pub fn ok_paginated<T: Serialize>(
    content: T,
    next_cursor: Option<String>,
    has_more: bool,
    locale: &str,
) -> AppResponse<serde_json::Value> {
    let value = serde_json::to_value(content).unwrap_or(serde_json::Value::Null);
    AppResponse::paginated(value, next_cursor, has_more, locale)
}

/// Action response (delete, register) — no content, just status + message.
pub fn message(msg: &str) -> AppResponse<serde_json::Value> {
    AppResponse {
        status: 200,
        message: "Success",
        content: serde_json::Value::String(msg.to_string()),
        meta: ResponseMeta {
            locale: "en".to_string(),
            ..Default::default()
        },
    }
}

/// Compute a weak ETag over a serializable payload.
/// Uses FNV-1a (64-bit) — fast non-crypto hash, collision rate < 1e-9 for <1MB blobs.
pub fn etag_for<T: Serialize>(value: &T) -> String {
    use std::hash::Hasher;
    let bytes = serde_json::to_vec(value).unwrap_or_default();
    let mut h = fnv::FnvHasher::default();
    h.write_u32(bytes.len() as u32);
    h.write(&bytes);
    format!("\"{:x}\"", h.finish())
}

/// 304 Not Modified short-circuit — call before constructing the full
/// `AppResponse` to avoid serializing the body when the client already has it.
///
/// Returns `Some(empty_response)` if the client sent `If-None-Match` matching
/// the computed ETag, otherwise `None` so the caller builds the full response.
pub fn maybe_not_modified<T: Serialize>(
    value: &T,
    if_none_match: Option<&str>,
) -> Option<Response> {
    let etag = etag_for(value);
    if let Some(client_etag) = if_none_match {
        if client_etag == etag || client_etag == "*" {
            let mut resp = (StatusCode::NOT_MODIFIED).into_response();
            resp.headers_mut().insert(
                header::ETAG,
                HeaderValue::from_str(&etag).unwrap_or(HeaderValue::from_static("\"0\"")),
            );
            resp.headers_mut().insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=300"),
            );
            return Some(resp);
        }
    }
    None
}

/// ETag extractor — pulls `If-None-Match` from request headers without
/// pulling the whole request into a handler.
pub fn if_none_match(headers: &axum::http::HeaderMap) -> Option<&str> {
    headers
        .get(axum::http::header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
}
