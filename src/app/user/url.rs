use super::view::{user_create, user_detail, user_list};
use axum::{
    routing::{get, post},
    Router,
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(user_list))
        .route("/", post(user_create))
        .route("/:id", get(user_detail))
}
