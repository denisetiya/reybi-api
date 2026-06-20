use super::handler;
use crate::config::AppState;
use axum::{routing::post, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/snap", post(handler::create_snap))
        .route("/midtrans/webhook", post(handler::webhook))
}
