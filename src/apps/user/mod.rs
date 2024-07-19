mod view;

pub mod schemas;

use axum::{routing::get, Router};

use view::{user_create, user_delete, user_detail, user_list, user_patch};

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(user_list).post(user_create))
        .route(
            "/:id",
            get(user_detail).patch(user_patch).delete(user_delete),
        )
}
