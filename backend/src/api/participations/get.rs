use crate::db;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use yaaytos_common::Participation;

pub async fn get_participations(State(state): State<AppState>) -> (StatusCode, Json<Vec<Participation>>) {
    let participations = db::participations::get::list(&state).into_iter().map(Participation::from).collect();
    (StatusCode::OK, Json(participations))
}

