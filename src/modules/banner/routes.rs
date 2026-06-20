use super::handler;
use crate::config::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

/// GET routes — public, no auth required.
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list))
        .route("/type/{type}", get(handler::list_by_type))
}

/// Write routes — admin only.
pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handler::create))
        .route("/{id}", put(handler::update).delete(handler::delete))
}