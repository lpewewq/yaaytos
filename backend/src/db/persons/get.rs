use crate::domain::models::person::PersonModel;
use crate::state::AppState;
use uuid::Uuid;

pub fn list(state: &AppState) -> Vec<PersonModel> {
    state.persons.iter().cloned().map(PersonModel::from).collect()
}

pub fn by_uuid(state: &AppState, uuid: Uuid) -> Option<PersonModel> {
    state.persons.iter().find(|s| s.uuid == uuid).cloned().map(PersonModel::from)
}