use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{Request, StatusCode, request::Parts},
    middleware::Next,
    response::Response,
};

use crate::{apps::auth::jwt, project::error::AppError};

/// 认证上下文，包含当前登录用户信息
#[derive(Clone)]
pub struct AuthContext {
    pub user_id: i32,
    #[allow(dead_code)]
    pub phone: String,
}

impl<S: Send + Sync> FromRequestParts<S> for AuthContext {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthContext>()
            .cloned()
            .ok_or_else(|| AppError::Api(StatusCode::UNAUTHORIZED, "缺少认证信息".to_string()))
    }
}

/// 公开路径（不需要鉴权）
const PUBLIC_PATHS: &[&str] = &[
    "/api/v1/auth/sms-code",
    "/api/v1/auth/register",
    "/api/v1/auth/login",
    "/api/v1/auth/refresh",
    "/docs",
    "/scalar",
    "/openapi.json",
];

/// 检查路径是否公开（不需要鉴权）
fn is_public_path(path: &str) -> bool {
    let path = path.trim_end_matches('/');
    PUBLIC_PATHS.contains(&path)
}

/// JWT 认证中间件
pub async fn auth_middleware(mut request: Request<Body>, next: Next) -> Result<Response, AppError> {
    let path = request.uri().path();

    // 公开路径直接放行
    if is_public_path(path) {
        return Ok(next.run(request).await);
    }

    // 从请求头获取 Authorization
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::Api(StatusCode::UNAUTHORIZED, "缺少认证信息".to_string()))?;

    // 提取 Bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Api(StatusCode::UNAUTHORIZED, "认证格式错误".to_string()))?;

    // 验证 token（jsonwebtoken 会自动验证 exp 过期时间）
    let claims = jwt::verify_access_token(token)
        .map_err(|e| AppError::Api(StatusCode::UNAUTHORIZED, format!("Token 无效: {}", e)))?;

    // 将用户信息插入请求扩展
    request.extensions_mut().insert(AuthContext {
        user_id: claims.user_id,
        phone: claims.phone,
    });

    Ok(next.run(request).await)
}
