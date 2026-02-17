use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};
use http_body_util::BodyExt;
use log::{info, error};

// const MAX_BODY_SIZE: usize = 1024 * 1024; // 1 MB

pub async fn log_request(request: Request, next: Next) -> Response {
    let (parts, body) = request.into_parts();

    let bytes = body
        .collect()
        .await
        .map(|collected| collected.to_bytes())
        .unwrap_or_default();

    // let payload = if bytes.len() > MAX_BODY_SIZE {
    //     format!("<body too large: {} bytes>", bytes.len())
    // } else {
    //     String::from_utf8_lossy(&bytes).into_owned()
    // };

    info!("[{}] {} ", parts.method, parts.uri);

    let request = Request::from_parts(parts, Body::from(bytes));
    next.run(request).await
}
