use axum::{routing::{post, put}, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create))
        .route("/:id", put(handler::update))
}
