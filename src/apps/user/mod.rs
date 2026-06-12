mod view;

pub mod api_doc;
pub mod constants;
pub mod schemas;

use axum::routing::get;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::constants::AppState;
use schemas::{UserCreate, UserListInput, UserListOutput, UserPatch};
use view::{user_create, user_delete, user_detail, user_list, user_patch};

pub fn user_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .route("/", get(user_list).post(user_create))
        .route(
            "/{id}",
            get(user_detail).patch(user_patch).delete(user_delete),
        )
}

#[derive(OpenApi)]
#[openapi(
    paths(
        view::user_list,
        view::user_create,
        view::user_detail,
        view::user_patch,
        view::user_delete,
    ),
    components(
        schemas(UserCreate, UserListInput, UserListOutput, UserPatch)
    ),
    tags(
        (name = "用户管理", description = "用户管理相关接口")
    )
)]
pub struct UserApi;
