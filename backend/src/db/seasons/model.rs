use chrono::NaiveDate;
use uuid::Uuid;
use yaaytos_common::{Season, SeasonStatus};

#[derive(Clone)]
pub struct SeasonDb {
    pub uuid: Uuid,
    pub number: i32,
    pub published: NaiveDate,
    pub is_vip: bool,
}

impl From<SeasonDb> for Season {
    fn from(value: SeasonDb) -> Self {
        Season {
            uuid: value.uuid.to_string(),
            number: value.number,
            published: value.published,
            is_vip: value.is_vip,
            status: SeasonStatus::UPCOMING,
        }
    }
}

