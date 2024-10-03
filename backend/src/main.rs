use axum::{
    http::StatusCode,
    routing::get,
    Json, Router,
};
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{debug, info};

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
        .route("/seasons", get(seasons))
        .layer(tracing_layer)
        .layer(cors_layer);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn seasons() -> (StatusCode, Json<Vec<Season>>) {
    let seasons = vec![Season { year: 2023 }, Season { year: 2024 }];
    debug!("seasons {:?}", seasons);
    (StatusCode::OK, Json(seasons))
}