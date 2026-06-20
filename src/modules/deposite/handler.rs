use axum::{extract::{Path, Query, State}, Json};
use crate::config::AppState;
use crate::common::{response::{ok, ok_paginated}, pagination::{PaginationQuery, paginate}};
use crate::errors::AppResult;
use super::{dto::CreateDepositeRequest, service::DepositeService};

pub async fn create(State(state): State<AppState>, Json(body): Json<CreateDepositeRequest>) -> AppResult<Json<serde_json::Value>> {
    let deposite = DepositeService::create(&state.db, cuid2::create_id(), body).await?;
    Ok(Json(ok(deposite, "en")))
}

pub async fn list_user(State(state): State<AppState>, Path(id): Path<String>, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let items = DepositeService::get_by_user(&state.db, Some(id), pq.take()).await?;
    let (data, cursor, has_more) = paginate(&items, pq.take());
    Ok(Json(ok_paginated(data, cursor, has_more, "en")))
}

pub async fn list_all(State(state): State<AppState>, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let items = DepositeService::get_by_user(&state.db, None, pq.take()).await?;
    let (data, cursor, has_more) = paginate(&items, pq.take());
    Ok(Json(ok_paginated(data, cursor, has_more, "en")))
}