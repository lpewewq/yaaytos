use crate::api::participations::get::get_participations;
use crate::state::AppState;
use axum::routing::get;
use axum::Router;

mod get;

pub fn router(state: &AppState) -> Router {
    Router::new()
        .route("/", get(get_participations))
        .with_state(state.clone())
}