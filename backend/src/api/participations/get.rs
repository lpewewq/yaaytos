use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Json;
use std::collections::HashMap;
use uuid::Uuid;
use yaaytos_common::{Gender, Participation, ParticipationType, Person};

pub async fn get_participations(Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<Vec<Participation>>) {
    let uuid: String = params.get("season_uuid")
        .map(|uuid_str| uuid_str.to_string())
        .unwrap_or(Uuid::new_v4().to_string());
    let participation = vec![
        Participation {
            season_uuid: uuid.to_string(),
            person: Person {
                uuid: uuid.clone(),
                name: "Person A".to_string(),
                ig_handle: Some("@personA".to_string()),
                gender: Gender::Male,
            },
            r#type: ParticipationType::Starter
        },
        Participation {
            season_uuid: uuid.to_string(),
            person: Person {
                uuid: uuid.clone(),
                name: "Person B".to_string(),
                ig_handle: Some("@personB".to_string()),
                gender: Gender::Female,
            },
            r#type: ParticipationType::Addition
        },
    ];
    (StatusCode::OK, Json(participation))
}
pub async fn get_participation(Path(uuid): Path<String>) -> (StatusCode, Json<Participation>) {
    let participation = Participation {
        season_uuid: uuid.to_string(),
        person: Person {
            uuid: uuid.clone(),
            name: "Person A".to_string(),
            ig_handle: Some("@personA".to_string()),
            gender: Gender::Male,
        },
        r#type: ParticipationType::Starter,
    };
    (StatusCode::OK, Json(participation))
}
