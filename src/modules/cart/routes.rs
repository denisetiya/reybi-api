use super::handler;
use crate::config::AppState;
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/{user_id}", get(handler::get))
        .route("/user/{user_id}", axum::routing::post(handler::add))
        .route("/item/{id}", axum::routing::delete(handler::delete))
}
