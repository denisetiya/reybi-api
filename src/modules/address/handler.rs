use super::{dto::CreateAddressRequest, service::AddressService};
use crate::common::locale::Locale;
use crate::common::response::ok;
use crate::config::AppState;
use crate::errors::AppResult;
use crate::utils::cache::keys;
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn create(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(user_id): Path<String>,
    Json(body): Json<CreateAddressRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let address = AddressService::create(&state.db, user_id.clone(), body).await?;
    state.cache.invalidate(&keys::address_list(&user_id)).await;
    state
        .cache
        .invalidate_pattern(keys::addresses_pattern())
        .await;
    Ok(Json(ok(address, &locale)))
}

pub async fn update(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(user_id): Path<String>,
    Json(body): Json<CreateAddressRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let address = AddressService::update(&state.db, user_id.clone(), body).await?;
    state.cache.invalidate(&keys::address(&user_id)).await;
    state.cache.invalidate(&keys::address_list(&user_id)).await;
    state
        .cache
        .invalidate_pattern(keys::addresses_pattern())
        .await;
    Ok(Json(ok(address, &locale)))
}
