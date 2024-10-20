use yaaytos_common::{Gender, Person};

#[derive(Clone, Debug)]
pub struct PersonModel {
    pub uuid: String,
    pub name: String,
    pub gender: GenderModel,
}

#[derive(Clone, Debug)]
pub enum GenderModel {
    Male,
    Female,
}

impl From<PersonModel> for Person {
    fn from(value: PersonModel) -> Self {
        Person {
            uuid: value.uuid.clone(),
            name: value.name.clone(),
            gender: value.gender.into(),
        }
    }
}

impl From<GenderModel> for Gender {
    fn from(value: GenderModel) -> Self {
        match value {
            GenderModel::Male => { Gender::Male }
            GenderModel::Female => { Gender::Female }
        }
    }
}