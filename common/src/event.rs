use crate::Person;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub uuid: String,
    pub season_uuid: String,
    pub order: usize,
    pub r#type: EventType,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    MatchBox {
        is_perfect: Option<bool>,
    },
    MatchingNight {
        num_perfect: Option<usize>,
    },
    NewPerson {
        person: Person,
    },
}
