use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use crate::config::AppState;
use crate::dto::PaginationQuery;
use crate::errors::AppResult;
use crate::services::saller::SallerService;

pub async fn get_saller_products(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let products = SallerService::get_products(&state.db, id, &pq).await?;
    let has_more = products.len() as i64 > pq.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { products[..products.len()-1].to_vec() } else { products };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": "en", "pagination": {
            "cursor": if has_more { data.last().map(|p| p.id.to_string()) } else { None },
            "has_more": has_more, "count": data.len()
        }}
    })))
}
