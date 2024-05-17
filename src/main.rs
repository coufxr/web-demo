use std::sync::Arc;

use axum::{Extension, http::StatusCode, Router, routing::get};
use tower::ServiceBuilder;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

use crate::configs::{Configs, FormEnv};

mod app;
pub mod configs;
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

    async fn fallback() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not Found")
    }

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" })) // 根路由
        .nest("/api/v1", routes::v1_routes()) // 路由嵌套方式
        .fallback(fallback)
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(trace::DefaultMakeSpan::new().level(Level::DEBUG))
                        .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG)),
                )
                .layer(Extension(state)),
        );

    let server_url = format!("{}:{}", &cfg.server.host, &cfg.server.port);

    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
