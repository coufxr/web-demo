use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schemas::LoginOutput;
use crate::constants::CONFIG;
use crate::project::error::AppError;

/// JWT Claims 结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Token 唯一 ID（用于撤销）
    pub jti: String,
    /// 用户 ID
    pub sub: i32,
    /// 签发时间 (UTC timestamp)
    pub iat: i64,
    /// 过期时间 (UTC timestamp)
    pub exp: i64,
    /// Token 类型: "access" | "refresh"
    #[serde(rename = "type")]
    pub typ: String,
}

/// 创建 Access Token
pub fn create_access_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + Duration::hours(CONFIG.jwt.expires_in_hours as i64);
    let claims = Claims {
        jti: Uuid::new_v4().to_string(),
        sub: user_id,
        iat: now.timestamp(),
        exp: exp.timestamp(),
        typ: "access".into(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
    )
}

/// 创建 Refresh Token
pub fn create_refresh_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + Duration::days(CONFIG.jwt.refresh_expires_in_days as i64);
    let claims = Claims {
        jti: Uuid::new_v4().to_string(),
        sub: user_id,
        iat: now.timestamp(),
        exp: exp.timestamp(),
        typ: "refresh".into(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt.refresh_secret.as_bytes()),
    )
}

/// 验证 Access Token
pub fn verify_access_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
        &Validation::default(),
    )?
    .claims;
    if claims.typ != "access" {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ));
    }
    Ok(claims)
}

/// 验证 Refresh Token
pub fn verify_refresh_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(CONFIG.jwt.refresh_secret.as_bytes()),
        &Validation::default(),
    )?
    .claims;
    if claims.typ != "refresh" {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ));
    }
    Ok(claims)
}

/// 生成 JWT token 对（Access + Refresh）
pub fn generate_token_pair(user_id: i32) -> Result<LoginOutput, AppError> {
    let token = create_access_token(user_id).map_err(|e| {
        tracing::error!("AccessToken 生成失败: {}", e);
        AppError::internal("服务器内部错误")
    })?;
    let refresh_token = create_refresh_token(user_id).map_err(|e| {
        tracing::error!("RefreshToken 生成失败: {}", e);
        AppError::internal("服务器内部错误")
    })?;
    Ok(LoginOutput {
        token,
        expires_in: (CONFIG.jwt.expires_in_hours as i64) * 3600,
        refresh_token,
        refresh_expires_in: (CONFIG.jwt.refresh_expires_in_days as i64) * 86400,
    })
}
