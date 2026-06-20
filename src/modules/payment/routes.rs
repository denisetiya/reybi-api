use axum::{routing::post, Router};
use crate::config::AppState;
use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/snap", post(handler::create_snap))
        .route("/midtrans/webhook", post(handler::webhook))
}