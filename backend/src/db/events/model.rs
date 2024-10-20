use crate::db::r#match::model::MatchDb;
use crate::db::seasons::model::SeasonDb;
use crate::domain::models::event::{EventModel, EventTypeModel};
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct EventDb {
    pub uuid: Uuid,
    pub season_uuid: Uuid,
    pub created_timestamp: NaiveDateTime,
    pub r#match: MatchDb,
    pub is_perfect: bool,
}

impl From<EventDb> for EventModel {
    fn from(value: EventDb) -> Self {
        EventModel {
            uuid: value.uuid.to_string(),
            season_uuid: value.season_uuid.to_string(),
            created_timestamp: value.created_timestamp.clone(),
            r#type: EventTypeModel::Start,
        }
    }
}


impl EventDb {
    pub fn create_match_box(season: &SeasonDb, r#match: MatchDb, is_perfect: bool) -> EventDb {
        EventDb {
            uuid: Uuid::new_v4(),
            season_uuid: season.uuid.clone(),
            created_timestamp: Utc::now().naive_utc(),
            r#match,
            is_perfect,
        }
    }
}
