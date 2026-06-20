use axum::{extract::{Path, Query, State}, Json};
use crate::config::AppState;
use crate::common::{response::{ok, ok_paginated, message}, pagination::{PaginationQuery, paginate}};
use crate::errors::AppResult;
use super::{dto::AddCartRequest, service::CartService};

pub async fn get(State(state): State<AppState>, Path(user_id): Path<String>, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let items = CartService::get(&state.db, user_id, pq.take()).await?;
    let (data, cursor, has_more) = paginate(&items, pq.take());
    Ok(Json(ok_paginated(data, cursor, has_more, "en")))
}

pub async fn add(State(state): State<AppState>, Path(user_id): Path<String>, Json(body): Json<AddCartRequest>) -> AppResult<Json<serde_json::Value>> {
    let item = CartService::add(&state.db, user_id, body).await?;
    Ok(Json(ok(item, "en")))
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<String>) -> AppResult<Json<serde_json::Value>> {
    CartService::delete(&state.db, id.clone()).await?;
    Ok(Json(message("Cart item removed")))
}