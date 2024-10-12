use crate::r#match::Match;
use crate::Participation;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub uuid: String,
    pub season_uuid: String,
    pub created_timestamp: NaiveDateTime,
    pub r#type: EventType,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    Start,
    MatchBox {
        r#match: Match,
        is_perfect: Option<bool>,
    },
    MatchingNight {
        matching: Vec<Match>,
        num_perfect: Option<i32>,
    },
    NewPerson {
        participation: Participation,
    },
}
