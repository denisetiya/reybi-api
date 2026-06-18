use axum::{routing::{get, post}, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list))
        .route("/create", post(handler::create))
        .route("/:id", get(handler::get).put(handler::update).delete(handler::delete))
}
