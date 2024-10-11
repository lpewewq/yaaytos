use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use yaaytos_common::{Event, EventType, Gender, Match, Participation, ParticipationType, Person};

pub async fn get_events() -> (StatusCode, Json<Vec<Event>>) {
    let events = vec![
        Event {
            uuid: Uuid::new_v4().to_string(),
            season_uuid: Uuid::new_v4().to_string(),
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
            season_uuid: Uuid::new_v4().to_string(),
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
            season_uuid: Uuid::new_v4().to_string(),
            order: 2,
            r#type: EventType::NewPerson {
                participation: Participation {
                    season_uuid: Uuid::new_v4().to_string(),
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
pub async fn get_event(Path(uuid): Path<String>) -> (StatusCode, Json<Event>) {
    let event = Event {
        uuid: Uuid::new_v4().to_string(),
        season_uuid: uuid,
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
    };
    (StatusCode::OK, Json(event))
}
