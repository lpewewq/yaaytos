use crate::api::seasons::get::{get_season, get_season_events, get_season_participations, get_seasons};
use axum::routing::get;
use axum::Router;

mod get;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_seasons))
        .route("/:id", get(get_season))
        .route("/:id/participations", get(get_season_participations))
        .route("/:id/events", get(get_season_events))
}