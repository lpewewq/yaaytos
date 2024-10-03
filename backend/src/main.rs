use axum::extract::Path;
use axum::{
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::NaiveDate;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use uuid::Uuid;
use yaaytos_common::Season;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    info!("starting app");

    let tracing_layer = TraceLayer::new_for_http();
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(AllowHeaders::any());

    // build our application with routes
    let app = Router::new()
        .route("/season", get(season))
        .route("/season/:uuid", get(season_by_uuid))
        .layer(tracing_layer)
        .layer(cors_layer);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn season() -> (StatusCode, Json<Vec<Season>>) {
    let seasons = vec![
        Season { uuid: Uuid::new_v4().to_string(), number: 0, published: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), is_vip: false },
        Season { uuid: Uuid::new_v4().to_string(), number: 1, published: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), is_vip: true },
    ];
    (StatusCode::OK, Json(seasons))
}

async fn season_by_uuid(Path(uuid): Path<String>) -> (StatusCode, Json<Season>) {
    let season = Season {
        uuid: uuid.to_string(),
        number: 0,
        published: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        is_vip: true,
    };
    (StatusCode::OK, Json(season))
}
