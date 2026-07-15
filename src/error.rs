use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// Centralized application error type.
///
/// Every handler returns `Result<_, AppError>`, and Axum converts it
/// to an HTTP response via the `IntoResponse` impl.
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    BadRequest(String),
    Internal(String),
    Database(sqlx::Error),
    WebSocket(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::BadRequest(msg) => write!(f, "Bad request: {msg}"),
            AppError::Internal(msg) => write!(f, "Internal error: {msg}"),
            AppError::Database(err) => write!(f, "Database error: {err}"),
            AppError::WebSocket(msg) => write!(f, "WebSocket error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".into()),
            AppError::BadRequest(msg) | AppError::WebSocket(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".into(),
                )
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}
