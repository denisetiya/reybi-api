use axum::Router;
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/{user_id}", axum::routing::post(handler::create))
        .route("/user/{user_id}", axum::routing::put(handler::update))
}
