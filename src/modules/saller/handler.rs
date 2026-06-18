use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use crate::config::AppState;
use crate::common::{response::ok_paginated, pagination::{PaginationQuery, paginate}};
use crate::errors::AppResult;
use super::service::SallerService;

pub async fn list_products(State(state): State<AppState>, Path(id): Path<Uuid>, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let items = SallerService::get_products(&state.db, id, pq.take()).await?;
    let (data, cursor, has_more) = paginate(&items, pq.take());
    Ok(Json(ok_paginated(data, cursor, has_more, "en")))
}
