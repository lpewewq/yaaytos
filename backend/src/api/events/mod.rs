use crate::api::events::get::{get_event, get_events};
use axum::routing::get;
use axum::Router;

mod get;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_events))
        .route("/:id", get(get_event))
}