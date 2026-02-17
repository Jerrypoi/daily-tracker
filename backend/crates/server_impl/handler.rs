use axum::{Json, http::StatusCode};
use db_model;
use diesel::prelude::*;
use models::*;
use db;

/// Get all topics
pub async fn get_topics(
    Json(params): Json<GetTopicsRequest>,
) -> Result<Json<GetTopicsResponse>, StatusCode> {
    let topics = db::get_topics(params.parent_topic_id).into_iter().map(|topic| db_topic_to_topic(&topic)).collect();
    Ok(Json(GetTopicsResponse {
        topics: Some(topics),
        base_response: BaseResponse::new(),
    }))
}
