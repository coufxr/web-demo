use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JsonResponse<T: Serialize> {
    code: u16,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> JsonResponse<T> {
    pub fn new(code: u16, msg: String, data: Option<T>) -> Self {
        Self {
            code,
            message: msg,
            data,
        }
    }

    pub fn success(data: T) -> Self {
        Self::new(
            StatusCode::OK.as_u16(),
            StatusCode::OK.to_string(),
            Some(data),
        )
    }

    // pub fn error(code: StatusCode, message: String) -> Self {
    //     Self::new(code.as_u16(), message, None)
    // }
}

impl<T> IntoResponse for JsonResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
