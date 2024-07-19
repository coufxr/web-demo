use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JsonResponse<T: Serialize> {
    code: u16,
    message: String,
    data: T,
}

impl<T: Serialize> JsonResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            message: StatusCode::OK.to_string(),
            data,
        }
    }
}

impl<T> IntoResponse for JsonResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct HttpException {
    code: u16,
    message: String,
}

impl HttpException {
    pub fn new(code: u16, message: String) -> Self {
        Self { code, message }
    }
}

impl IntoResponse for HttpException {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
