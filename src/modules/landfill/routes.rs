use axum::{routing::{get, post, put, delete}, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list))
        .route("/create", post(handler::create))
        .route("/:id", put(handler::update).delete(handler::delete))
}
