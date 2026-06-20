use super::service::SallerService;
use crate::common::locale::Locale;
use crate::common::response::AppResponse;
use crate::common::{
    pagination::{paginate, PaginationQuery},
    response::ok_paginated,
};
use crate::config::AppState;
use crate::errors::AppResult;
use crate::models::Product;
use crate::utils::cache::keys;
use axum::extract::{Path, Query, State};
use std::time::Duration;

pub async fn list_products(
    State(state): State<AppState>,
    Locale(locale): Locale,
    Path(id): Path<String>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<AppResponse<serde_json::Value>> {
    let limit = pq.take();
    let cache_key = format!(
        "{}:p{}:l{}",
        keys::saller_products(&id),
        pq.cursor.clone().unwrap_or_else(|| "0".to_string()),
        limit,
    );

    let items: Vec<Product> = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(300), || async {
            SallerService::get_products(&state.db, id.clone(), limit).await
        })
        .await?;

    let (data, cursor, has_more) = paginate(&items, limit);
    Ok(ok_paginated(data, cursor, has_more, &locale))
}
