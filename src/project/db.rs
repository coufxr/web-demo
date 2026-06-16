use std::time::Duration;

use redis::RedisResult;
use redis::aio::ConnectionManager;
use sea_orm::{ConnectOptions, Database, DbConn};
use tracing::info;

use super::configs::{Database as cfg_database, Redis as cfg_redis};

/// 初始化数据库连接池
pub async fn init(db: &cfg_database) -> Result<DbConn, sea_orm::DbErr> {
    // 创建连接选项
    let mut opt = ConnectOptions::new(db.url());
    opt.min_connections(2) // 最少保持的连接数
        .max_connections(10) // 最多允许的连接数
        .connect_timeout(Duration::from_secs(10)) // 连接超时时间
        .idle_timeout(Duration::from_secs(300)) // 连接空闲超时时间
        .max_lifetime(Duration::from_secs(3600)) // 连接最大生命周期
        .sqlx_logging(db.debug); // 是否打印 SQL 日志

    // 建立连接
    let conn = Database::connect(opt).await?;

    // 测试连接
    conn.ping().await?;

    info!("数据库连接池初始化完成");
    Ok(conn)
}

/// 初始化 Redis 连接
pub async fn init_redis(redis: &cfg_redis) -> RedisResult<ConnectionManager> {
    // 创建客户端（内部管理连接池）
    let client = redis::Client::open(redis.url())?;

    // 创建连接管理器（自动管理连接池）
    let conn = ConnectionManager::new(client).await?;

    info!("Redis 连接初始化完成");
    Ok(conn)
}
