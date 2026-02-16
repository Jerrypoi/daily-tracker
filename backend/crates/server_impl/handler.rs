use axum::{Json, http::StatusCode};
use models::*;

/// Get all topics
pub async fn get_topics() -> Result<Json<GetTopicsResponse>, StatusCode> {

    Ok(Json(GetTopicsResponse {
            topics: Some(vec![]),
            base_response: BaseResponse::new(),
        }),
    )
}
