use super::handler;
use crate::config::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/{user_id}", axum::routing::post(handler::create))
        .route("/user/{user_id}", axum::routing::put(handler::update))
}
