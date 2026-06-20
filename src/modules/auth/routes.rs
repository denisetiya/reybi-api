use super::handler;
use crate::config::AppState;
use axum::{routing::post, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::login))
        .route("/register", post(handler::register))
        .route("/reset-password", post(handler::reset_password))
}
