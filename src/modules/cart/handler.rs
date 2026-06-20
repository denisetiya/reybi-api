use axum::{extract::{Path, Query, State}, Json};
use crate::common::locale::Locale;
use std::time::Duration;
use crate::config::AppState;
use crate::common::{response::{ok, ok_paginated, message}, pagination::{PaginationQuery, paginate}};
use crate::errors::AppResult;
use crate::models::Cart;
use crate::utils::cache::keys;
use super::{dto::AddCartRequest, service::CartService};

pub async fn get(State(state): State<AppState>,
    Locale(locale): Locale, Path(user_id): Path<String>, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let limit = pq.take();
    let cache_key = format!(
        "{}:p{}:l{}",
        keys::cart_list(&user_id),
        pq.cursor.clone().unwrap_or_else(|| "0".to_string()),
        limit,
    );

    let items: Vec<Cart> = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(300), || async {
            CartService::get(&state.db, user_id.clone(), limit).await
        })
        .await?;

    let (data, cursor, has_more) = paginate(&items, limit);
    Ok(Json(ok_paginated(data, cursor, has_more, &locale)))
}

pub async fn add(State(state): State<AppState>,
    Locale(locale): Locale, Path(user_id): Path<String>, Json(body): Json<AddCartRequest>) -> AppResult<Json<serde_json::Value>> {
    let item = CartService::add(&state.db, user_id.clone(), body).await?;
    state.cache.invalidate(&keys::cart_list(&user_id)).await;
    state.cache.invalidate_pattern(keys::carts_pattern()).await;
    Ok(Json(ok(item, &locale)))
}

pub async fn delete(State(state): State<AppState>,
    Locale(locale): Locale, Path(id): Path<String>) -> AppResult<Json<serde_json::Value>> {
    CartService::delete(&state.db, id.clone()).await?;
    state.cache.invalidate(&keys::cart(&id)).await;
    state.cache.invalidate_pattern(keys::carts_pattern()).await;
    Ok(Json(message("Cart item removed")))
}
