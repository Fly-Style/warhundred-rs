use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use axum_macros::FromRequest;
use serde_json::json;
use crate::error::{AppError, PlayerError};

// Define a custom extractor for JSON data
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]  // Derive the FromRequest trait with specific configuration
pub struct JsonExtractor<T>(pub T);


impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        // Convert the JsonRejection into a BodyParsingError with the rejection message
        AppError::BodyParsingError(rejection.to_string())
    }
}


// Implement the `IntoResponse` trait for the `AppError` enumeration
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
            ),
            Self::BodyParsingError(message) => (
                StatusCode::BAD_REQUEST,
                format!("Bad request error: {}", message),
            ),
        };

        (status, Json(json!({ "message": err_msg }))).into_response()
    }
}

impl IntoResponse for PlayerError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::CannotRegister(nick) => (
                StatusCode::BAD_REQUEST,
                format!("Cannot register player with nickname {}", nick),
            ),
            Self::NotFound(nick) => (
                StatusCode::NOT_FOUND,
                format!("Player with nickname {} has not been found", nick),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            Json(
                json!({
                    "message": err_msg,
                    "happened_at" : std::time::SystemTime::now()
                }),
            ),
        ).into_response()
    }
}