use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use yaaytos_common::{Gender, Person};

pub async fn get_persons() -> (StatusCode, Json<Vec<Person>>) {
    let persons = vec![
        Person {
            uuid: Uuid::new_v4().to_string(),
            name: "Person A".to_string(),
            ig_handle: Some("@personA".to_string()),
            gender: Gender::Male,
        }, Person {
            uuid: Uuid::new_v4().to_string(),
            name: "Person B".to_string(),
            ig_handle: Some("@personB".to_string()),
            gender: Gender::Female,
        },
    ];
    (StatusCode::OK, Json(persons))
}
pub async fn get_person(Path(uuid): Path<String>) -> (StatusCode, Json<Person>) {
    let person = Person {
        uuid: uuid.clone(),
        name: "Person A".to_string(),
        ig_handle: Some("@personA".to_string()),
        gender: Gender::Male,
    };
    (StatusCode::OK, Json(person))
}
