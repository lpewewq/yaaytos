use crate::api::seasons::get::{get_season, get_seasons};
use crate::state::AppState;
use axum::routing::get;
use axum::Router;

mod get;
mod errors;

pub fn router(state: &AppState) -> Router {
    Router::new()
        .route("/", get(get_seasons))
        .route("/:uuid", get(get_season))
        .with_state(state.clone())
}