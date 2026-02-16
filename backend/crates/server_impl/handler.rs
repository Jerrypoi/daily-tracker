use crate::db::DB_CONNECTION;
use axum::{Json, http::StatusCode};
use db_model;
use diesel::prelude::*;
// use diesel::query_dsl::methods::SelectDsl;
use models::*;

/// Get all topics
pub async fn get_topics(
    Json(params): Json<GetTopicsRequest>,
) -> Result<Json<GetTopicsResponse>, StatusCode> {
    let mut conn = DB_CONNECTION.lock().unwrap();
    let topics = db_model::schema::topic::dsl::topic
        .select(db_model::models::Topic::as_select())
        .load(&mut *conn)
        .unwrap();
    let topics = topics.into_iter().map(|topic| Topic {
        id: topic.id.iter().reduce(|value, acc| 10).into(),
        topic_name: topic.topic_name,
        created_at: topic.created_at,
        updated_at: topic.updated_at,
        parent_topic_id: topic.parent_topic_id,
    }).collect();
    
    let response = GetTopicsResponse {
        topics: Some(topics),
        base_response: BaseResponse::new(),
    };
    Ok(Json(response))
}
