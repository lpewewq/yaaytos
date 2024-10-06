use crate::r#match::Match;
use crate::Participation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub uuid: String,
    pub season_uuid: String,
    pub order: usize,
    pub r#type: EventType,
    pub matching_probabilities: HashMap<String, HashMap<String, f32>>,
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
        probabilities: Option<Vec<f32>>,
        num_perfect: Option<usize>,
    },
    NewPerson {
        participation: Participation,
    },
}
