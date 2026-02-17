use crate::db::DB_CONNECTION;
use axum::{Json, http::StatusCode};
use db_model;
use diesel::prelude::*;
use models::*;

/// Get all topics
pub async fn get_topics(Json(_params): Json<GetTopicsRequest>) -> Result<Json<GetTopicsResponse>, StatusCode> {
    let mut conn = DB_CONNECTION.lock().unwrap();
    let db_topics = db_model::schema::topic::dsl::topic
        .select(db_model::models::Topic::as_select())
        .load(&mut *conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let topics = db_topics.into_iter().map(|topic| db_topic_to_topic(&topic)).collect();
    let response = GetTopicsResponse {
        topics: Some(topics),
        base_response: BaseResponse::new(),
    };
    Ok(Json(response))
}
