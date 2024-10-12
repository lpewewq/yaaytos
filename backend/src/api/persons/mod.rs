use crate::api::persons::get::{get_person, get_persons};
use crate::state::AppState;
use axum::routing::get;
use axum::Router;

mod get;
mod errors;

pub fn router(state: &AppState) -> Router {
    Router::new()
        .route("/", get(get_persons))
        .route("/:uuid", get(get_person))
        .with_state(state.clone())
}