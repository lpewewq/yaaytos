use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub male_uuid: String,
    pub female_uuid: String,
}
