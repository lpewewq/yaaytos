use crate::db::participations::model::ParticipationDb;
use crate::state::AppState;

pub fn list(state: &AppState) -> Vec<ParticipationDb> {
    state.participations.iter().cloned().collect()
}
