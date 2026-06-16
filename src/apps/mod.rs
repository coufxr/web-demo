use axum::Router;

use super::constants::AppState;
use auth::auth_routes;
use user::user_routes;

pub mod auth;
pub mod user;

pub fn v1_routes() -> Router<AppState> {
    Router::new()
        .nest("/user", user_routes())
        .nest("/auth", auth_routes())
}
