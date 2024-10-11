use crate::api::seasons::get::{get_season, get_seasons};
use axum::routing::get;
use axum::Router;

mod get;
mod errors;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_seasons))
        .route("/:id", get(get_season))
}