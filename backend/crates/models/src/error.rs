use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

pub enum ApiError {
    BadRequest(String),
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
    Forbidden(String),
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg),
            ApiError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg)
            }
        };

        let body = ErrorResponse {
            error: error_code.to_string(),
            message,
        };

        (status, axum::Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    use http_body_util::BodyExt;

    async fn extract_error_response(api_error: ApiError) -> (StatusCode, ErrorResponse) {
        let response = api_error.into_response();
        let status = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let error_response: ErrorResponse = serde_json::from_slice(&body).unwrap();
        (status, error_response)
    }

    #[tokio::test]
    async fn bad_request_returns_400() {
        let (status, body) = extract_error_response(
            ApiError::BadRequest("bad input".to_string()),
        ).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body.error, "VALIDATION_ERROR");
        assert_eq!(body.message, "bad input");
    }

    #[tokio::test]
    async fn not_found_returns_404() {
        let (status, body) = extract_error_response(
            ApiError::NotFound("not here".to_string()),
        ).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body.error, "NOT_FOUND");
        assert_eq!(body.message, "not here");
    }

    #[tokio::test]
    async fn conflict_returns_409() {
        let (status, body) = extract_error_response(
            ApiError::Conflict("duplicate".to_string()),
        ).await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(body.error, "CONFLICT");
        assert_eq!(body.message, "duplicate");
    }

    #[tokio::test]
    async fn unauthorized_returns_401() {
        let (status, body) = extract_error_response(
            ApiError::Unauthorized("no auth".to_string()),
        ).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body.error, "UNAUTHORIZED");
        assert_eq!(body.message, "no auth");
    }

    #[tokio::test]
    async fn forbidden_returns_403() {
        let (status, body) = extract_error_response(
            ApiError::Forbidden("not allowed".to_string()),
        ).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(body.error, "FORBIDDEN");
        assert_eq!(body.message, "not allowed");
    }

    #[tokio::test]
    async fn internal_server_error_returns_500() {
        let (status, body) = extract_error_response(
            ApiError::InternalServerError("something broke".to_string()),
        ).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body.error, "INTERNAL_ERROR");
        assert_eq!(body.message, "something broke");
    }

    #[test]
    fn error_response_serializes_to_json() {
        let resp = ErrorResponse {
            error: "TEST".to_string(),
            message: "test message".to_string(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"error\":\"TEST\""));
        assert!(json.contains("\"message\":\"test message\""));
    }

    #[test]
    fn error_response_deserializes_from_json() {
        let json = r#"{"error":"CONFLICT","message":"dup"}"#;
        let resp: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.error, "CONFLICT");
        assert_eq!(resp.message, "dup");
    }
}
