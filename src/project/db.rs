use std::time::Duration;

use redis::RedisResult;
use redis::aio::ConnectionManager;
use sea_orm::{ConnectOptions, Database, DbConn};
use tracing::info;

use super::configs::{Database as cfg_database, Redis as cfg_redis};

/// 初始化数据库连接池并同步 Schema
pub async fn init(db: &cfg_database) -> Result<DbConn, sea_orm::DbErr> {
    let mut opt = ConnectOptions::new(db.url());
    opt.min_connections(2)
        .max_connections(10)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600))
        .sqlx_logging(db.debug);

    let conn = Database::connect(opt).await?;
    conn.ping().await?;

    // 自动同步 Entity 定义到数据库表结构
    conn.get_schema_registry("entity::*").sync(&conn).await?;

    info!("数据库初始化完成");
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
