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
    #[error("{1}")]
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
            AppError::Validation(e) => {
                tracing::warn!("输入验证失败: {}", e);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "code": 400,
                        "message": "输入验证失败",
                        "errors": e,
                        "data": null
                    })),
                )
                    .into_response();
            }
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

impl AppError {
    pub fn internal(msg: &str) -> Self {
        AppError::Api(StatusCode::INTERNAL_SERVER_ERROR, msg.to_string())
    }
}

pub type ApiResult<T> = Result<Json<T>, AppError>;

/// 成功响应的辅助函数
pub fn ok<T>(data: T) -> ApiResult<T> {
    Ok(Json(data))
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use super::*;

    #[test]
    fn internal_error_has_correct_status() {
        let err = AppError::internal("测试错误");
        match err {
            AppError::Api(StatusCode::INTERNAL_SERVER_ERROR, msg) => {
                assert_eq!(msg, "测试错误");
            }
            _ => panic!("internal() 应返回 Api(500, ..)"),
        }
    }

    #[test]
    fn db_error_display() {
        let err = AppError::DB(sea_orm::DbErr::Custom("db failure".to_string()));
        let msg = format!("{}", err);
        assert!(
            msg.contains("db failure"),
            "Display should include db error: {}",
            msg
        );
    }

    #[test]
    fn ok_helper_returns_ok() {
        let result: ApiResult<i32> = ok(42);
        assert!(result.is_ok());
        let Json(val) = result.unwrap();
        assert_eq!(val, 42);
    }

    #[test]
    fn api_error_display() {
        let err = AppError::Api(StatusCode::NOT_FOUND, "资源不存在".to_string());
        let msg = format!("{}", err);
        assert_eq!(msg, "资源不存在");
    }

    #[test]
    fn from_validation_errors() {
        use validator::Validate;
        #[derive(Validate)]
        struct TestInput {
            #[validate(required)]
            pub name: Option<String>,
        }
        let input = TestInput { name: None };
        let err = AppError::from(input.validate().unwrap_err());
        assert!(matches!(err, AppError::Validation(_)));
    }

    #[test]
    fn from_db_error() {
        let db_err = sea_orm::DbErr::Custom("conn failed".to_string());
        let err: AppError = db_err.into();
        assert!(matches!(err, AppError::DB(_)));
    }
}
