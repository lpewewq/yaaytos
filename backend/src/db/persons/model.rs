use uuid::Uuid;
use yaaytos_common::{Gender, Person};

#[derive(Clone)]
pub struct PersonDb {
    pub uuid: Uuid,
    pub name: String,
    pub is_male: bool,
}

impl From<PersonDb> for Person {
    fn from(value: PersonDb) -> Self {
        Person {
            uuid: value.uuid.to_string(),
            name: value.name.clone(),
            gender: if value.is_male { Gender::Male } else { Gender::Female },
        }
    }
}

impl PersonDb {
    pub fn create_male(name: &str) -> Self {
        PersonDb {
            uuid: Default::default(),
            name: name.to_string(),
            is_male: true,
        }
    }
    pub fn create_female(name: &str) -> Self {
        PersonDb {
            uuid: Default::default(),
            name: name.to_string(),
            is_male: false,
        }
    }
}
