use axum::{routing::get, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/:email", get(handler::get))
        .route("/:email", axum::routing::put(handler::update))
}
