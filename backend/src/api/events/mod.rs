use crate::api::events::get::{get_event, get_events};
use crate::state::AppState;
use axum::routing::get;
use axum::Router;

mod get;

pub fn router(state: &AppState) -> Router {
    Router::new()
        .route("/", get(get_events))
        .route("/:id", get(get_event))
        .with_state(state.clone())
}