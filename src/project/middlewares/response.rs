use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;
use serde_json::Value;

use crate::project::response::JsonResponse;

pub async fn redirect_response(
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let res = next.run(request).await;

    let (_, body) = res.into_parts();
    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    // 将byte转为可读文本
    let str = serde_json::from_slice::<Value>(&bytes);

    // 初始化JsonResponse
    let response = match str {
        Ok(s) => JsonResponse::new(s),
        Err(e) => JsonResponse::error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    //返回 Response
    Ok(response)
}
