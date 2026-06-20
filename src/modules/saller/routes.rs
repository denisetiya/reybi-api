use axum::{routing::get, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new().route("/products/{id}", get(handler::list_products))
}
