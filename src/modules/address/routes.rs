use axum::{routing::{post, put}, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new().route("/user/:user_id", post(handler::create).put(handler::update))
}
