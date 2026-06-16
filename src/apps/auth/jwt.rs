use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::constants::CONFIG;

/// JWT Claims 结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// 用户 ID
    pub user_id: i32,
    /// 手机号
    pub phone: String,
    /// Token 类型: "access" | "refresh"
    #[serde(rename = "type")]
    pub typ: String,
    /// 过期时间 (UTC timestamp)
    pub exp: i64,
}

/// 创建 Access Token
pub fn create_access_token(
    user_id: i32,
    phone: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now() + Duration::hours(CONFIG.jwt.expires_in_hours as i64);
    let claims = Claims {
        user_id,
        phone: phone.to_string(),
        typ: "access".into(),
        exp: exp.timestamp(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
    )
}

/// 创建 Refresh Token
pub fn create_refresh_token(
    user_id: i32,
    phone: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now() + Duration::days(CONFIG.jwt.refresh_expires_in_days as i64);
    let claims = Claims {
        user_id,
        phone: phone.to_string(),
        typ: "refresh".into(),
        exp: exp.timestamp(),
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
