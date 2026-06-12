use sea_orm::DbConn;

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
}
