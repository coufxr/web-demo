use std::sync::Arc;

use axum::{Extension, Router, routing::get};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

use crate::configs::{Configs, FormEnv};
use crate::routes::fallback;

mod app;
mod configs;
mod db;
mod entity;
mod error;
mod logger;
mod response;
mod routes;
mod tools;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let cfg = Configs::form_env();
    let _guard = logger::init(&cfg).await;

    let db = db::init(&cfg).await;
    let state = Arc::new(db::AppState { db });

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
        .layer(Extension(state));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" })) // 根路由
        .nest("/api/v1", routes::v1_routes()) // 路由嵌套方式
        .fallback(fallback) // 路由错误处理
        .layer(middleware_stack);

    let server_url = format!("{}:{}", &cfg.server.host, &cfg.server.port);

    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
