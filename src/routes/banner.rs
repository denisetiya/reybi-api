use axum::{
    extract::{Path, Query, State},
    Json,
};
use crate::config::AppState;
use crate::dto::{CreateBannerRequest, PaginationQuery};
use crate::errors::AppResult;
use crate::services::banner::BannerService;

pub async fn list_banners(
    State(state): State<AppState>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let banners = BannerService::list(&state.db, None, &pq).await?;
    let has_more = banners.len() as i64 > pq.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { banners[..banners.len()-1].to_vec() } else { banners };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": "en", "pagination": {
            "cursor": if has_more { data.last().map(|b| b.id.to_string()) } else { None },
            "has_more": has_more, "count": data.len()
        }}
    })))
}

pub async fn list_banners_by_type(
    State(state): State<AppState>,
    Path(r#type): Path<String>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let banners = BannerService::list(&state.db, Some(&r#type), &pq).await?;
    let has_more = banners.len() as i64 > pq.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { banners[..banners.len()-1].to_vec() } else { banners };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": "en", "pagination": {
            "cursor": if has_more { data.last().map(|b| b.id.to_string()) } else { None },
            "has_more": has_more, "count": data.len()
        }}
    })))
}

pub async fn create_banner(
    State(state): State<AppState>,
    Json(body): Json<CreateBannerRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let banner = BannerService::create(&state.db, &body.image, body.r#type.as_deref()).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": banner,
        "meta": { "locale": "en" }
    })))
}
