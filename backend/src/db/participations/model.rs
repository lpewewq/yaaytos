use crate::domain::models::participation::{ParticipationModel, ParticipationTypeModel};
use uuid::Uuid;

#[derive(Clone)]
pub struct ParticipationDb {
    pub person_uuid: Uuid,
    pub season_uuid: Uuid,
    pub is_starter: bool,
}

impl From<ParticipationDb> for ParticipationModel {
    fn from(value: ParticipationDb) -> Self {
        ParticipationModel {
            season_uuid: value.season_uuid.to_string(),
            person_uuid: value.person_uuid.to_string(),
            r#type: if value.is_starter { ParticipationTypeModel::Starter } else { ParticipationTypeModel::Addition },
        }
    }
}
