use axum::extract::{Path, Query, State};
use axum::Json;
use std::time::Duration;

use crate::common::locale::Locale;
use crate::common::pagination::{paginate, PaginationQuery};
use crate::common::response::{ok, ok_paginated, AppResponse};
use crate::config::AppState;
use crate::errors::AppResult;
use crate::models::user::Banner;
use crate::utils::cache::keys;

use super::dto::CreateBannerRequest;
use super::service::BannerService;

pub async fn list(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let limit = pq.take();
    let cache_key = format!(
        "{}:p{}:l{}",
        keys::banner_list(),
        pq.cursor.clone().unwrap_or_else(|| "0".to_string()),
        limit
    );

    let banners: Vec<Banner> = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(300), || async {
            BannerService::list(&state.db, None, pq.cursor.as_deref(), limit).await
        })
        .await?;

    let (data, cursor, has_more) = paginate(&banners, limit);
    Ok(ok_paginated(data, cursor, has_more, &locale))
}

pub async fn list_by_type(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(r#type): Path<String>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let limit = pq.take();
    let cache_key = format!(
        "{}:{}:p{}:l{}",
        keys::banner_list(),
        r#type,
        pq.cursor.clone().unwrap_or_else(|| "0".to_string()),
        limit
    );

    let banners: Vec<Banner> = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(300), || async {
            BannerService::list(&state.db, Some(&r#type), pq.cursor.as_deref(), limit).await
        })
        .await?;

    let (data, cursor, has_more) = paginate(&banners, limit);
    Ok(ok_paginated(data, cursor, has_more, &locale))
}

pub async fn create(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Json(body): Json<CreateBannerRequest>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let banner = BannerService::create(&state.db, &body.image, body.r#type.as_deref()).await?;

    // Invalidate ALL banner caches — list, by-type, and item
    state
        .cache
        .invalidate_pattern(keys::banners_pattern())
        .await;

    Ok(ok(banner, &locale))
}

pub async fn update(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(id): Path<String>,
    Json(body): Json<CreateBannerRequest>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let banner =
        BannerService::update(&state.db, &id, Some(&body.image), body.r#type.as_deref()).await?;

    state
        .cache
        .invalidate_pattern(keys::banners_pattern())
        .await;

    Ok(ok(banner, &locale))
}

pub async fn delete(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(id): Path<String>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let n = BannerService::delete(&state.db, &id).await?;

    state
        .cache
        .invalidate_pattern(keys::banners_pattern())
        .await;

    Ok(ok(serde_json::json!({ "deleted": n }), &locale))
}
