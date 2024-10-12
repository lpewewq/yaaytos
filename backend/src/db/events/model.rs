use crate::db::seasons::model::SeasonDb;
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;
use yaaytos_common::{Event, EventType, Match};

#[derive(Clone)]
pub struct EventDb {
    pub uuid: Uuid,
    pub season_uuid: Uuid,
    pub created_timestamp: NaiveDateTime,
    pub r#type: EventType,
}

impl From<EventDb> for Event {
    fn from(value: EventDb) -> Self {
        Event {
            uuid: value.uuid.to_string(),
            season_uuid: value.season_uuid.to_string(),
            created_timestamp: value.created_timestamp.clone(),
            r#type: value.r#type.clone(),
        }
    }
}

impl EventDb {
    pub fn create_match_box(season: &SeasonDb, r#match: Match, is_perfect: Option<bool>) -> EventDb {
        EventDb {
            uuid: Uuid::new_v4(),
            season_uuid: season.uuid.clone(),
            created_timestamp: Utc::now().naive_utc(),
            r#type: EventType::MatchBox {
                r#match,
                is_perfect,
            },
        }
    }
}
