use crate::domain::models::participation::ParticipationModel;
use crate::state::AppState;

pub fn list(state: &AppState) -> Vec<ParticipationModel> {
    state.participations.iter().cloned().map(ParticipationModel::from).collect()
}
