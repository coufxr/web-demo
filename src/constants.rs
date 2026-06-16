use redis::aio::ConnectionManager;
use sea_orm::DbConn;
use std::sync::LazyLock;

use crate::project::configs::Configs;

pub static CONFIG: LazyLock<Configs> = LazyLock::new(Configs::new);

/// 应用状态，通过 axum::extract::State 注入到 handler 中
#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
    pub redis: ConnectionManager,
}
