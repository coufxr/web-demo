use axum::{
    extract::{FromRequestParts, Path},
    http::{StatusCode, request::Parts},
};

use super::error::AppError;

/// 路径参数 ID 提取器
/// - 自动解析为 u32（保证非负）
/// - 安全转换为 i32（与 Entity 主键类型匹配）
/// - 超出范围时返回 400 错误
pub struct ResourceId(pub i32);

impl<S: Send + Sync> FromRequestParts<S> for ResourceId {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(id): Path<u32> = Path::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Api(StatusCode::BAD_REQUEST, "无效的ID".to_string()))?;

        let id = i32::try_from(id)
            .map_err(|_| AppError::Api(StatusCode::BAD_REQUEST, "无效的ID".to_string()))?;

        Ok(ResourceId(id))
    }
}
