use ::oauth2::AuthorizationCode;
use ::oauth2::TokenResponse as _;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use entity::prelude::Account;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, IntoActiveModel, QueryFilter};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use super::jwt;
use super::oauth2::{self, fetch_user_info, generate_auth_url, get_oauth2_client};
use super::schemas::{LoginInput, LoginOutput, RefreshInput, RegisterInput, SendSmsCodeInput};
use crate::apps::user::constants::ClassType;
use crate::constants::AppState;
use crate::helper::crypto;
use crate::helper::sms;
use crate::project::error::{ApiResult, AppError, ok};
use crate::project::extractor::ValidatedJson;
use crate::project::redis;

/// 发送验证码
#[utoipa::path(
    post,
    path = "/auth/sms-code",
    tag = "认证",
    request_body = SendSmsCodeInput,
    security(()),
    responses(
        (status = 200),
        (status = 400, description = "Validation error")
    )
)]
pub async fn send_sms_code(
    State(state): State<AppState>,
    ValidatedJson(input): ValidatedJson<SendSmsCodeInput>,
) -> ApiResult<()> {
    sms::send_code(&state.redis, &input.phone).await?;
    ok(())
}

/// 用户注册（带验证码验证）
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "认证",
    request_body = RegisterInput,
    security(()),
    responses(
        (status = 200),
        (status = 400, description = "Validation error")
    )
)]
pub async fn register(
    State(state): State<AppState>,
    ValidatedJson(input): ValidatedJson<RegisterInput>,
) -> ApiResult<()> {
    // 校验验证码
    sms::verify_code(&state.redis, &input.phone, &input.code).await?;

    // 检查手机号是否已注册（先检查，避免浪费验证码）
    let existing_user = Account::Entity::find()
        .filter(Account::Column::Telephone.eq(input.phone.clone()))
        .one(&state.db)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::Api(
            StatusCode::BAD_REQUEST,
            "手机号已注册".to_string(),
        ));
    }

    // 密码哈希
    let hashed_password = crypto::hash_password(&input.password)?;

    // 创建用户
    let obj = Account::ActiveModel {
        uid: Set(Uuid::new_v4().to_string()),
        nickname: Set(input.nickname),
        password: Set(hashed_password),
        name: Set(input.name),
        telephone: Set(Some(input.phone.clone())),
        r#type: Set(ClassType::User as i16),
        ..Default::default()
    };

    obj.insert(&state.db).await?;

    ok(())
}

/// 用户登录
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "认证",
    request_body = LoginInput,
    security(()),
    responses(
        (status = 200, body = LoginOutput),
        (status = 400, description = "Validation error")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    ValidatedJson(input): ValidatedJson<LoginInput>,
) -> ApiResult<LoginOutput> {
    // 查找用户
    let user = Account::Entity::find()
        .filter(Account::Column::Telephone.eq(input.phone.clone()))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Api(StatusCode::UNAUTHORIZED, "手机号或密码错误".to_string()))?;

    // 验证密码（OAuth 用户密码为空，直接拒绝）
    if user.password.is_empty() {
        return Err(AppError::Api(
            StatusCode::UNAUTHORIZED,
            "手机号或密码错误".to_string(),
        ));
    }
    let password_valid = crypto::verify_password(&input.password, &user.password)?;

    if !password_valid {
        return Err(AppError::Api(
            StatusCode::UNAUTHORIZED,
            "手机号或密码错误".to_string(),
        ));
    }

    // 更新最后登录时间
    let mut active_model = user.clone().into_active_model();
    active_model.last_login_dt = Set(Some(chrono::Utc::now().naive_utc()));
    active_model.update(&state.db).await?;

    // 生成双 Token
    ok(jwt::generate_token_pair(user.id)?)
}

/// 刷新 Token
#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "认证",
    request_body = RefreshInput,
    security(()),
    responses(
        (status = 200, body = LoginOutput),
        (status = 401, description = "Refresh Token 无效")
    )
)]
pub async fn refresh(
    State(state): State<AppState>,
    ValidatedJson(input): ValidatedJson<RefreshInput>,
) -> ApiResult<LoginOutput> {
    // 验证 Refresh Token
    let claims = jwt::verify_refresh_token(&input.refresh_token).map_err(|_| {
        AppError::Api(
            StatusCode::UNAUTHORIZED,
            "Refresh Token 无效或已过期".to_string(),
        )
    })?;

    // 检查是否已被撤销（原子操作）
    let blacklist_key = format!("token:blacklist:{}", claims.jti);
    let ttl = claims.exp - chrono::Utc::now().timestamp();
    if ttl >= 0 {
        let was_set = redis::set_nx_with_expire(&state.redis, &blacklist_key, ttl + 1).await?;
        if !was_set {
            return Err(AppError::Api(
                StatusCode::UNAUTHORIZED,
                "Refresh Token 已失效".to_string(),
            ));
        }
    }

    // 查询用户确认存在
    let user = Account::Entity::find()
        .filter(Account::Column::Id.eq(claims.sub))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Api(StatusCode::UNAUTHORIZED, "用户不存在".to_string()))?;

    // 生成新双 Token
    ok(jwt::generate_token_pair(user.id)?)
}

