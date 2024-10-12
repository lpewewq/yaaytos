use crate::db::events::model::EventDb;
use crate::db::participations::model::ParticipationDb;
use crate::db::persons::model::PersonDb;
use crate::db::seasons::model::SeasonDb;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub seasons: Vec<SeasonDb>,
    pub persons: Vec<PersonDb>,
    pub events: Vec<EventDb>,
    pub participations: Vec<ParticipationDb>,
}


impl Default for AppState {
    fn default() -> Self {
        let males: Vec<PersonDb> = vec![
            "Aleks",
            "Axel",
            "Dominic",
            "Elisha",
            "Ferhat",
            "Juliano",
            "Kevin",
            "Laurin",
            "Mo",
            "Rene"
        ].into_iter().map(PersonDb::create_male).collect();

        let females: Vec<PersonDb> = vec![
            "Aline",
            "Ivana",
            "Katharina",
            "Laura",
            "Luisa",
            "Madleine",
            "Melissa",
            "Michelle",
            "Nadine",
            "Sabrina",
        ].into_iter().map(PersonDb::create_female).collect();
        let persons = [&males[..], &females[..]].concat();

        let s1 = SeasonDb {
            uuid: Uuid::new_v4(),
            number: 1,
            is_vip: false,
        };

        AppState {
            participations: s1.add_persons(&persons),
            persons,
            seasons: vec![s1],
            events: vec![],
        }
    }
}