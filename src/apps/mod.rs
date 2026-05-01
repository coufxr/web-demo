use utoipa_axum::router::OpenApiRouter;

use user::user_routes;

pub mod user;

pub fn v1_routes() -> OpenApiRouter {
    OpenApiRouter::new().nest("/user", user_routes())
}
