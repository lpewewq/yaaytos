use crate::api::persons::get::{get_person, get_persons};
use axum::routing::get;
use axum::Router;
use crate::state::AppState;

mod get;

pub fn router(state: &AppState) -> Router {
    Router::new()
        .route("/", get(get_persons))
        .route("/:id", get(get_person))
        .with_state(state.clone())
}