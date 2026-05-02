use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

// 定义自定义错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    DB(#[from] DbErr),
    #[error("验证错误: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("{0}")]
    Api(StatusCode, String),
}

// 实现 IntoResponse trait，以便能够将 MyError 直接转换为响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DB(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, json!(e).to_string()),
            AppError::Api(status, msg) => (status, msg),
        };
        error!("err info: {}", error_message);
        (status, error_message).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
