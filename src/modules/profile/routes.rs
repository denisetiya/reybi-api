use axum::{routing::{get, put}, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new().route("/:email", get(handler::get).put(handler::update))
}
