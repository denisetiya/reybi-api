use axum::body::{to_bytes, Body};
use axum::extract::Request;
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

/// ETag / 304 Not Modified middleware.
///
/// Buffers the response body for safe GET requests, computes a weak FNV-1a
/// ETag over the bytes, and:
///   • if the client's `If-None-Match` matches → returns `304 Not Modified`
///     with an empty body (saves bandwidth, client keeps its cached copy),
///   • otherwise → attaches the computed `ETag` header so the next request
///     can revalidate.
///
/// Only GET/HEAD are processed; mutations pass straight through.  Bodies
/// larger than 2 MiB are streamed untouched (buffering them would defeat the
/// streaming win and risk memory pressure).
const MAX_ETAG_BODY: usize = 2 * 1024 * 1024;

pub async fn etag_middleware(req: Request, next: Next) -> Response {
    let is_safe = matches!(*req.method(), Method::GET | Method::HEAD);
    // Capture the client's If-None-Match before consuming the request.
    let if_none_match = req
        .headers()
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let resp = next.run(req).await;

    if !is_safe || resp.status() != StatusCode::OK {
        return resp;
    }

    let (mut parts, body) = resp.into_parts();
    let bytes = match to_bytes(body, MAX_ETAG_BODY).await {
        Ok(b) => b,
        // Body too large or stream error — rebuild untouched (best-effort).
        Err(_) => {
            return Response::from_parts(parts, Body::empty());
        }
    };

    // Compute weak ETag — FNV-1a over the body bytes, length-prefixed.
    let etag = {
        use std::hash::Hasher;
        let mut h = fnv::FnvHasher::default();
        h.write_u32(bytes.len() as u32);
        h.write(&bytes);
        format!("\"{:x}\"", h.finish())
    };

    // Conditional request — client already has this exact body.
    if let Some(client) = if_none_match.as_deref() {
        if client == etag || client == "*" {
            let mut not_modified = Response::new(Body::empty());
            *not_modified.status_mut() = StatusCode::NOT_MODIFIED;
            if let Ok(v) = HeaderValue::from_str(&etag) {
                not_modified.headers_mut().insert(header::ETAG, v);
            }
            not_modified.headers_mut().insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=60"),
            );
            return not_modified;
        }
    }

    // Cache miss / first fetch — attach ETag and return the full body.
    if let Ok(v) = HeaderValue::from_str(&etag) {
        parts.headers.insert(header::ETAG, v);
    }
    Response::from_parts(parts, Body::from(bytes))
}
