use axum::{routing::post, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::login))
        .route("/register", post(handler::register))
        .route("/reset-password", post(handler::reset_password))
}
