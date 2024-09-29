use axum::{
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Serialize;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with routes
    let app = Router::new()
        .route("/seasons", get(seasons));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Season {
    year: u16,
}

async fn seasons() -> (StatusCode, Json<Vec<Season>>) {
    let seasons = vec![Season { year: 2023 }, Season { year: 2024 }];

    (StatusCode::OK, Json(seasons))
}