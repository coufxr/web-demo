use std::str::from_utf8;

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
    let (parts, body) = res.into_parts();
    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    // 将byte转为可读文本
    let str = serde_json::from_slice::<Value>(&bytes);
    let data = match str {
        Ok(s) => s,
        Err(_) => Value::from(from_utf8(&bytes).unwrap().to_string()),
    };
    // 无法在 Response 没有结果值时返回 {}
    let response = if parts.status == StatusCode::OK {
        JsonResponse::success(data)
    } else {
        JsonResponse::new(parts.status.as_u16(), parts.status.to_string(), Some(data))
    };

    //返回 Response
    Ok(Response::from_parts(parts, response.into_response()))
}
