use crate::db::participations::model::ParticipationDb;
use crate::db::persons::model::PersonDb;
use uuid::Uuid;
use yaaytos_common::{Season, SeasonStatus};

#[derive(Clone)]
pub struct SeasonDb {
    pub uuid: Uuid,
    pub number: i32,
    pub is_vip: bool,
}

impl From<SeasonDb> for Season {
    fn from(value: SeasonDb) -> Self {
        Season {
            uuid: value.uuid.to_string(),
            number: value.number,
            is_vip: value.is_vip,
            status: SeasonStatus::COMPLETED,
        }
    }
}


impl SeasonDb {
    pub fn create_starter_participations(&self, persons: &Vec<PersonDb>) -> Vec<ParticipationDb> {
        persons.into_iter()
            .map(|person|
                ParticipationDb {
                    season_uuid: self.uuid.clone(),
                    person_uuid: person.uuid.clone(),
                    is_starter: true,
                }
            ).collect()
    }

    pub fn create_additional_participations(&self, persons: &Vec<PersonDb>) -> Vec<ParticipationDb> {
        persons.into_iter()
            .map(|person|
                ParticipationDb {
                    season_uuid: self.uuid.clone(),
                    person_uuid: person.uuid.clone(),
                    is_starter: false,
                }
            ).collect()
    }
}
