use axum::{routing::get, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_all))
        .route("/user/:user_id", get(handler::list_user))
        .route("/user/:user_id", axum::routing::post(handler::create))
        .route("/:id", axum::routing::delete(handler::delete))
}
