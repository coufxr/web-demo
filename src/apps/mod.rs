use utoipa_axum::router::OpenApiRouter;

use crate::constants::AppState;
use user::user_routes;

pub mod user;

pub fn v1_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/user", user_routes())
}
