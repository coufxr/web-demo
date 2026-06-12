use axum::Json;
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

// 实现 IntoResponse trait，将 AppError 转换为统一 JSON 格式响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DB(e) => {
                // 数据库错误不暴露内部细节，仅记录日志
                error!("数据库错误: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "服务器内部错误".to_string(),
                )
            }
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, json!(e).to_string()),
            AppError::Api(status, msg) => (status, msg),
        };
        error!("err info: {}", error_message);
        let body = Json(json!({
            "code": status.as_u16(),
            "message": error_message,
            "data": null
        }));
        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<Json<T>, AppError>;

/// 成功响应的辅助函数
pub fn ok<T>(data: T) -> ApiResult<T> {
    Ok(Json(data))
}
