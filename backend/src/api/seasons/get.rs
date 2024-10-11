use crate::api::seasons::errors::SeasonError;
use crate::db;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use yaaytos_common::Season;

pub async fn get_seasons() -> (StatusCode, Json<Vec<Season>>) {
    let seasons = db::seasons::get::list().into_iter().map(Season::from).collect();
    (StatusCode::OK, Json(seasons))
}

pub async fn get_season(Path(uuid): Path<String>) -> Result<Json<Season>, SeasonError> {
    let Ok(uuid) = uuid.parse() else {
        return Err(SeasonError::InvalidUuid(uuid))
    };
    let Some(season) = db::seasons::get::by_uuid(uuid) else {
        return Err(SeasonError::UuidNotFound(uuid))
    };
    Ok(Json(season.into()))
}
