use axum::{routing::{get, post, delete}, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_all))
        .route("/user/:user_id", get(handler::list_user).post(handler::create))
        .route("/:id", delete(handler::delete))
}
