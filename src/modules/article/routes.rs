use super::handler;
use crate::config::AppState;
use axum::{
    routing::{get, post},
    Router,
};

/// GET routes — public, no auth required.
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list))
        .route(
            "/{id}",
            get(handler::get),
        )
}

/// Write routes — JWT required.
pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handler::create))
        .route(
            "/{id}",
            axum::routing::put(handler::update).delete(handler::delete),
        )
}