use super::view::{user_detail, user_list};
use axum::{routing::get, Router};

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(user_list))
        .route("/:id", get(user_detail))
}
