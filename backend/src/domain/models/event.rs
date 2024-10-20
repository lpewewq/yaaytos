use crate::domain::models::participation::ParticipationModel;
use crate::domain::models::r#match::MatchModel;
use chrono::NaiveDateTime;
use yaaytos_common::{Event, EventType};

#[derive(Clone, Debug)]
pub struct EventModel {
    pub uuid: String,
    pub season_uuid: String,
    pub created_timestamp: NaiveDateTime,
    pub r#type: EventTypeModel,
}


#[derive(Clone, Debug)]
pub enum EventTypeModel {
    Start,
    MatchBox {
        r#match: MatchModel,
        is_perfect: Option<bool>,
    },
    MatchingNight {
        matching: Vec<MatchModel>,
        num_perfect: Option<i32>,
    },
    NewPerson {
        participation: ParticipationModel,
    },
}

impl From<EventModel> for Event {
    fn from(value: EventModel) -> Self {
        Event {
            uuid: value.uuid,
            season_uuid: value.season_uuid.clone(),
            created_timestamp: value.created_timestamp,
            r#type: value.r#type.into(),
        }
    }
}

impl From<EventTypeModel> for EventType {
    fn from(value: EventTypeModel) -> Self {
        match value {
            EventTypeModel::Start => { EventType::Start }
            EventTypeModel::MatchBox { .. } => { EventType::Start }
            EventTypeModel::MatchingNight { .. } => { EventType::Start }
            EventTypeModel::NewPerson { .. } => { EventType::Start }
        }
    }
}