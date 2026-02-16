use axum::{Json, http::StatusCode};
use models::*;

/// Get all topics
pub async fn get_topics(Json(params): Json<GetTopicsRequest>) -> Result<Json<GetTopicsResponse>, StatusCode> {

    let response = GetTopicsResponse {
        topics: Some(vec![]),
        base_response: BaseResponse::new(),
    };

    Ok(Json(response))
}
