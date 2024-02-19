use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct EmptyStruct {}

#[derive(Debug, Serialize)]
pub struct JsonResponse<T: Serialize> {
    code: u16,
    message: String,
    data: Option<T>,
}

impl<T> JsonResponse<T>
where
    T: Serialize,
{
    pub fn new(code: u16, msg: String, data: Option<T>) -> Self {
        Self {
            code,
            message: msg,
            data,
        }
    }
    pub fn success(data: T) -> Self {
        Self::new(0, "OK".to_string(), Some(data))
    }
    pub fn error(code: u16, msg: String) -> JsonResponse<EmptyStruct> {
        // 为什么Self 无法设置<>泛型
        JsonResponse::new(code, msg, Some(EmptyStruct::default()))
    }
}

impl<T> IntoResponse for JsonResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let res = Json(self);
        res.into_response()
    }
}
