use axum::http::StatusCode;
use error::AppResult;
use response::JsonResponse;

pub mod configs;
pub mod db;
pub mod error;
pub mod logger;
pub mod middlewares;
pub mod response;

pub async fn fallback() -> AppResult<JsonResponse<()>> {
    Ok(JsonResponse::error(
        StatusCode::NOT_FOUND,
        StatusCode::NOT_FOUND.to_string(),
    ))
}
