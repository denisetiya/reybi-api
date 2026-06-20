use super::handler;
use crate::config::AppState;
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_all))
        .route("/user/{user_id}", get(handler::list_user))
        .route("/user/{user_id}", axum::routing::post(handler::create))
        .route("/{id}", axum::routing::delete(handler::delete))
}
