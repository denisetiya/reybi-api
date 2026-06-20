use axum::{extract::{Path, Query, State}, Json};
use crate::common::locale::Locale;
use std::time::Duration;
use crate::config::AppState;
use crate::common::{response::{ok, ok_paginated, message}, pagination::{PaginationQuery, paginate}};
use crate::errors::AppResult;
use crate::models::Deposite;
use crate::utils::cache::keys;
use super::{dto::CreateDepositeRequest, service::DepositeService};

pub async fn create(State(state): State<AppState>,
    Locale(locale): Locale, Json(body): Json<CreateDepositeRequest>) -> AppResult<Json<serde_json::Value>> {
    let deposite = DepositeService::create(&state.db, cuid2::create_id(), body).await?;
    state.cache.invalidate_pattern(keys::deposites_pattern()).await;
    Ok(Json(ok(deposite, &locale)))
}

pub async fn list_user(State(state): State<AppState>,
    Locale(locale): Locale, Path(id): Path<String>, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let limit = pq.take();
    let scope = format!("u:{}", id);
    let cache_key = format!(
        "{}:p{}:l{}",
        keys::deposite_list(&scope),
        pq.cursor.clone().unwrap_or_else(|| "0".to_string()),
        limit,
    );

    let items: Vec<Deposite> = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(300), || async {
            DepositeService::get_by_user(&state.db, Some(id.clone()), limit).await
        })
        .await?;

    let (data, cursor, has_more) = paginate(&items, limit);
    Ok(Json(ok_paginated(data, cursor, has_more, &locale)))
}

pub async fn list_all(State(state): State<AppState>,
    Locale(locale): Locale, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let limit = pq.take();
    let cache_key = format!(
        "{}:p{}:l{}",
        keys::deposite_list("all"),
        pq.cursor.clone().unwrap_or_else(|| "0".to_string()),
        limit,
    );

    let items: Vec<Deposite> = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(300), || async {
            DepositeService::get_by_user(&state.db, None, limit).await
        })
        .await?;

    let (data, cursor, has_more) = paginate(&items, limit);
    Ok(Json(ok_paginated(data, cursor, has_more, &locale)))
}
