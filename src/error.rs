use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;
use thiserror::Error;
use tracing::error;

use crate::response::{EmptyStruct, JsonResponse};

// 定义自定义错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    DBError(#[from] DbErr),
    #[error("Axum框架错误: {0}")]
    AxumError(#[from] axum::Error),
    #[error("自定义错误: {0}")]
    Other(String),
}

// 实现 IntoResponse trait，以便能够将 MyError 直接转换为响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DBError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")),
            AppError::AxumError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")),
            AppError::Other(e) => (StatusCode::BAD_REQUEST, e),
            // ... 其他匹配
        };
        // 为什么需要在下方指定 <EmptyStruct> 明明结构体已经返回的是 EmptyStruct
        error!("err info: {}", error_message);
        let res = JsonResponse::<EmptyStruct>::error(status.as_u16(), error_message);
        res.into_response()
    }
}

pub type AppResult<T> = Result<JsonResponse<T>, AppError>;
