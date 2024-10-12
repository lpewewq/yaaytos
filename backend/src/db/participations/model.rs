use uuid::Uuid;
use yaaytos_common::{Participation, ParticipationType};

#[derive(Clone)]
pub struct ParticipationDb {
    pub person_uuid: Uuid,
    pub season_uuid: Uuid,
    pub is_starter: bool,
}

impl From<ParticipationDb> for Participation {
    fn from(value: ParticipationDb) -> Self {
        Participation {
            season_uuid: value.season_uuid.to_string(),
            person_uuid: value.person_uuid.to_string(),
            r#type: if value.is_starter { ParticipationType::Starter } else { ParticipationType::Addition },
        }
    }
}
