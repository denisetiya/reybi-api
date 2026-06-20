use super::handler;
use crate::config::AppState;
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/types", get(handler::list))
        .route("/type", axum::routing::post(handler::create))
        .route("/type/{id}", axum::routing::put(handler::update))
        .route("/type/{id}", axum::routing::delete(handler::delete))
}
