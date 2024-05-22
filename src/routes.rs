use axum::{Router, routing::get};
use axum::http::StatusCode;

use crate::app::user::user_routes;
use crate::error::AppResult;
use crate::response::{HttpException, JsonResponse};

pub async fn fallback() -> AppResult<HttpException> {
    Ok(HttpException::new(
        StatusCode::NOT_FOUND.as_u16(),
        "Not Found".to_string(),
    ))
}

async fn handler() -> AppResult<JsonResponse<String>> {
    Ok(JsonResponse::new("hello, user".to_string()))
}

pub fn info_routes() -> Router {
    Router::new().route("/", get(handler))
}

pub fn v1_routes() -> Router {
    Router::new()
        .nest("/user", user_routes())
        .nest("/info", info_routes())
}
