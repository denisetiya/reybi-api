use super::handler;
use crate::config::AppState;
use axum::{routing::get, Router};

/// User routes — only the user's own orders.
pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/user/{user_id}", get(handler::list_user))
        .route("/user/{user_id}", axum::routing::post(handler::create))
        .route("/{id}", axum::routing::delete(handler::delete))
}

/// Admin-only — list ALL orders across the system.
pub fn admin_routes() -> Router<AppState> {
    Router::new().route("/", get(handler::list_all))
}