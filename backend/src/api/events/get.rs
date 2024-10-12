use crate::api::events::errors::EventError;
use crate::db;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use yaaytos_common::Event;

pub async fn get_events(State(state): State<AppState>) -> (StatusCode, Json<Vec<Event>>) {
    let events = db::events::get::list(&state).into_iter().map(Event::from).collect();
    (StatusCode::OK, Json(events))
}
pub async fn get_event(State(state): State<AppState>, Path(uuid): Path<String>) -> Result<Json<Event>, EventError> {
    let Ok(uuid) = uuid.parse() else {
        return Err(EventError::InvalidUuid(uuid))
    };
    let Some(event) = db::events::get::by_uuid(&state, uuid) else {
        return Err(EventError::UuidNotFound(uuid))
    };
    Ok(Json(event.into()))
}
