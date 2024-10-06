use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDate;
use uuid::Uuid;
use yaaytos_common::{Event, EventType, Gender, Match, Participation, ParticipationType, Person, Season};

pub async fn get_seasons() -> (StatusCode, Json<Vec<Season>>) {
    let seasons = vec![
        Season { uuid: Uuid::new_v4().to_string(), number: 0, published: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), is_vip: false, is_finished: false },
        Season { uuid: Uuid::new_v4().to_string(), number: 1, published: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), is_vip: true, is_finished: false },
    ];
    (StatusCode::OK, Json(seasons))
}

pub async fn get_season(Path(uuid): Path<String>) -> (StatusCode, Json<Season>) {
    let season = Season {
        uuid: uuid.to_string(),
        number: 0,
        published: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        is_vip: true,
        is_finished: false,
    };
    (StatusCode::OK, Json(season))
}

pub async fn get_season_participations(Path(uuid): Path<String>) -> (StatusCode, Json<Vec<Participation>>) {
    let participation = vec![
        Participation {
            season_uuid: uuid.to_string(),
            person: Person {
                uuid: Uuid::new_v4().to_string(),
                name: "Person A".to_string(),
                ig_handle: Some("@personA".to_string()),
                gender: Gender::Male,
            },
            r#type: ParticipationType::Starter
        },
        Participation {
            season_uuid: uuid.to_string(),
            person: Person {
                uuid: Uuid::new_v4().to_string(),
                name: "Person B".to_string(),
                ig_handle: Some("@personB".to_string()),
                gender: Gender::Female,
            },
            r#type: ParticipationType::Addition
        },
    ];
    (StatusCode::OK, Json(participation))
}

pub async fn get_season_events(Path(uuid): Path<String>) -> (StatusCode, Json<Vec<Event>>) {
    let events = vec![
        Event {
            uuid: Uuid::new_v4().to_string(),
            season_uuid: uuid.to_string(),
            order: 0,
            r#type: EventType::MatchBox {
                r#match: Match {
                    male: Person {
                        uuid: Uuid::new_v4().to_string(),
                        name: "Male".to_string(),
                        ig_handle: None,
                        gender: Gender::Male,
                    },
                    female: Person {
                        uuid: Uuid::new_v4().to_string(),
                        name: "Female".to_string(),
                        ig_handle: None,
                        gender: Gender::Female,
                    },
                    probability: None,
                },
                is_perfect: Some(true),
            },
            matching_probabilities: Default::default(),
        },
        Event {
            uuid: Uuid::new_v4().to_string(),
            season_uuid: uuid.to_string(),
            order: 1,
            r#type: EventType::MatchingNight {
                matching: vec![],
                probabilities: None,
                num_perfect: Some(3),
            },
            matching_probabilities: Default::default(),
        },
        Event {
            uuid: Uuid::new_v4().to_string(),
            season_uuid: uuid.to_string(),
            order: 2,
            r#type: EventType::NewPerson {
                participation: Participation {
                    season_uuid: uuid,
                    person: Person {
                        uuid: Uuid::new_v4().to_string(),
                        name: "New Person".to_string(),
                        ig_handle: None,
                        gender: Gender::Male,
                    },
                    r#type: ParticipationType::Addition,
                },
            },
            matching_probabilities: Default::default(),
        }
    ];
    (StatusCode::OK, Json(events))
}
