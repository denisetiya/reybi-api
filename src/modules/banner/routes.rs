use super::handler;
use crate::config::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list))
        .route("/type/{type}", get(handler::list_by_type))
        .route("/create", post(handler::create))
}
