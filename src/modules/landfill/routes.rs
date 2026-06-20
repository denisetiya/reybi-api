use super::handler;
use crate::config::AppState;
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list))
        .route("/create", axum::routing::post(handler::create))
        .route("/{id}", axum::routing::put(handler::update))
        .route("/{id}", axum::routing::delete(handler::delete))
}
