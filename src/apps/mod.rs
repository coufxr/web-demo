use axum::Router;

use user::user_routes;

pub mod user;

pub fn v1_routes() -> Router {
    Router::new().nest("/user", user_routes())
}
