use crate::db::participations::model::ParticipationDb;
use crate::db::persons::model::PersonDb;
use crate::domain::models::season::{SeasonModel, SeasonStatusModel};
use uuid::Uuid;

#[derive(Clone)]
pub struct SeasonDb {
    pub uuid: Uuid,
    pub number: i32,
    pub is_vip: bool,
}

impl From<SeasonDb> for SeasonModel {
    fn from(value: SeasonDb) -> Self {
        SeasonModel {
            uuid: value.uuid.to_string(),
            number: value.number,
            is_vip: value.is_vip,
            status: SeasonStatusModel::Completed,
        }
    }
}

impl SeasonDb {
    pub fn create_starter_participations(&self, persons: &[PersonDb]) -> Vec<ParticipationDb> {
        persons.iter()
            .map(|person|
                ParticipationDb {
                    season_uuid: self.uuid,
                    person_uuid: person.uuid,
                    is_starter: true,
                }
            ).collect()
    }

    pub fn create_additional_participations(&self, persons: &[PersonDb]) -> Vec<ParticipationDb> {
        persons.iter()
            .map(|person|
                ParticipationDb {
                    season_uuid: self.uuid,
                    person_uuid: person.uuid,
                    is_starter: false,
                }
            ).collect()
    }
}
