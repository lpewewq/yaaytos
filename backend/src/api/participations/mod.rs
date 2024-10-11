use crate::api::participations::get::{get_participation, get_participations};
use axum::routing::get;
use axum::Router;
use crate::state::AppState;

mod get;

pub fn router(state: &AppState) -> Router {
    Router::new()
        .route("/", get(get_participations))
        .route("/:id", get(get_participation))
        .with_state(state.clone())
}