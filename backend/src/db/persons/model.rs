use crate::domain::models::person::{GenderModel, PersonModel};
use uuid::Uuid;

#[derive(Clone)]
pub struct PersonDb {
    pub uuid: Uuid,
    pub name: String,
    pub is_male: bool,
}

impl From<PersonDb> for PersonModel {
    fn from(value: PersonDb) -> Self {
        PersonModel {
            uuid: value.uuid.to_string(),
            name: value.name.clone(),
            gender: if value.is_male { GenderModel::Male } else { GenderModel::Female },
        }
    }
}

impl PersonDb {
    pub fn create_male(name: &str) -> Self {
        PersonDb {
            uuid: Uuid::new_v4(),
            name: name.to_string(),
            is_male: true,
        }
    }
    pub fn create_female(name: &str) -> Self {
        PersonDb {
            uuid: Uuid::new_v4(),
            name: name.to_string(),
            is_male: false,
        }
    }
}
