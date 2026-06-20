use axum::{routing::get, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list))
        .route("/create", axum::routing::post(handler::create))
        .route("/{id}", axum::routing::put(handler::update))
        .route("/{id}", axum::routing::delete(handler::delete))
}
