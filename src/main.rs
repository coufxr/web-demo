use std::sync::Arc;

use axum::{Extension, middleware, routing::get};
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{Level, info};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use apps::user::api_doc::ApiDoc;
use constants::AppState;
use migration::MigratorTrait;
use project::{configs::Configs, db, fallback, logger, middlewares::response::redirect_response};

mod apps;
mod constants;
mod helper;
mod project;

#[tokio::main]
async fn main() {
    // 加载配置文件（config.toml）
    let cfg = Configs::new();
    // 初始化日志系统，返回 guard 用于保持日志写入器存活
    // guard 必须保持到程序结束，否则日志可能未写入就丢失
    let _guard = logger::init(&cfg);

    // 初始化数据库连接
    let db = db::init(&cfg.database).await.expect("数据库连接失败");
    // 将数据库连接放入共享状态，供处理器使用
    let state = Arc::new(AppState { db });

    // 运行数据库迁移，确保表结构是最新的
    migration::Migrator::up(&state.db, None)
        .await
        .expect("数据库生成失败");

    // 请求日志中间件：记录每个请求和响应
    let trace = TraceLayer::new_for_http()
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    // 中间件堆栈（自下而上执行）：
    // 1. trace - 请求日志
    // 2. CompressionLayer - 响应压缩
    // 3. Extension(state) - 注入共享状态
    // 4. redirect_response - 自定义响应处理
    let middleware_stack = ServiceBuilder::new()
        .layer(trace)
        .layer(CompressionLayer::new())
        .layer(Extension(state))
        .layer(middleware::from_fn(redirect_response));

    let (app, mut openapi) = OpenApiRouter::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/v1", apps::v1_routes())
        .fallback(fallback)
        .layer(middleware_stack)
        .split_for_parts();

    openapi.merge(ApiDoc::openapi());
    let app = app.merge(Scalar::with_url("/docs", openapi));

    // 绑定监听地址
    let server_url = format!("{}:{}", &cfg.app.host, &cfg.app.port);
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
