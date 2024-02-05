use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

// 创建数据库连接池
// pub async fn init_db_connection(db_url: String) -> DatabaseConnection {
//     Database::connect(db_url).await.unwrap()
// }

pub async fn init_db_connection(db_url: &str) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(db_url.to_owned());

    // 设置连接池大小和其他选项
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(std::time::Duration::from_secs(8))
        .idle_timeout(std::time::Duration::from_secs(8))
        .max_lifetime(std::time::Duration::from_secs(8));

    let db = Database::connect(opt)
        .await
        .expect("Failed to connect to database");

    info!("数据库连接池初始化完成");

    return db;
}
