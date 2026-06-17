use axum::{Router, middleware, routing::get};
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{Level, info};
use utoipa_scalar::{Scalar, Servable};

use api_doc::ApiDoc;
use constants::{AppState, CONFIG};
use project::middlewares::auth::auth_middleware;
use project::{db, fallback, logger, middlewares::response::redirect_response};

mod api_doc;
mod apps;
mod constants;
mod helper;
mod project;

#[tokio::main]
async fn main() {
    // 初始化日志系统，返回 guard 用于保持日志写入器存活
    // guard 必须保持到程序结束，否则日志可能未写入就丢失
    let _guard = logger::init(&CONFIG);

    // 初始化数据库连接（自动同步 Schema）
    let db = db::init(&CONFIG.database).await.expect("数据库连接失败");

    // 初始化 Redis 连接
    let redis_conn = db::init_redis(&CONFIG.redis).await.expect("Redis 连接失败");

    // 请求日志中间件：记录每个请求和响应
    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    // 应用状态，通过 State 注入到 handler
    let app_state = AppState {
        db,
        redis: redis_conn,
    };

    // 中间件堆栈（自外向内执行）：
    // 1. redirect_response - 自定义响应处理
    // 2. auth_middleware - JWT 认证（公开路径自动跳过）
    // 3. CompressionLayer - 响应压缩
    // 4. trace - 请求日志
    let middleware_stack = ServiceBuilder::new()
        .layer(trace)
        .layer(CompressionLayer::new())
        .layer(middleware::from_fn(auth_middleware))
        .layer(middleware::from_fn(redirect_response));

    // OpenAPI 文档（utoipauto 自动收集，响应体自动包装为 JsonResponse）
    let openapi = ApiDoc::spec();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/v1", apps::v1_routes())
        .merge(Scalar::with_url("/docs", openapi))
        .fallback(fallback)
        .layer(middleware_stack)
        .with_state(app_state);

    // 绑定监听地址
    let server_url = format!("{}:{}", &CONFIG.app.host, &CONFIG.app.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    info!("listening on http://{}", listener.local_addr().unwrap());

    // 启动服务器，支持优雅关闭（收到信号后等待请求处理完成再退出）
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// 监听关闭信号：Ctrl+C（本地开发）或 SIGTERM（Docker/系统 kill）
/// 收到信号后等待正在处理的请求完成，再安全关闭服务器
async fn shutdown_signal() {
    // 监听 Ctrl+C（本地开发环境）
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    // 监听 SIGTERM（Docker 容器收到 docker stop 时触发）
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .await
    };

    // Windows 不支持 SIGTERM，使用 pending 占位
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // 等待任意一个信号触发
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("shutdown signal received, stopping server...");
}
