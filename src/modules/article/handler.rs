use axum::extract::{Path, Query, State};
use axum::Json;
use std::time::Duration;

use crate::common::pagination::{paginate, PaginationQuery};
use crate::common::response::{message, ok, ok_paginated};
use crate::config::AppState;
use crate::errors::AppResult;
use crate::models::user::Article;
use crate::utils::cache::keys;

use super::dto::CreateArticleRequest;
use super::service::ArticleService;

pub async fn list(
    State(state): State<AppState>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let limit = pq.take();
    let cache_key = format!("{}:p{}:l{}",
        keys::article_list(), pq.cursor.clone().unwrap_or_else(|| "0".to_string()), limit);

    let articles: Vec<Article> = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(300), || async {
            ArticleService::list(&state.db, limit).await
        })
        .await?;

    let (data, cursor, has_more) = paginate(&articles, limit);
    Ok(Json(ok_paginated(data, cursor, has_more, "en")))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let cache_key = keys::article(&id.to_string());
    let article: Article = state
        .cache
        .get_or_load(&cache_key, Duration::from_secs(600), || async {
            ArticleService::get_by_id(&state.db, id).await
        })
        .await?;
    Ok(Json(ok(article, "en")))
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateArticleRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let article = ArticleService::create(&state.db, body).await?;
    state.cache.invalidate_pattern(keys::articles_pattern()).await;
    Ok(Json(ok(article, "en")))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<CreateArticleRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let article = ArticleService::update(&state.db, id.clone(), body).await?;
    // invalidate both the item cache and the list cache
    state.cache.invalidate(&keys::article(&id)).await;
    state.cache.invalidate_pattern(keys::articles_pattern()).await;
    Ok(Json(ok(article, "en")))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    ArticleService::delete(&state.db, id.clone()).await?;
    state.cache.invalidate(&keys::article(&id)).await;
    state.cache.invalidate_pattern(keys::articles_pattern()).await;
    Ok(Json(message("Article deleted")))
}