use axum::{
    extract::{FromRequestParts, Path},
    http::{StatusCode, request::Parts},
};

use super::error::AppError;

/// 路径参数 ID 提取器（u32，保证非负）
pub struct ResourceId(pub u32);

impl ResourceId {
    /// 安全转换为 i32（与 Entity 主键类型匹配），超出范围返回 400
    pub fn as_i32(&self) -> Result<i32, AppError> {
        i32::try_from(self.0)
            .map_err(|_| AppError::Api(StatusCode::BAD_REQUEST, "ID 超出范围".to_string()))
    }
}

impl<S: Send + Sync> FromRequestParts<S> for ResourceId {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(id): Path<u32> = Path::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Api(StatusCode::BAD_REQUEST, "无效的ID".to_string()))?;

        Ok(ResourceId(id))
    }
}
