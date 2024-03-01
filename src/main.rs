use std::{env, sync::Arc};
use std::str::FromStr;

use axum::{Extension, http::StatusCode, Router, routing::get};
use tower::ServiceBuilder;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

mod app;
mod entity;
mod error;
mod logger;
mod response;
mod routes;
mod states;
mod tools;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let logger_level = env::var("logger_level").expect("logger_level is not set in .env file");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let db = states::init_db_connection(db_url.as_str()).await;
    let state = Arc::new(states::AppState { db });

    logger::init_logger(Level::from_str(&logger_level).expect("Invalid log level"));

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

    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
