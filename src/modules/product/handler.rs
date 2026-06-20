use axum::{extract::{Path, Query, State}, Json};
use crate::common::locale::Locale;
use std::time::Duration;
use crate::config::AppState;
use crate::common::{response::{ok, ok_paginated, message}, pagination::paginate};
use crate::errors::{AppError, AppResult};
use crate::models::Product;
use crate::utils::cache::keys;
use super::{dto::*, service::ProductService};

pub async fn list(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Query(filter): Query<ProductFilter>,
) -> AppResult<Json<serde_json::Value>> {
    let limit = filter.limit.unwrap_or(25);
    // Encode all filter dimensions into the cache key so different queries
    // don't collide on the same entry.
    let cache_key = format!(
        "{}:c{}:s{}:p{}:l{}",
        keys::product_list(filter.category.as_deref(), 0),
        filter.cursor.clone().unwrap_or_else(|| "0".to_string()),
        filter.search.clone().unwrap_or_else(|| "_".to_string()),
        0,
        limit,
    );

    let products: Vec<Product> = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(300), || async {
            ProductService::list(&state.db, &filter).await
        })
        .await?;

    let (data, cursor, has_more) = paginate(&products, limit);
    Ok(Json(ok_paginated(data, cursor, has_more, &locale)))
}

pub async fn get(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let cache_key = keys::product(&id);
    let product: Product = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(600), || async {
            ProductService::get_by_id(&state.db, id)
                .await?
                .ok_or_else(|| AppError::NotFound("Product not found".into()))
        })
        .await?;
    Ok(Json(ok(product, &locale)))
}

pub async fn create(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Json(body): Json<CreateProductRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let product = ProductService::create(&state.db, body).await?;
    state.cache.invalidate_pattern(keys::products_pattern()).await;
    Ok(Json(ok(product, &locale)))
}

pub async fn update(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(id): Path<String>,
    Json(body): Json<UpdateProductRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let product = ProductService::update(&state.db, id.clone(), body).await?;
    state.cache.invalidate(&keys::product(&id)).await;
    state.cache.invalidate_pattern(keys::products_pattern()).await;
    Ok(Json(ok(product, &locale)))
}

pub async fn delete(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    ProductService::delete(&state.db, id.clone()).await?;
    state.cache.invalidate(&keys::product(&id)).await;
    state.cache.invalidate_pattern(keys::products_pattern()).await;
    Ok(Json(message("Product deleted")))
}

pub async fn create_variant(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(id): Path<String>,
    Json(body): Json<CreateVariantRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let variant = ProductService::add_variant(&state.db, id.clone(), body).await?;
    // Adding a variant changes the parent product — drop both caches.
    state.cache.invalidate(&keys::product(&id)).await;
    state.cache.invalidate_pattern(keys::products_pattern()).await;
    Ok(Json(ok(variant, &locale)))
}
