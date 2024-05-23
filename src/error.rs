use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

use crate::response::HttpException;

// 定义自定义错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    DBError(#[from] DbErr),
    #[error("Axum框架错误: {0}")]
    AxumError(#[from] axum::Error),
    #[error("Validate框架错误: {0}")]
    ValidatorError(ValidationErrors),
    #[error("自定义错误: {0}")]
    Other(String),
}

// 实现 IntoResponse trait，以便能够将 MyError 直接转换为响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DBError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::AxumError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::ValidatorError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, json!(e).to_string())
            }
            AppError::Other(e) => (StatusCode::BAD_REQUEST, e),
            // ... 其他匹配
        };
        error!("err info: {}", error_message);
        HttpException::new(status.as_u16(), error_message).into_response()
    }
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        AppError::ValidatorError(errors)
    }
}

pub type AppResult<T> = Result<T, AppError>;
