use axum::{routing::get, Router};

use crate::app::user::user_routes;

async fn handler() -> &'static str {
    "hello, user"
}

pub fn info_routes() -> Router {
    Router::new().route("/", get(handler))
}

pub fn v1_routes() -> Router {
    Router::new()
        .nest("/user", user_routes())
        .nest("/info", info_routes())
}
