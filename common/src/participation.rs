use crate::Person;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Participation {
    pub season_uuid: String,
    pub person: Person,
    pub r#type: ParticipationType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ParticipationType {
    Starter,
    Addition,
}
