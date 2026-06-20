use super::handler;
use crate::config::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list))
        .route("/create", post(handler::create))
        .route("/variant/{id}", post(handler::create_variant))
        .route(
            "/{id}",
            get(handler::get)
                .put(handler::update)
                .delete(handler::delete),
        )
}
