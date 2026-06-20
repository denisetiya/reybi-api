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
        .route("/{id}", get(handler::get))
}

/// POST / PUT / DELETE — require a valid JWT.
/// Note: do NOT register GET handlers here — they live in public_routes.
pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handler::create))
        .route("/variant/{id}", post(handler::create_variant))
        .route(
            "/{id}",
            axum::routing::put(handler::update).delete(handler::delete),
        )
}