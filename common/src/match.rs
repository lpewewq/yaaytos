use crate::Person;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub male: Person,
    pub female: Person
}
