use axum::{extract::{Path, Query, State}, Json};
use crate::config::AppState;
use crate::common::{response::{ok, ok_paginated}, pagination::{PaginationQuery, paginate}};
use crate::errors::AppResult;
use super::{dto::CreateBannerRequest, service::BannerService};

pub async fn list(
    State(state): State<AppState>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let limit = pq.take();
    let banners = BannerService::list(&state.db, None, limit).await?;
    let (data, cursor, has_more) = paginate(&banners, limit);
    Ok(Json(ok_paginated(data, cursor, has_more, "en")))
}

pub async fn list_by_type(
    State(state): State<AppState>,
    Path(r#type): Path<String>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let limit = pq.take();
    let banners = BannerService::list(&state.db, Some(&r#type), limit).await?;
    let (data, cursor, has_more) = paginate(&banners, limit);
    Ok(Json(ok_paginated(data, cursor, has_more, "en")))
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateBannerRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let banner = BannerService::create(&state.db, &body.image, body.r#type.as_deref()).await?;
    Ok(Json(ok(banner, "en")))
}
