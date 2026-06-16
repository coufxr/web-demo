pub mod view;

pub mod constants;
pub mod schemas;

use axum::Router;
use axum::routing::get;

use crate::constants::AppState;
use view::{user_create, user_delete, user_detail, user_list, user_patch};

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/list", get(user_list)).route(
        "/",
        get(user_detail)
            .post(user_create)
            .patch(user_patch)
            .delete(user_delete),
    )
}
