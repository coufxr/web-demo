use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use utoipa::ToSchema;
use validator::Validate;

static PHONE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^1[3-9]\d{9}$").unwrap());

/// 发送验证码请求
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct SendSmsCodeInput {
    /// 手机号
    #[validate(regex(path = *PHONE_REGEX, message = "手机号格式不正确"))]
    pub phone: String,
}

/// 注册请求
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct RegisterInput {
    /// 手机号
    #[validate(regex(path = *PHONE_REGEX, message = "手机号格式不正确"))]
    pub phone: String,
    /// 验证码
    #[validate(length(equal = 6, message = "验证码必须为6位"))]
    pub code: String,
    /// 密码
    #[validate(length(min = 6, message = "密码长度不能少于6位"))]
    pub password: String,
    /// 昵称
    #[validate(length(min = 1, message = "昵称不能为空"))]
    pub nickname: String,
    /// 姓名（可选）
    pub name: Option<String>,
}

/// 登录请求
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct LoginInput {
    /// 手机号
    #[validate(regex(path = *PHONE_REGEX, message = "手机号格式不正确"))]
    pub phone: String,
    /// 密码
    #[validate(length(min = 6, max = 128, message = "密码长度必须在6-128位之间"))]
    pub password: String,
}

/// 登录响应
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LoginOutput {
    /// Access Token
    pub token: String,
    /// Access Token 过期时间（秒）
    pub expires_in: i64,
    /// Refresh Token
    pub refresh_token: String,
    /// Refresh Token 过期时间（秒）
    pub refresh_expires_in: i64,
}

/// 刷新 Token 请求
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct RefreshInput {
    /// Refresh Token
    pub refresh_token: String,
}
