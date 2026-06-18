use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use crate::config::AppState;
use crate::common::{response::{ok, ok_paginated, message}, pagination::{PaginationQuery, paginate}};
use crate::errors::AppResult;
use super::{dto::CreateArticleRequest, service::ArticleService};

pub async fn list(State(state): State<AppState>, Query(pq): Query<PaginationQuery>) -> AppResult<Json<serde_json::Value>> {
    let articles = ArticleService::list(&state.db, pq.take()).await?;
    let (data, cursor, has_more) = paginate(&articles, pq.take());
    Ok(Json(ok_paginated(data, cursor, has_more, "en")))
}

pub async fn get(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    let article = ArticleService::get_by_id(&state.db, id).await?;
    Ok(Json(ok(article, "en")))
}

pub async fn create(State(state): State<AppState>, Json(body): Json<CreateArticleRequest>) -> AppResult<Json<serde_json::Value>> {
    let article = ArticleService::create(&state.db, body).await?;
    Ok(Json(ok(article, "en")))
}

pub async fn update(State(state): State<AppState>, Path(id): Path<Uuid>, Json(body): Json<CreateArticleRequest>) -> AppResult<Json<serde_json::Value>> {
    let article = ArticleService::update(&state.db, id, body).await?;
    Ok(Json(ok(article, "en")))
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    ArticleService::delete(&state.db, id).await?;
    Ok(Json(message("Article deleted")))
}