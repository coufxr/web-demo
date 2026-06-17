use crate::project::error::AppError;

/// Redis SETEX 辅助函数
pub async fn set_ex(
    redis: &redis::aio::ConnectionManager,
    key: &str,
    value: &str,
    ttl: u64,
) -> Result<(), AppError> {
    use redis::AsyncCommands;
    let mut conn = redis.clone();
    conn.set_ex(key, value, ttl).await.map_err(|e| {
        tracing::error!("Redis SETEX 失败 ({}): {}", key, e);
        AppError::internal("服务器内部错误")
    })
}

/// Redis GET 辅助函数
#[allow(dead_code)]
pub async fn get(
    redis: &redis::aio::ConnectionManager,
    key: &str,
) -> Result<Option<String>, AppError> {
    use redis::AsyncCommands;
    let mut conn = redis.clone();
    conn.get(key).await.map_err(|e| {
        tracing::error!("Redis GET 失败 ({}): {}", key, e);
        AppError::internal("服务器内部错误")
    })
}

/// Redis GETDEL 辅助函数
pub async fn get_del(
    redis: &redis::aio::ConnectionManager,
    key: &str,
) -> Result<Option<String>, AppError> {
    use redis::AsyncCommands;
    let mut conn = redis.clone();
    conn.get_del(key).await.map_err(|e| {
        tracing::error!("Redis GETDEL 失败 ({}): {}", key, e);
        AppError::internal("服务器内部错误")
    })
}

/// Redis SET NX EX（原子频率限制）
/// 返回 true 表示首次设置（通过），false 表示已存在（频率限制）
pub async fn set_nx_with_expire(
    redis: &redis::aio::ConnectionManager,
    key: &str,
    ttl: i64,
) -> Result<bool, AppError> {
    let mut conn = redis.clone();
    let result: Option<String> = redis::cmd("SET")
        .arg(key)
        .arg("1")
        .arg("NX")
        .arg("EX")
        .arg(ttl)
        .query_async(&mut conn)
        .await
        .map_err(|e| {
            tracing::error!("Redis SET NX EX 失败 ({}): {}", key, e);
            AppError::internal("服务器内部错误")
        })?;
    Ok(result.is_some())
}
