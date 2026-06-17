use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use crate::config::AppState;
use crate::dto::*;
use crate::errors::AppResult;
use crate::services::order::OrderService;

pub async fn create_order(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<CreateOrderRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let order = OrderService::create(&state.db, user_id, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": order,
        "meta": { "locale": "en" }
    })))
}

pub async fn get_orders(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let orders = OrderService::get_by_user(&state.db, user_id, &pq).await?;
    let has_more = orders.len() as i64 > pq.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { orders[..orders.len()-1].to_vec() } else { orders };
    let next_cursor = if has_more { data.last().map(|o| o.id.to_string()) } else { None };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": "en", "pagination": { "cursor": next_cursor, "has_more": has_more, "count": data.len() } }
    })))
}

pub async fn get_all_orders(
    State(state): State<AppState>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let orders = OrderService::get_all(&state.db, &pq).await?;
    let has_more = orders.len() as i64 > pq.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { orders[..orders.len()-1].to_vec() } else { orders };
    let next_cursor = if has_more { data.last().map(|o| o.id.to_string()) } else { None };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": "en", "pagination": { "cursor": next_cursor, "has_more": has_more, "count": data.len() } }
    })))
}

pub async fn delete_order(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    OrderService::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order cancelled",
        "meta": { "locale": "en" }
    })))
}
