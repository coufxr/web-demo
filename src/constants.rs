use sea_orm::DbConn;

/// 应用状态，通过 axum::extract::State 注入到 handler 中
#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
}
