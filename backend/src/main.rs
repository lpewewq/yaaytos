mod api;
mod db;
mod state;

use crate::api::{events, participations, persons, seasons};
use axum::Router;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    info!("starting app");

    let tracing_layer = TraceLayer::new_for_http();
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(AllowHeaders::any());

    let state = Default::default();
    let app = Router::new()
        .nest("/seasons", seasons::router(&state))
        .nest("/events", events::router(&state))
        .nest("/participants", participations::router(&state))
        .nest("/persons", persons::router(&state))
        .layer(tracing_layer)
        .layer(cors_layer);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
