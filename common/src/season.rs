use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Season {
    pub uuid: String,
    pub number: i32,
    pub published: NaiveDate,
    pub is_vip: bool,
    pub is_finished: bool,
}
