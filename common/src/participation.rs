use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Participation {
    pub season_uuid: String,
    pub person_uuid: String,
    pub r#type: ParticipationType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ParticipationType {
    Starter,
    Addition,
}
