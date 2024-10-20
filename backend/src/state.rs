use crate::db::events::model::EventDb;
use crate::db::participations::model::ParticipationDb;
use crate::db::persons::model::PersonDb;
use crate::db::r#match::model::MatchDb;
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
            "Rene",
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
        let starter_persons = [&males[..], &females[..]].concat();
        let additional_persons = vec![PersonDb::create_male("Edin")];
        let persons = [&starter_persons[..], &additional_persons[..]].concat();

        let get_uuid = |name: &str, persons: &Vec<PersonDb>| {
            persons.into_iter().find(|p| p.name.eq(name)).map(|p| p.uuid)
        };
        let create_match = |male_name: &str, female_name: &str, persons: &Vec<PersonDb>| {
            MatchDb {
                male_uuid: get_uuid(male_name, persons).unwrap().to_string(),
                female_uuid: get_uuid(female_name, persons).unwrap().to_string(),
            }
        };

        let s1 = SeasonDb {
            uuid: Uuid::new_v4(),
            number: 1,
            is_vip: false,
        };

        let events = vec![
            EventDb::create_match_box(&s1, create_match("Mo", "Ivana", &persons), false),
            EventDb::create_match_box(&s1, create_match("Dominic", "Melissa", &persons), false),
            EventDb::create_match_box(&s1, create_match("Mo", "Aline", &persons), true),
            EventDb::create_match_box(&s1, create_match("Laurin", "Melissa", &persons), true),
            EventDb::create_match_box(&s1, create_match("Elisha", "Ivana", &persons), false),
            EventDb::create_match_box(&s1, create_match("Rene", "Michelle", &persons), true),
            EventDb::create_match_box(&s1, create_match("Edin", "Ivana", &persons), false),
            EventDb::create_match_box(&s1, create_match("Axel", "Sabrina", &persons), false),
            EventDb::create_match_box(&s1, create_match("Ferhat", "Luisa", &persons), false),
            EventDb::create_match_box(&s1, create_match("Kevin", "Katharina", &persons), true),
        ];

        AppState {
            participations: [&s1.create_starter_participations(&starter_persons)[..], &s1.create_additional_participations(&additional_persons)[..]].concat(),
            persons,
            seasons: vec![s1],
            events,
        }
    }
}