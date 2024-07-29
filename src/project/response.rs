use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JsonResponse<T: Serialize> {
    code: u16,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> JsonResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            message: StatusCode::OK.to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: StatusCode, message: String) -> Self {
        Self {
            code: code.as_u16(),
            message,
            data: None,
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
