use axum::{routing::{get, post}, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list))
        .route("/type/{type}", get(handler::list_by_type))
        .route("/create", post(handler::create))
}
