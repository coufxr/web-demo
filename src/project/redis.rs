/// 检查并设置频率限制
///
/// - 如果 key 不存在，设置 key 并返回 Ok(())
/// - 如果 key 已存在，返回 Err(TOO_MANY_REQUESTS)
///
/// # Usage
/// ```rust
/// rate_limit_check!(state.redis, "sms:limit:13800138000", 60)?;
/// ```
#[macro_export]
macro_rules! rate_limit_check {
    ($redis:expr, $key:expr, $expire:expr) => {{
        use axum::http::StatusCode;
        use redis::AsyncCommands;
        let mut conn = $redis.clone();
        let was_set: bool = conn.set_nx($key, "1").await.map_err(|e| {
            $crate::project::error::AppError::Api(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("频率限制检查失败: {}", e),
            )
        })?;
        if !was_set {
            return Err($crate::project::error::AppError::Api(
                StatusCode::TOO_MANY_REQUESTS,
                "发送过于频繁，请稍后再试".to_string(),
            ));
        }
        let _: () = conn.expire($key, $expire as i64).await.map_err(|e| {
            $crate::project::error::AppError::Api(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("频率限制设置失败: {}", e),
            )
        })?;
    }};
}
