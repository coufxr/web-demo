use entity::prelude::{Account, OauthAccount};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, IntoActiveModel, QueryFilter};
use std::collections::HashMap;
use std::sync::LazyLock;
use uuid::Uuid;

use crate::apps::user::constants::ClassType;
use crate::constants::CONFIG;
use crate::helper::github;
use crate::helper::github::OAuthUserInfo;
use crate::project::configs::Oauth2Provider;
use crate::project::error::AppError;
use macros::with_transaction;

/// 缓存的 OAuth2 客户端（按 provider）
static OAUTH2_CLIENTS: LazyLock<HashMap<String, BasicClient>> = LazyLock::new(|| {
    CONFIG
        .oauth2
        .providers
        .iter()
        .filter_map(|(name, cfg)| {
            let auth_url = AuthUrl::new(cfg.auth_url.clone()).ok()?;
            let token_url = TokenUrl::new(cfg.token_url.clone()).ok()?;
            let redirect_url = RedirectUrl::new(cfg.redirect_url.clone()).ok()?;
            let client = BasicClient::new(
                ClientId::new(cfg.client_id.clone()),
                Some(ClientSecret::new(cfg.client_secret.clone())),
                auth_url,
                Some(token_url),
            )
            .set_redirect_uri(redirect_url);
            Some((name.clone(), client))
        })
        .collect()
});

/// 获取指定 provider 的配置
pub fn get_provider_config(provider: &str) -> Result<&Oauth2Provider, AppError> {
    CONFIG.oauth2.providers.get(provider).ok_or_else(|| {
        AppError::Api(
            axum::http::StatusCode::BAD_REQUEST,
            format!("不支持的 OAuth2 提供商: {}", provider),
        )
    })
}

/// 获取缓存的 OAuth2 客户端
pub fn get_oauth2_client(provider: &str) -> Result<&BasicClient, AppError> {
    get_provider_config(provider)?;
    OAUTH2_CLIENTS
        .get(provider)
        .ok_or_else(|| AppError::internal("OAuth2 客户端初始化失败"))
}

/// 生成 OAuth2 授权 URL
pub fn generate_auth_url(
    client: &BasicClient,
    provider: &str,
) -> Result<(url::Url, CsrfToken), AppError> {
    let cfg = get_provider_config(provider)?;
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(cfg.scopes.iter().map(|s| Scope::new(s.clone())))
        .url();

    Ok((auth_url, csrf_token))
}

/// 获取用户信息（按 provider 分发）
pub async fn fetch_user_info(
    access_token: &str,
    provider: &str,
) -> Result<OAuthUserInfo, AppError> {
    match provider {
        "github" => github::fetch_user(access_token).await,
        _ => Err(AppError::Api(
            axum::http::StatusCode::BAD_REQUEST,
            format!("不支持的 OAuth2 提供商: {}", provider),
        )),
    }
}

/// 查找或创建本地账户并绑定 OAuth 身份
#[with_transaction]
pub async fn link_or_create_account(
    db: &sea_orm::DbConn,
    provider: &str,
    user_info: &OAuthUserInfo,
) -> Result<entity::account::Model, AppError> {
    let existing_oauth = OauthAccount::Entity::find()
        .filter(OauthAccount::Column::Provider.eq(provider))
        .filter(OauthAccount::Column::ProviderUserId.eq(&user_info.provider_user_id))
        .one(db)
        .await?;

    let user = if let Some(oauth) = existing_oauth {
        let user = Account::Entity::find()
            .filter(Account::Column::Id.eq(oauth.account_id))
            .one(db)
            .await?
            .ok_or_else(|| AppError::internal("关联的本地账户不存在"))?;

        let mut account_active = user.clone().into_active_model();
        account_active.last_login_dt = Set(Some(chrono::Utc::now().naive_utc()));
        if let Some(email) = &user_info.email {
            account_active.email = Set(Some(email.clone()));
        }
        account_active.update(db).await?;

        let mut oauth_active = oauth.into_active_model();
        if let Some(email) = &user_info.email {
            oauth_active.email = Set(Some(email.clone()));
        }
        oauth_active.nickname = Set(Some(user_info.nickname.clone()));
        oauth_active.avatar_url = Set(user_info.avatar_url.clone());
        oauth_active.update(db).await?;

        // 重新查询获取最新数据
        Account::Entity::find()
            .filter(Account::Column::Id.eq(user.id))
            .one(db)
            .await?
            .ok_or_else(|| AppError::internal("关联的本地账户不存在"))?
    } else {
        let obj = Account::ActiveModel {
            uid: Set(Uuid::new_v4().to_string()),
            nickname: Set(user_info.nickname.clone()),
            password: Set(String::new()),
            name: Set(None),
            telephone: Set(None),
            email: Set(user_info.email.clone()),
            r#type: Set(ClassType::User as i16),
            last_login_dt: Set(Some(chrono::Utc::now().naive_utc())),
            ..Default::default()
        };

        let user = obj.insert(db).await.map_err(|e| {
            tracing::error!("账户创建失败: {}", e);
            AppError::internal("服务器内部错误")
        })?;

        let oauth_obj = OauthAccount::ActiveModel {
            account_id: Set(user.id),
            provider: Set(provider.to_string()),
            provider_user_id: Set(user_info.provider_user_id.clone()),
            email: Set(user_info.email.clone()),
            nickname: Set(Some(user_info.nickname.clone())),
            avatar_url: Set(user_info.avatar_url.clone()),
            ..Default::default()
        };

        oauth_obj.insert(db).await.map_err(|e| {
            tracing::error!("OAuth 绑定失败: {}", e);
            // 注：当前 entity 未声明 `(provider, provider_user_id)` 唯一索引，
            // 因此此处不会因唯一约束冲突而失败。所有 insert 错误均视为基础设施故障。
            AppError::internal("服务器内部错误")
        })?;

        user
    };

    Ok(user)
}
