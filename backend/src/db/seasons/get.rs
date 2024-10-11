use crate::db::seasons::model::SeasonDb;
use crate::state::AppState;
use uuid::Uuid;

pub fn list(state: &AppState) -> Vec<SeasonDb> {
    state.seasons.iter().cloned().collect()
}

pub fn by_uuid(state: &AppState, uuid: Uuid) -> Option<SeasonDb> {
    state.seasons.iter().find(|s| s.uuid == uuid).cloned()
}