use super::handler;
use crate::config::AppState;
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/products/{id}", get(handler::list_products))
}
