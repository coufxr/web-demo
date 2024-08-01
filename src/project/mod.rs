use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub mod configs;
pub mod db;
pub mod error;
pub mod logger;
pub mod middlewares;
pub mod response;

pub async fn fallback() -> Response {
    (StatusCode::NOT_FOUND, StatusCode::NOT_FOUND.to_string()).into_response()
}
