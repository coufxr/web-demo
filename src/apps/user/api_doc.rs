use utoipa::OpenApi;

/// 顶层 API 文档元数据（路径由 OpenApiRouter 自动收集，无需手动 nest）
#[derive(OpenApi)]
#[openapi(info(
    title = "Web Demo API",
    description = "A demo API built with Axum and SeaORM",
    version = "1.0.0"
))]
pub struct ApiDoc;
