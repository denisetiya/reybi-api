use super::{dto::CreateOrderRequest, service::OrderService};
use crate::common::locale::Locale;
use crate::common::response::AppResponse;
use crate::common::{
    pagination::{paginate, PaginationQuery},
    response::{message, ok, ok_paginated},
};
use crate::config::AppState;
use crate::errors::AppResult;
use crate::models::Order;
use crate::utils::cache::keys;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::time::Duration;

pub async fn create(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(user_id): Path<String>,
    Json(body): Json<CreateOrderRequest>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let order = OrderService::create(&state.db, user_id.clone(), body).await?;
    state.cache.invalidate_pattern(keys::orders_pattern()).await;
    Ok(ok(order, &locale))
}

pub async fn list_user(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(user_id): Path<String>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let limit = pq.take();
    let scope = format!("u:{user_id}");
    let cache_key = format!(
        "{}:p{}:l{}",
        keys::order_list(&scope),
        pq.cursor.clone().unwrap_or_else(|| "0".to_string()),
        limit,
    );

    let orders: Vec<Order> = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(300), || async {
            OrderService::get_by_user(&state.db, user_id.clone(), limit).await
        })
        .await?;

    let (data, cursor, has_more) = paginate(&orders, limit);
    Ok(ok_paginated(data, cursor, has_more, &locale))
}

pub async fn list_all(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let limit = pq.take();
    let cache_key = format!(
        "{}:p{}:l{}",
        keys::order_list("all"),
        pq.cursor.clone().unwrap_or_else(|| "0".to_string()),
        limit,
    );

    let orders: Vec<Order> = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(300), || async {
            OrderService::get_all(&state.db, limit).await
        })
        .await?;

    let (data, cursor, has_more) = paginate(&orders, limit);
    Ok(ok_paginated(data, cursor, has_more, &locale))
}

pub async fn delete(
    State(state): State<AppState>,
    Locale(_locale): Locale,
    Path(id): Path<String>,
) -> AppResult<AppResponse<serde_json::Value>> {
    OrderService::delete(&state.db, id.clone()).await?;
    state.cache.invalidate(&keys::order(&id)).await;
    state.cache.invalidate_pattern(keys::orders_pattern()).await;
    Ok(message("Order cancelled"))
}
