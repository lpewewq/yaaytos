use crate::api::participations::get::{get_participation, get_participations};
use axum::routing::get;
use axum::Router;

mod get;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_participations))
        .route("/:id", get(get_participation))
}