/// OAuth2 回调参数
#[derive(Debug, Deserialize)]
pub struct OauthCallbackParams {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

/// CSRF token 前缀（Redis key）
const CSRF_TOKEN_PREFIX: &str = "oauth2:csrf:";
/// CSRF token 有效期（10分钟）
const CSRF_TOKEN_EXPIRE_SECONDS: u64 = 600;

/// OAuth2 登录 - 重定向到对应提供商授权页
#[utoipa::path(
    get,
    path = "/auth/oauth2/{provider}",
    tag = "认证",
    security(()),
    responses(
        (status = 302, description = "重定向到 OAuth2 授权页"),
        (status = 500, description = "内部错误")
    )
)]
pub async fn oauth2_login(
    State(state): State<AppState>,
    Path(provider): Path<String>,
) -> Result<Redirect, AppError> {
    let client = get_oauth2_client(&provider)?;
    let (auth_url, csrf_token) = generate_auth_url(client, &provider)?;

    // 将 csrf_token 存入 Redis，供回调时验证
    let csrf_key = format!("{}{}", CSRF_TOKEN_PREFIX, csrf_token.secret());
    redis::set_ex(
        &state.redis,
        &csrf_key,
        &provider,
        CSRF_TOKEN_EXPIRE_SECONDS,
    )
    .await?;

    Ok(Redirect::temporary(auth_url.as_str()))
}

/// OAuth2 回调 - 处理授权码，创建/关联用户，返回 JWT token
#[utoipa::path(
    get,
    path = "/auth/oauth2/{provider}/callback",
    tag = "认证",
    params(
        ("code" = String, Query, description = "授权码"),
        ("state" = Option<String>, Query, description = "CSRF token")
    ),
    security(()),
    responses(
        (status = 200, body = LoginOutput),
        (status = 400, description = "OAuth2 错误"),
        (status = 500, description = "内部错误")
    )
)]
pub async fn oauth2_callback(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    Query(params): Query<OauthCallbackParams>,
) -> ApiResult<LoginOutput> {
    // 0. 检查提供商是否返回错误
    if let Some(error) = params.error {
        let desc = params.error_description.unwrap_or_default();
        tracing::warn!("OAuth2 授权被拒绝 ({}): {} - {}", provider, error, desc);
        return Err(AppError::Api(
            StatusCode::BAD_REQUEST,
            "OAuth2 授权被拒绝".to_string(),
        ));
    }

    let code = params
        .code
        .ok_or_else(|| AppError::Api(StatusCode::BAD_REQUEST, "缺少授权码".to_string()))?;

    // 1. 验证 CSRF token（原子操作：获取并删除）
    let csrf_state = params
        .state
        .ok_or_else(|| AppError::Api(StatusCode::BAD_REQUEST, "无效的请求参数".to_string()))?;

    let csrf_key = format!("{}{}", CSRF_TOKEN_PREFIX, csrf_state);
    let stored_provider = redis::get_del(&state.redis, &csrf_key).await?;

    if stored_provider.is_none() {
        return Err(AppError::Api(
            StatusCode::BAD_REQUEST,
            "CSRF token 无效或已过期".to_string(),
        ));
    }
    if stored_provider.as_deref() != Some(&provider) {
        return Err(AppError::Api(
            StatusCode::BAD_REQUEST,
            "CSRF token 不匹配".to_string(),
        ));
    }

    // 2. 用 authorization code 换取 access token
    let client = get_oauth2_client(&provider)?;
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(::oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| {
            tracing::error!("OAuth2 token 交换失败 ({}): {}", provider, e);
            AppError::Api(StatusCode::BAD_REQUEST, "OAuth2 授权失败".to_string())
        })?;

    let access_token = token_result.access_token().secret();

    // 3. 用 access token 获取用户信息
    let user_info = fetch_user_info(access_token, &provider).await?;

    // 4. 查找或创建本地用户并绑定 OAuth 身份
    let user = oauth2::link_or_create_account(&state.db, &provider, &user_info).await?;

    // 5. 生成 JWT token
    let output = jwt::generate_token_pair(user.id)?;

    info!(
        "OAuth2 登录成功 -> 提供商: {}, 用户: {}",
        provider, user.nickname
    );

    ok(output)
}
