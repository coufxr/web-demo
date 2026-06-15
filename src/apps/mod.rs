use axum::Router;

use crate::constants::AppState;
use user::user_routes;

pub mod user;

pub fn v1_routes() -> Router<AppState> {
    Router::new().nest("/user", user_routes())
}
