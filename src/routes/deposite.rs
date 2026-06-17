use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use crate::config::AppState;
use crate::dto::{CreateDepositeRequest, PaginationQuery};
use crate::errors::AppResult;
use crate::services::deposite::DepositeService;

pub async fn create_deposite(
    State(state): State<AppState>,
    axum::extract::Extension(claims): axum::extract::Extension<crate::services::auth::Claims>,
    Json(body): Json<CreateDepositeRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let user_id = Uuid::parse_str(&claims.id).map_err(|_| crate::errors::AppError::Unauthorized)?;
    let deposite = DepositeService::create(&state.db, user_id, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": deposite,
        "meta": { "locale": "en" }
    })))
}

pub async fn get_user_deposites(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let deposites = DepositeService::get_by_user(&state.db, Some(id), &pq).await?;
    let has_more = deposites.len() as i64 > pq.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { deposites[..deposites.len()-1].to_vec() } else { deposites };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": "en", "pagination": {
            "cursor": if has_more { data.last().map(|d| d.id.to_string()) } else { None },
            "has_more": has_more, "count": data.len()
        }}
    })))
}

pub async fn get_all_deposites(
    State(state): State<AppState>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let deposites = DepositeService::get_by_user(&state.db, None, &pq).await?;
    let has_more = deposites.len() as i64 > pq.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { deposites[..deposites.len()-1].to_vec() } else { deposites };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": "en", "pagination": {
            "cursor": if has_more { data.last().map(|d| d.id.to_string()) } else { None },
            "has_more": has_more, "count": data.len()
        }}
    })))
}
