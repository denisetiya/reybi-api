use super::handler;
use crate::config::AppState;
use axum::{routing::get, Router};

/// User routes — create deposit + view own deposits.
pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::post(handler::create))
        .route("/user/{id}", get(handler::list_user))
}

/// Admin-only — list ALL deposits.
pub fn admin_routes() -> Router<AppState> {
    Router::new().route("/", get(handler::list_all))
}
