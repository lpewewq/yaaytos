use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub uuid: String,
    pub name: String,
    pub gender: Gender,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}
