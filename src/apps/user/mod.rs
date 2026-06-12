mod view;

pub mod api_doc;
pub mod constants;
pub mod schemas;

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::constants::AppState;
use view::{
    __path_user_create, __path_user_delete, __path_user_detail, __path_user_list,
    __path_user_patch, user_create, user_delete, user_detail, user_list, user_patch,
};

pub fn user_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(user_list, user_create))
        .routes(routes!(user_detail, user_patch, user_delete))
}
