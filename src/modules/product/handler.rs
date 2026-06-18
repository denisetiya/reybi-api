use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use crate::config::AppState;
use crate::common::{response::{ok, ok_paginated, message}, pagination::paginate};
use crate::errors::{AppError, AppResult};
use super::{dto::*, service::ProductService};

pub async fn list(
    State(state): State<AppState>,
    Query(filter): Query<ProductFilter>,
) -> AppResult<Json<serde_json::Value>> {
    let products = ProductService::list(&state.db, &filter).await?;
    let limit = filter.limit.unwrap_or(25);
    let (data, cursor, has_more) = paginate(&products, limit);
    Ok(Json(ok_paginated(data, cursor, has_more)))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let product = ProductService::get_by_id(&state.db, id).await?
        .ok_or_else(|| AppError::NotFound("Product not found".into()))?;
    Ok(Json(ok(product)))
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateProductRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let product = ProductService::create(
        &state.db, &body.name, body.price, body.stock,
        &body.description, &body.category, body.location.as_deref(),
        body.discount, body.coin, body.recommended,
        body.saller_id, body.thumbnail.as_deref(), body.images,
    ).await?;
    Ok(Json(ok(product)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateProductRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let product = ProductService::update(
        &state.db, id,
        body.name.as_deref(), body.price, body.stock,
        body.description.as_deref(), body.category.as_deref(),
        body.location.as_deref(), body.discount, body.coin,
        body.recommended, body.thumbnail.as_deref(), body.images,
    ).await?;
    Ok(Json(ok(product)))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    ProductService::delete(&state.db, id).await?;
    Ok(Json(message("Product deleted")))
}

pub async fn create_variant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateVariantRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let variant = ProductService::add_variant(&state.db, id, body).await?;
    Ok(Json(ok(variant)))
}
