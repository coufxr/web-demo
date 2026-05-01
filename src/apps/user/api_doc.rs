use utoipa::OpenApi;

use super::UserApi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/api/v1/user", api = UserApi)
    ),
    info(
        title = "Web Demo API",
        description = "A demo API built with Axum and SeaORM",
        version = "1.0.0"
    )
)]
pub struct ApiDoc;
