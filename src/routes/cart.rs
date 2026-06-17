use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use crate::config::AppState;
use crate::dto::*;
use crate::errors::AppResult;
use crate::services::cart::CartService;

pub async fn get_cart(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let items = CartService::get(&state.db, user_id, &pq).await?;
    let has_more = items.len() as i64 > pq.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { items[..items.len()-1].to_vec() } else { items };
    let next_cursor = if has_more { data.last().map(|i| i.id.to_string()) } else { None };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": "en", "pagination": { "cursor": next_cursor, "has_more": has_more, "count": data.len() } }
    })))
}

pub async fn add_cart(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<AddCartRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let item = CartService::add(&state.db, user_id, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": item,
        "meta": { "locale": "en" }
    })))
}

pub async fn delete_cart(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    CartService::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Cart item removed",
        "meta": { "locale": "en" }
    })))
}
