use super::handler;
use crate::config::AppState;
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{email}", get(handler::get))
        .route("/{email}", axum::routing::put(handler::update))
}
