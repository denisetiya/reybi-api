use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use crate::config::AppState;
use crate::dto::{CreateArticleRequest, PaginationQuery};
use crate::errors::AppResult;
use crate::services::article::ArticleService;

pub async fn list_articles(
    State(state): State<AppState>,
    Query(pq): Query<PaginationQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let articles = ArticleService::list(&state.db, &pq).await?;
    let has_more = articles.len() as i64 > pq.limit.unwrap_or(25);
    let data: Vec<_> = if has_more { articles[..articles.len()-1].to_vec() } else { articles };
    Ok(Json(serde_json::json!({
        "success": true,
        "data": data,
        "meta": { "locale": "en", "pagination": {
            "cursor": if has_more { data.last().map(|a| a.id.to_string()) } else { None },
            "has_more": has_more, "count": data.len()
        }}
    })))
}

pub async fn get_article(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let article = ArticleService::get_by_id(&state.db, id).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": article,
        "meta": { "locale": "en" }
    })))
}

pub async fn create_article(
    State(state): State<AppState>,
    Json(body): Json<CreateArticleRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let article = ArticleService::create(&state.db, &body.thumbnail, &body.header, &body.content).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": article,
        "meta": { "locale": "en" }
    })))
}

pub async fn update_article(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateArticleRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let article = ArticleService::update(&state.db, id, body).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "data": article,
        "meta": { "locale": "en" }
    })))
}

pub async fn delete_article(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    ArticleService::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Article deleted",
        "meta": { "locale": "en" }
    })))
}
