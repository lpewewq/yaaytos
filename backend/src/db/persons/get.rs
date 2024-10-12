use crate::db::persons::model::PersonDb;
use crate::state::AppState;
use uuid::Uuid;

pub fn list(state: &AppState) -> Vec<PersonDb> {
    state.persons.iter().cloned().collect()
}

pub fn by_uuid(state: &AppState, uuid: Uuid) -> Option<PersonDb> {
    state.persons.iter().find(|s| s.uuid == uuid).cloned()
}