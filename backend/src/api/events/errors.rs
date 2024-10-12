use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use uuid::Uuid;

pub enum EventError {
    InvalidUuid(String),
    UuidNotFound(Uuid),
}

impl IntoResponse for EventError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            Self::InvalidUuid(invalid_uuid) => (
                StatusCode::BAD_REQUEST,
                format!("Event uuid {} is invalid", invalid_uuid),
            ),
            Self::UuidNotFound(uuid) => (
                StatusCode::NOT_FOUND,
                format!("Event with uuid {} not found", uuid),
            )
        };
        (status, Json(json!({"error": msg}))).into_response()
    }
}