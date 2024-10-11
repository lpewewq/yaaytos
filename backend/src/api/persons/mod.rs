use crate::api::persons::get::{get_person, get_persons};
use axum::routing::get;
use axum::Router;

mod get;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_persons))
        .route("/:id", get(get_person))
}