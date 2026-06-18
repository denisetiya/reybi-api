use axum::{routing::{get, post, put, delete}, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/types", get(handler::list))
        .route("/type", post(handler::create))
        .route("/type/:id", put(handler::update).delete(handler::delete))
}
