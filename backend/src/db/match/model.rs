use crate::domain::models::r#match::MatchModel;

#[derive(Clone)]
pub struct MatchDb {
    pub male_uuid: String,
    pub female_uuid: String,
}

impl From<MatchDb> for MatchModel {
    fn from(value: MatchDb) -> Self {
        MatchModel {
            male_uuid: value.male_uuid.clone(),
            female_uuid: value.female_uuid.clone(),
        }
    }
}