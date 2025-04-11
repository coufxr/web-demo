use std::sync::Arc;

use axum::{middleware, routing::get, Extension, Router};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

use constants::AppState;
use migration::MigratorTrait;
use project::{configs::Configs, db, fallback, logger, middlewares::response::redirect_response};

mod apps;
mod constants;
mod entity;
mod helper;
mod project;


#[tokio::main]
async fn main() {
    let cfg = Configs::new();
    let _guard = logger::init(&cfg).await;

    let db = db::init(&cfg.database).await;
    let state = Arc::new(AppState { db });

    migration::Migrator::up(&state.db, None)
        .await
        .expect("数据库生成失败");

    // 请求日志
    let trace = TraceLayer::new_for_http()
        .make_span_with(
            trace::DefaultMakeSpan::new()
                .include_headers(true) // 包含请求头
                .level(Level::INFO),
        )
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO)); // 请求结束时的行为

    // 中间件
    let middleware_stack = ServiceBuilder::new()
        .layer(trace)
        .layer(CompressionLayer::new()) // 启用Brotli压缩
        .layer(Extension(state))
        .layer(middleware::from_fn(redirect_response));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" })) // 根路由
        .nest("/api/v1", apps::v1_routes()) // 路由嵌套方式
        .fallback(fallback) // 路由错误处理
        .layer(middleware_stack);

    let server_url = format!("{}:{}", &cfg.app.host, &cfg.app.port);

    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
