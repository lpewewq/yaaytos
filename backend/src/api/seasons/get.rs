use crate::api::seasons::errors::SeasonError;
use crate::db;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use yaaytos_common::Season;

pub async fn get_seasons(State(state): State<AppState>) -> (StatusCode, Json<Vec<Season>>) {
    let seasons = db::seasons::get::list(&state).into_iter().map(Season::from).collect();
    (StatusCode::OK, Json(seasons))
}

pub async fn get_season(State(state): State<AppState>, Path(uuid): Path<String>) -> Result<Json<Season>, SeasonError> {
    let Ok(uuid) = uuid.parse() else {
        return Err(SeasonError::InvalidUuid(uuid))
    };
    let Some(season) = db::seasons::get::by_uuid(&state, uuid) else {
        return Err(SeasonError::UuidNotFound(uuid))
    };
    Ok(Json(season.into()))
}
