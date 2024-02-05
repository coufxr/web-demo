pub mod app;
mod middleware;
pub mod routes;
pub mod states;

use axum::{http::StatusCode, routing::get, Extension, Router};
use std::{env, sync::Arc};
use tower::ServiceBuilder;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = states::init_db_connection(db_url.as_str()).await;
    let state = states::AppState { conn };

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG) //只有Debug 模式下才能打印sea-orm的完整sql日志
        .with_writer(std::io::stdout)
        // .with_test_writer()
        .with_target(false)
        .compact()
        .init();

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
                .layer(Extension(Arc::new(state))),
        );

    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
