use yaaytos_common::{Participation, ParticipationType};

#[derive(Clone, Debug)]
pub struct ParticipationModel {
    pub season_uuid: String,
    pub person_uuid: String,
    pub r#type: ParticipationTypeModel,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParticipationTypeModel {
    Starter,
    Addition,
}

impl From<ParticipationModel> for Participation {
    fn from(value: ParticipationModel) -> Self {
        Participation {
            season_uuid: value.season_uuid.clone(),
            person_uuid: value.person_uuid.clone(),
            r#type: value.r#type.into(),
        }
    }
}


impl From<ParticipationTypeModel> for ParticipationType {
    fn from(value: ParticipationTypeModel) -> Self {
        match value {
            ParticipationTypeModel::Starter => { ParticipationType::Starter }
            ParticipationTypeModel::Addition => { ParticipationType::Addition }
        }
    }
}
