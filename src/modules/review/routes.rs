use super::handler;
use crate::config::AppState;
use axum::{
    routing::{post, put},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create))
        .route("/{id}", put(handler::update))
}
