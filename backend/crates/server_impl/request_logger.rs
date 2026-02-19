use axum::{extract::Request, middleware::Next, response::Response};

use log::info;

// const MAX_BODY_SIZE: usize = 1024 * 1024; // 1 MB

pub async fn log_request(request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    let uri = request.uri().to_string();
    let response = next.run(request).await;
    let status = response.status().to_string();
    info!("Method: [{}], URI: [{}], Status: [{}]", method, uri, status);
    return response;
}
