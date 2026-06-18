use axum::{routing::{get, post, delete}, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/:user_id", get(handler::get).post(handler::add))
        .route("/item/:id", delete(handler::delete))
}
