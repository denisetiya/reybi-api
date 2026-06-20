use axum::{routing::get, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_all))
        .route("/", axum::routing::post(handler::create))
        .route("/user/{id}", get(handler::list_user))
}
