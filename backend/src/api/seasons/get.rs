use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDate;
use uuid::Uuid;
use yaaytos_common::Season;

pub async fn get_seasons() -> (StatusCode, Json<Vec<Season>>) {
    let seasons = vec![
        Season { uuid: Uuid::new_v4().to_string(), number: 0, published: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(), is_vip: false, is_finished: false },
        Season { uuid: Uuid::new_v4().to_string(), number: 1, published: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), is_vip: true, is_finished: false },
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
