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
    Upcoming,
    Ongoing,
    Completed,
}


impl From<SeasonModel> for Season {
    fn from(value: SeasonModel) -> Self {
        Season {
            uuid: value.uuid,
            number: value.number,
            is_vip: value.is_vip,
            status: value.status.into(),
        }
    }
}

impl From<SeasonStatusModel> for SeasonStatus {
    fn from(value: SeasonStatusModel) -> Self {
        match value {
            SeasonStatusModel::Upcoming => { SeasonStatus::UPCOMING }
            SeasonStatusModel::Ongoing => { SeasonStatus::ONGOING }
            SeasonStatusModel::Completed => { SeasonStatus::COMPLETED }
        }
    }
}