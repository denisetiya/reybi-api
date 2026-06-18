use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use crate::config::AppState;
use crate::common::{response::{ok, ok_paginated, message}, pagination::{PaginationQuery, paginate}};
use crate::errors::AppResult;
use super::{dto::CreateLandfillRequest, service::LandfillService};

pub async fn list(State(state): State<AppState>, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let items = LandfillService::list(&state.db, pq.take()).await?;
    let (data, cursor, has_more) = paginate(&items, pq.take());
    Ok(Json(ok_paginated(data, cursor, has_more)))
}

pub async fn create(State(state): State<AppState>, Json(body): Json<CreateLandfillRequest>) -> AppResult<Json<serde_json::Value>> {
    let item = LandfillService::create(&state.db, body).await?;
    Ok(Json(ok(item)))
}

pub async fn update(State(state): State<AppState>, Path(id): Path<Uuid>, Json(body): Json<CreateLandfillRequest>) -> AppResult<Json<serde_json::Value>> {
    let item = LandfillService::update(&state.db, id, body).await?;
    Ok(Json(ok(item)))
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    LandfillService::delete(&state.db, id).await?;
    Ok(Json(message("Landfill deleted")))
}
