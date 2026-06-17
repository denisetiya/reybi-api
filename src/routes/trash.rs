use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use crate::config::AppState;
use crate::dto::{CreateTrashTypeRequest, PaginationQuery};
use crate::errors::AppResult;
use crate::services::trash::TrashService;

pub async fn list_trash_types(
    State(state): State<AppState>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let types = TrashService::list(&state.db, &pq).await?;
    let has_more = types.len() as i64 > pq.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { types[..types.len()-1].to_vec() } else { types };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": "en", "pagination": {
            "cursor": if has_more { data.last().map(|t| t.id.to_string()) } else { None },
            "has_more": has_more, "count": data.len()
        }}
    })))
}

pub async fn create_trash_type(
    State(state): State<AppState>,
    Json(body): Json<CreateTrashTypeRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let trash = TrashService::create(&state.db, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": trash,
        "meta": { "locale": "en" }
    })))
}

pub async fn update_trash_type(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateTrashTypeRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let trash = TrashService::update(&state.db, id, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": trash,
        "meta": { "locale": "en" }
    })))
}

pub async fn delete_trash_type(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    TrashService::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Trash type deleted",
        "meta": { "locale": "en" }
    })))
}
