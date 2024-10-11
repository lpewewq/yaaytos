use crate::db::seasons::model::SeasonDb;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub seasons: Vec<SeasonDb>,
}

impl Default for AppState {
    fn default() -> Self {
        let seasons = vec![
            SeasonDb {
                uuid: Uuid::new_v4(),
                number: 1,
                published: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                is_vip: false,
            },
            SeasonDb {
                uuid: Uuid::new_v4(),
                number: 2,
                published: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
                is_vip: false,
            }
        ];
        AppState { seasons }
    }
}