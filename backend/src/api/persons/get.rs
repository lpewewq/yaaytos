use crate::api::persons::errors::PersonError;
use crate::db;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use yaaytos_common::Person;

pub async fn get_persons(State(state): State<AppState>) -> (StatusCode, Json<Vec<Person>>) {
    let persons = db::persons::get::list(&state).into_iter().map(Person::from).collect();
    (StatusCode::OK, Json(persons))
}
pub async fn get_person(State(state): State<AppState>, Path(uuid): Path<String>) -> Result<Json<Person>, PersonError> {
    let Ok(uuid) = uuid.parse() else {
        return Err(PersonError::InvalidUuid(uuid))
    };
    let Some(person) = db::persons::get::by_uuid(&state, uuid) else {
        return Err(PersonError::UuidNotFound(uuid))
    };
    Ok(Json(person.into()))
}
