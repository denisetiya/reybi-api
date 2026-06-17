use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use crate::config::AppState;
use crate::dto::*;
use crate::errors::{AppError, AppResult};
use crate::services::product::ProductService;
use crate::i18n::messages::t;

pub async fn list_products(
    State(state): State<AppState>,
    Query(filter): Query<ProductFilter>,
) -> AppResult<Json<serde_json::Value>> {
    let products = ProductService::list(&state.db, &filter).await?;
    let has_more = products.len() as i64 > filter.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { products[..products.len()-1].to_vec() } else { products };
    let next_cursor = if has_more { data.last().map(|p| p.id.to_string()) } else { None };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": {
            "locale": "en",
            "pagination": { "cursor": next_cursor, "has_more": has_more, "count": data.len() }
        }
    })))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let product = ProductService::get_by_id(&state.db, id).await?
        .ok_or_else(|| AppError::NotFound(t("en", "PRODUCT_NOT_FOUND")))?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": product,
        "meta": { "locale": "en" }
    })))
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(body): Json<CreateProductRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let product = ProductService::create(
        &state.db,
        &body.name, body.price, body.stock,
        &body.description, &body.category,
        body.location.as_deref(), body.discount,
        body.coin, body.recommended,
        body.saller_id,
        body.thumbnail.as_deref(), body.images,
    ).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": product,
        "meta": { "locale": "en" }
    })))
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateProductRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let product = ProductService::update(
        &state.db, id,
        body.name.as_deref(), body.price,
        body.stock, body.description.as_deref(),
        body.category.as_deref(), body.location.as_deref(),
        body.discount, body.coin,
        body.recommended, body.thumbnail.as_deref(),
        body.images,
    ).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": product,
        "meta": { "locale": "en" }
    })))
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    ProductService::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Product deleted",
        "meta": { "locale": "en" }
    })))
}

pub async fn create_variant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateVariantRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let variant = ProductService::add_variant(
        &state.db, id, &body.name, body.price, body.stock, body.image.as_deref(),
    ).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": variant,
        "meta": { "locale": "en" }
    })))
}
