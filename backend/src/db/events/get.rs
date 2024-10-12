use crate::db::events::model::EventDb;
use crate::state::AppState;
use uuid::Uuid;

pub fn list(state: &AppState) -> Vec<EventDb> {
    state.events.iter().cloned().collect()
}

pub fn by_uuid(state: &AppState, uuid: Uuid) -> Option<EventDb> {
    state.events.iter().find(|s| s.uuid == uuid).cloned()
}