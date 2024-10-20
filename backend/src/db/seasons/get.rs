use crate::domain::models::season::SeasonModel;
use crate::state::AppState;
use uuid::Uuid;

pub fn list(state: &AppState) -> Vec<SeasonModel> {
    state.seasons.iter().cloned().map(SeasonModel::from).collect()
}

pub fn by_uuid(state: &AppState, uuid: Uuid) -> Option<SeasonModel> {
    state.seasons.iter().find(|&s| s.uuid == uuid).cloned().map(SeasonModel::from)
}