use crate::domain::models::event::EventModel;
use crate::state::AppState;
use uuid::Uuid;

pub fn list(state: &AppState) -> Vec<EventModel> {
    state.events.iter().cloned().map(EventModel::from).collect()
}

pub fn by_uuid(state: &AppState, uuid: Uuid) -> Option<EventModel> {
    state.events.iter().find(|s| s.uuid == uuid).cloned().map(EventModel::from)
}
