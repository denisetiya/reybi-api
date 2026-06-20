use super::handler;
use crate::config::AppState;
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_all))
        .route("/", axum::routing::post(handler::create))
        .route("/user/{id}", get(handler::list_user))
}
