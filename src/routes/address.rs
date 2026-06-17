use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use crate::config::AppState;
use crate::dto::CreateAddressRequest;
use crate::errors::AppResult;
use crate::services::address::AddressService;

pub async fn create_address(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<CreateAddressRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let address = AddressService::create(&state.db, user_id, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": address,
        "meta": { "locale": "en" }
    })))
}

pub async fn update_address(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<CreateAddressRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let address = AddressService::update(&state.db, user_id, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": address,
        "meta": { "locale": "en" }
    })))
}
