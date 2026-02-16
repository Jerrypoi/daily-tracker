use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateTopicResponse {
    /// Topic created successfully
    Status201_TopicCreatedSuccessfully
    (models::Topic)
    ,
    /// Invalid input
    Status400_InvalidInput
    (models::ErrorResponse)
    ,
    /// Topic name already exists
    Status409_TopicNameAlreadyExists
    (models::ErrorResponse)
    ,
    /// Internal server error
    Status500_InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTopicByIdResponse {
    /// Successful operation
    Status200_SuccessfulOperation
    (models::Topic)
    ,
    /// Topic not found
    Status404_TopicNotFound
    (models::ErrorResponse)
    ,
    /// Internal server error
    Status500_InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTopicsResponse {
    /// Successful operation
    Status200_SuccessfulOperation
    (Vec<models::Topic>)
    ,
    /// Internal server error
    Status500_InternalServerError
    (models::ErrorResponse)
}




/// Topic
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Topic<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Create a new topic.
    ///
    /// CreateTopic - POST /api/v1/topics
    async fn create_topic(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
            body: &models::CreateTopicRequest,
    ) -> Result<CreateTopicResponse, E>;

    /// Get a topic by ID.
    ///
    /// GetTopicById - GET /api/v1/topics/{id}
    async fn get_topic_by_id(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetTopicByIdPathParams,
    ) -> Result<GetTopicByIdResponse, E>;

    /// Get all topics.
    ///
    /// GetTopics - GET /api/v1/topics
    async fn get_topics(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::GetTopicsQueryParams,
    ) -> Result<GetTopicsResponse, E>;
}
