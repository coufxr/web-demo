use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::project::error::AppError;

/// 共享的 HTTP 客户端（复用连接池）
static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent("web-demo-oauth2")
        .build()
        .expect("Failed to create HTTP client")
});

/// GitHub API GET 请求（自动追加 Authorization header 和错误处理）
async fn github_get<T: for<'a> Deserialize<'a>>(
    path: &str,
    access_token: &str,
) -> Result<T, AppError> {
    let response = HTTP_CLIENT
        .get(format!("https://api.github.com{}", path))
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("GitHub API 请求失败: {}", e);
            AppError::internal("服务器内部错误")
        })?;

    response
        .error_for_status()
        .map_err(|e| {
            if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) {
                tracing::error!("GitHub API 授权失效: {}", e);
                AppError::Api(
                    axum::http::StatusCode::UNAUTHORIZED,
                    "OAuth2 授权已失效".to_string(),
                )
            } else {
                tracing::error!("GitHub API 返回错误状态: {}", e);
                AppError::Api(
                    axum::http::StatusCode::BAD_GATEWAY,
                    "OAuth2 提供商服务异常".to_string(),
                )
            }
        })?
        .json()
        .await
        .map_err(|e| {
            tracing::error!("GitHub API 响应解析失败: {}", e);
            AppError::internal("服务器内部错误")
        })
}

/// GitHub 用户信息 API 响应
#[derive(Debug, Clone, Deserialize)]
struct GitHubUserInfo {
    pub id: u64,
    pub login: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub name: Option<String>,
}

/// GitHub 邮箱 API 响应
#[derive(Debug, Deserialize)]
struct GitHubEmail {
    pub email: String,
    pub primary: bool,
}

/// 通用 OAuth2 用户信息
#[derive(Debug, Clone, Serialize)]
pub struct OAuthUserInfo {
    pub provider: String,
    pub provider_user_id: String,
    pub email: Option<String>,
    pub nickname: String,
    pub avatar_url: Option<String>,
}

/// 获取 GitHub 用户基本信息
async fn fetch_profile(access_token: &str) -> Result<GitHubUserInfo, AppError> {
    github_get("/user", access_token).await
}

/// 获取 GitHub 主邮箱
async fn fetch_primary_email(access_token: &str) -> Result<Option<String>, AppError> {
    let emails: Vec<GitHubEmail> = github_get("/user/emails", access_token).await?;
    Ok(emails.into_iter().find(|e| e.primary).map(|e| e.email))
}

/// 从 GitHub API 获取用户信息
pub async fn fetch_user(access_token: &str) -> Result<OAuthUserInfo, AppError> {
    let info = fetch_profile(access_token).await?;

    let email = match info.email {
        Some(e) => Some(e),
        None => fetch_primary_email(access_token).await?,
    };

    Ok(OAuthUserInfo {
        provider: "github".to_string(),
        provider_user_id: info.id.to_string(),
        email,
        nickname: info.name.unwrap_or(info.login),
        avatar_url: info.avatar_url,
    })
}
