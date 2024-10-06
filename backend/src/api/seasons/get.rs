use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDate;
use uuid::Uuid;
use yaaytos_common::{Gender, Participation, ParticipationType, Person, Season};

pub async fn get_seasons() -> (StatusCode, Json<Vec<Season>>) {
    let seasons = vec![
        Season { uuid: Uuid::new_v4().to_string(), number: 0, published: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), is_vip: false },
        Season { uuid: Uuid::new_v4().to_string(), number: 1, published: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), is_vip: true },
    ];
    (StatusCode::OK, Json(seasons))
}

pub async fn get_season(Path(uuid): Path<String>) -> (StatusCode, Json<Season>) {
    let season = Season {
        uuid: uuid.to_string(),
        number: 0,
        published: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        is_vip: true,
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
