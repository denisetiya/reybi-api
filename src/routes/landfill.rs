use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use crate::config::AppState;
use crate::dto::{CreateLandfillRequest, PaginationQuery};
use crate::errors::AppResult;
use crate::services::landfill::LandfillService;

pub async fn list_landfills(
    State(state): State<AppState>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let landfills = LandfillService::list(&state.db, &pq).await?;
    let has_more = landfills.len() as i64 > pq.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { landfills[..landfills.len()-1].to_vec() } else { landfills };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": {
            "locale": "en",
            "pagination": {
                "cursor": if has_more { data.last().map(|l| l.id.to_string()) } else { None },
                "has_more": has_more,
                "count": data.len()
            }
        }
    })))
}

pub async fn create_landfill(
    State(state): State<AppState>,
    Json(body): Json<CreateLandfillRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let landfill = LandfillService::create(&state.db, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": landfill,
        "meta": { "locale": "en" }
    })))
}

pub async fn update_landfill(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateLandfillRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let landfill = LandfillService::update(&state.db, id, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": landfill,
        "meta": { "locale": "en" }
    })))
}

pub async fn delete_landfill(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    LandfillService::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Landfill deleted",
        "meta": { "locale": "en" }
    })))
}
