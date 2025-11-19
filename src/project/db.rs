use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DbConn};
use tracing::info;

use super::configs::Database as cfg_database;

pub async fn init(db: &cfg_database) -> DbConn {
    let mut opt = ConnectOptions::new(db.url());
    // 设置连接池大小和其他选项
    opt.min_connections(2) // 最少保持的连接数
        .max_connections(10) //最多允许的连接数
        .connect_timeout(Duration::from_secs(10)) //连接数据库的最大等待时间
        .idle_timeout(Duration::from_secs(300)) // 连接空闲超时时间
        .max_lifetime(Duration::from_secs(3600)) // 连接最大生命周期
        .sqlx_logging(db.debug);

    let conn = Database::connect(opt)
        .await
        .unwrap_or_else(|e| panic!("数据库连接失败：{}", e));
    // 测试能否ping通
    let _ = conn
        .ping()
        .await
        .is_err_and(|e| panic!("数据库连接失败：{}", e));

    info!("数据库连接池初始化完成");
    conn
}
