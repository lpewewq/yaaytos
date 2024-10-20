use yaaytos_common::{Season, SeasonStatus};

#[derive(Clone, Debug)]
pub struct SeasonModel {
    pub uuid: String,
    pub number: i32,
    pub is_vip: bool,
    pub status: SeasonStatusModel,
}

#[derive(Clone, Debug)]
pub enum SeasonStatusModel {
    UPCOMING,
    ONGOING,
    COMPLETED,
}


impl From<SeasonModel> for Season {
    fn from(value: SeasonModel) -> Self {
        Season {
            uuid: value.uuid.clone(),
            number: value.number,
            is_vip: value.is_vip,
            status: value.status.into(),
        }
    }
}

impl From<SeasonStatusModel> for SeasonStatus {
    fn from(value: SeasonStatusModel) -> Self {
        match value {
            SeasonStatusModel::UPCOMING => { SeasonStatus::UPCOMING }
            SeasonStatusModel::ONGOING => { SeasonStatus::ONGOING }
            SeasonStatusModel::COMPLETED => { SeasonStatus::COMPLETED }
        }
    }
}