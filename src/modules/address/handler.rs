use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use crate::config::AppState;
use crate::common::response::ok;
use crate::errors::AppResult;
use super::{dto::CreateAddressRequest, service::AddressService};

pub async fn create(State(state): State<AppState>, Path(user_id): Path<Uuid>, Json(body): Json<CreateAddressRequest>) -> AppResult<Json<serde_json::Value>> {
    let address = AddressService::create(&state.db, user_id, body).await?;
    Ok(Json(ok(address)))
}

pub async fn update(State(state): State<AppState>, Path(user_id): Path<Uuid>, Json(body): Json<CreateAddressRequest>) -> AppResult<Json<serde_json::Value>> {
    let address = AddressService::update(&state.db, user_id, body).await?;
    Ok(Json(ok(address)))
}
