use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;
use entity::prelude::Account;
use rand::Rng;
use redis::AsyncCommands;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, IntoActiveModel, QueryFilter};
use tracing::info;
use uuid::Uuid;
use validator::Validate;

use super::jwt;
use super::schemas::{LoginInput, LoginOutput, RefreshInput, RegisterInput, SendSmsCodeInput};
use crate::apps::user::constants::ClassType;
use crate::constants::{AppState, CONFIG};
use crate::project::error::{ApiResult, AppError, ok};
use crate::rate_limit_check;

/// 验证码前缀（Redis key）
const SMS_CODE_PREFIX: &str = "sms:code:";
/// 验证码有效期（5分钟）
const SMS_CODE_EXPIRE_SECONDS: u64 = 300;
/// 验证码发送频率限制前缀（Redis key）
const SMS_LIMIT_PREFIX: &str = "sms:limit:";
/// 验证码发送最小间隔（60秒）
const SMS_MIN_INTERVAL_SECONDS: u64 = 60;

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
    Json(input): Json<SendSmsCodeInput>,
) -> ApiResult<()> {
    input.validate()?;

    // 检查发送频率限制
    let limit_key = format!("{}{}", SMS_LIMIT_PREFIX, input.phone);
    rate_limit_check!(state.redis, &limit_key, SMS_MIN_INTERVAL_SECONDS);

    // 生成6位数字验证码
    let code: String = rand::thread_rng().gen_range(100000..=999999).to_string();

    // 存入验证码，设置过期时间
    let mut conn = state.redis.clone();
    let code_key = format!("{}{}", SMS_CODE_PREFIX, input.phone);
    let _: () = conn
        .set_ex(&code_key, &code, SMS_CODE_EXPIRE_SECONDS)
        .await
        .map_err(|e| {
            AppError::Api(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("验证码存储失败: {}", e),
            )
        })?;

    // 在日志中输出验证码（实际项目中应接入短信服务）
    info!(
        "📱 验证码已发送 -> 手机号: {}, 验证码: {}",
        input.phone, code
    );

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
    Json(input): Json<RegisterInput>,
) -> ApiResult<()> {
    input.validate()?;

    let key = format!("{}{}", SMS_CODE_PREFIX, input.phone);
    let mut conn = state.redis.clone();

    // 获取验证码
    let stored_code: Option<String> = conn.get(&key).await.map_err(|e| {
        AppError::Api(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("验证码获取失败: {}", e),
        )
    })?;

    // 验证码不存在（已过期或未发送）
    let stored_code = stored_code.ok_or_else(|| {
        AppError::Api(StatusCode::BAD_REQUEST, "验证码已过期或不存在".to_string())
    })?;

    // 验证码已被使用
    if stored_code.ends_with(":used") {
        return Err(AppError::Api(
            StatusCode::BAD_REQUEST,
            "验证码已被使用".to_string(),
        ));
    }

    // 验证码错误
    if stored_code != input.code {
        return Err(AppError::Api(
            StatusCode::BAD_REQUEST,
            "验证码错误".to_string(),
        ));
    }

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
    let hashed_password = bcrypt::hash(&input.password, bcrypt::DEFAULT_COST).map_err(|e| {
        AppError::Api(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("密码哈希失败: {}", e),
        )
    })?;

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

    // 标记验证码为已使用
    let used_code = format!("{}:used", input.code);
    let _: Result<(), _> = conn
        .set_ex(&key, &used_code, SMS_CODE_EXPIRE_SECONDS)
        .await
        .inspect_err(|e| tracing::error!("验证码状态更新失败: {}", e));

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
    Json(input): Json<LoginInput>,
) -> ApiResult<LoginOutput> {
    input.validate()?;

    // 查找用户
    let user = Account::Entity::find()
        .filter(Account::Column::Telephone.eq(input.phone.clone()))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Api(StatusCode::UNAUTHORIZED, "手机号或密码错误".to_string()))?;

    // 验证密码
    let password_valid = bcrypt::verify(&input.password, &user.password).map_err(|e| {
        AppError::Api(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("密码验证失败: {}", e),
        )
    })?;

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
    let token = jwt::create_access_token(user.id, &input.phone).map_err(|e| {
        AppError::Api(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Token 生成失败: {}", e),
        )
    })?;
    let refresh_token = jwt::create_refresh_token(user.id, &input.phone).map_err(|e| {
        AppError::Api(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Refresh Token 生成失败: {}", e),
        )
    })?;

    ok(LoginOutput {
        token,
        expires_in: (CONFIG.jwt.expires_in_hours as i64) * 3600,
        refresh_token,
        refresh_expires_in: (CONFIG.jwt.refresh_expires_in_days as i64) * 86400,
    })
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
    Json(input): Json<RefreshInput>,
) -> ApiResult<LoginOutput> {
    // 验证 Refresh Token
    let claims = jwt::verify_refresh_token(&input.refresh_token).map_err(|_| {
        AppError::Api(
            StatusCode::UNAUTHORIZED,
            "Refresh Token 无效或已过期".to_string(),
        )
    })?;

    // 查询用户确认存在
    let user = Account::Entity::find()
        .filter(Account::Column::Id.eq(claims.user_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Api(StatusCode::UNAUTHORIZED, "用户不存在".to_string()))?;

    // 生成新双 Token
    let token = jwt::create_access_token(user.id, &claims.phone).map_err(|e| {
        AppError::Api(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Token 生成失败: {}", e),
        )
    })?;
    let refresh_token = jwt::create_refresh_token(user.id, &claims.phone).map_err(|e| {
        AppError::Api(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Refresh Token 生成失败: {}", e),
        )
    })?;

    ok(LoginOutput {
        token,
        expires_in: (CONFIG.jwt.expires_in_hours as i64) * 3600,
        refresh_token,
        refresh_expires_in: (CONFIG.jwt.refresh_expires_in_days as i64) * 86400,
    })
}
