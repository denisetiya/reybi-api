use axum::{extract::{Path, Query, State}, Json};
use crate::common::locale::Locale;
use std::time::Duration;
use crate::config::AppState;
use crate::common::{response::{ok, ok_paginated, message}, pagination::{PaginationQuery, paginate}};
use crate::errors::AppResult;
use crate::models::TrashType;
use crate::utils::cache::keys;
use super::{dto::CreateTrashTypeRequest, service::TrashService};

pub async fn list(State(state): State<AppState>,
    Locale(locale): Locale, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let limit = pq.take();
    let cache_key = format!(
        "{}:p{}:l{}",
        keys::trash_list(),
        pq.cursor.clone().unwrap_or_else(|| "0".to_string()),
        limit,
    );

    let items: Vec<TrashType> = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(300), || async {
            TrashService::list(&state.db, limit).await
        })
        .await?;

    let (data, cursor, has_more) = paginate(&items, limit);
    Ok(Json(ok_paginated(data, cursor, has_more, &locale)))
}

pub async fn create(State(state): State<AppState>,
    Locale(locale): Locale, Json(body): Json<CreateTrashTypeRequest>) -> AppResult<Json<serde_json::Value>> {
    let item = TrashService::create(&state.db, body).await?;
    state.cache.invalidate_pattern(keys::trash_pattern()).await;
    Ok(Json(ok(item, &locale)))
}

pub async fn update(State(state): State<AppState>,
    Locale(locale): Locale, Path(id): Path<String>, Json(body): Json<CreateTrashTypeRequest>) -> AppResult<Json<serde_json::Value>> {
    let item = TrashService::update(&state.db, id.clone(), body).await?;
    state.cache.invalidate(&keys::trash(&id)).await;
    state.cache.invalidate_pattern(keys::trash_pattern()).await;
    Ok(Json(ok(item, &locale)))
}

pub async fn delete(State(state): State<AppState>,
    Locale(locale): Locale, Path(id): Path<String>) -> AppResult<Json<serde_json::Value>> {
    TrashService::delete(&state.db, id.clone()).await?;
    state.cache.invalidate(&keys::trash(&id)).await;
    state.cache.invalidate_pattern(keys::trash_pattern()).await;
    Ok(Json(message("Trash type deleted")))
}
