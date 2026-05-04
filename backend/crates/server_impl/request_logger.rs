use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};

use log::info;
use logging::LOG_ID;
use uuid::Uuid;

const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

pub async fn log_request(request: Request, next: Next) -> Response {
    // Honor an inbound x-request-id when present; otherwise generate one.
    let log_id = request
        .headers()
        .get(&REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().simple().to_string());

    let method = request.method().to_string();
    let uri = request.uri().to_string();

    LOG_ID
        .scope(log_id.clone(), async move {
            info!("--> Method: [{}], URI: [{}]", method, uri);
            let mut response = next.run(request).await;
            let status = response.status().to_string();
            info!(
                "<-- Method: [{}], URI: [{}], Status: [{}]",
                method, uri, status
            );

            if let Ok(value) = HeaderValue::from_str(&log_id) {
                response.headers_mut().insert(REQUEST_ID_HEADER, value);
            }
            response
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, body::Body, middleware, routing::get};
    use http_body_util::BodyExt;
    use tower::ServiceExt as _;

    async fn ok_handler() -> &'static str {
        "ok"
    }

    fn test_app() -> Router {
        Router::new()
            .route("/test", get(ok_handler))
            .layer(middleware::from_fn(log_request))
    }

    #[tokio::test]
    async fn log_request_passes_through_get() {
        let app = test_app();
        let request = Request::builder()
            .uri("/test")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response: axum::response::Response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), 200);
        assert!(response.headers().get(&REQUEST_ID_HEADER).is_some());
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"ok");
    }

    #[tokio::test]
    async fn log_request_returns_404_for_unknown_route() {
        let app = test_app();
        let request = Request::builder()
            .uri("/nonexistent")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response: axum::response::Response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), 404);
    }

    #[tokio::test]
    async fn log_request_honors_inbound_request_id() {
        let app = test_app();
        let request = Request::builder()
            .uri("/test")
            .method("GET")
            .header("x-request-id", "abc123")
            .body(Body::empty())
            .unwrap();

        let response: axum::response::Response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), 200);
        let value = response.headers().get(&REQUEST_ID_HEADER).unwrap();
        assert_eq!(value.to_str().unwrap(), "abc123");
    }
}
