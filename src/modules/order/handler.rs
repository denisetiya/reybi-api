use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use crate::config::AppState;
use crate::common::{response::{ok, ok_paginated, message}, pagination::{PaginationQuery, paginate}};
use crate::errors::AppResult;
use super::{dto::CreateOrderRequest, service::OrderService};

pub async fn create(State(state): State<AppState>, Path(user_id): Path<Uuid>, Json(body): Json<CreateOrderRequest>) -> AppResult<Json<serde_json::Value>> {
    let order = OrderService::create(&state.db, user_id, body).await?;
    Ok(Json(ok(order, "en")))
}

pub async fn list_user(State(state): State<AppState>, Path(user_id): Path<Uuid>, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let orders = OrderService::get_by_user(&state.db, user_id, pq.take()).await?;
    let (data, cursor, has_more) = paginate(&orders, pq.take());
    Ok(Json(ok_paginated(data, cursor, has_more, "en")))
}

pub async fn list_all(State(state): State<AppState>, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let orders = OrderService::get_all(&state.db, pq.take()).await?;
    let (data, cursor, has_more) = paginate(&orders, pq.take());
    Ok(Json(ok_paginated(data, cursor, has_more, "en")))
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    OrderService::delete(&state.db, id).await?;
    Ok(Json(message("Order cancelled")))
}