use axum::http::StatusCode;
use error::AppResult;
use response::HttpException;

pub mod configs;
pub mod db;
pub mod error;
pub mod logger;
pub mod response;

pub async fn fallback() -> AppResult<HttpException> {
    Ok(HttpException::new(
        StatusCode::NOT_FOUND.as_u16(),
        "Not Found".to_string(),
    ))
}
