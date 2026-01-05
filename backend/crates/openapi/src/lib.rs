#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, unused_attributes, non_camel_case_types)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
#[cfg(feature = "mock")]
use mockall::automock;
use std::error::Error;
use std::collections::BTreeSet;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper, auth::Authorization};
use serde::{Serialize, Deserialize};

#[cfg(any(feature = "client", feature = "server"))]
type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "/api/v1";
pub const API_VERSION: &str = "1.0.0";

mod auth;
pub use auth::{AuthenticationApi, Claims};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateDailyTrackResponse {
    /// Daily track record created successfully
    DailyTrackRecordCreatedSuccessfully
    (models::DailyTrack)
    ,
    /// Invalid input (e.g., start_time not at :00 or :30)
    InvalidInput
    (models::ErrorResponse)
    ,
    /// Referenced topic not found
    ReferencedTopicNotFound
    (models::ErrorResponse)
    ,
    /// A record already exists for this time period
    ARecordAlreadyExistsForThisTimePeriod
    (models::ErrorResponse)
    ,
    /// Internal server error
    InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetDailyTracksResponse {
    /// Successful operation
    SuccessfulOperation
    (Vec<models::DailyTrack>)
    ,
    /// Invalid date format or parameters
    InvalidDateFormatOrParameters
    (models::ErrorResponse)
    ,
    /// Internal server error
    InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetDailyTrackByIdResponse {
    /// Successful operation
    SuccessfulOperation
    (models::DailyTrack)
    ,
    /// Daily track record not found
    DailyTrackRecordNotFound
    (models::ErrorResponse)
    ,
    /// Internal server error
    InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateTopicResponse {
    /// Topic created successfully
    TopicCreatedSuccessfully
    (models::Topic)
    ,
    /// Invalid input
    InvalidInput
    (models::ErrorResponse)
    ,
    /// Topic name already exists
    TopicNameAlreadyExists
    (models::ErrorResponse)
    ,
    /// Internal server error
    InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetTopicsResponse {
    /// Successful operation
    SuccessfulOperation
    (Vec<models::Topic>)
    ,
    /// Internal server error
    InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetTopicByIdResponse {
    /// Successful operation
    SuccessfulOperation
    (models::Topic)
    ,
    /// Topic not found
    TopicNotFound
    (models::ErrorResponse)
    ,
    /// Internal server error
    InternalServerError
    (models::ErrorResponse)
}

/// API
#[cfg_attr(feature = "mock", automock)]
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    /// Create a new daily track record
    async fn create_daily_track(
        &self,
        body: models::CreateDailyTrackRequest,
        context: &C) -> Result<CreateDailyTrackResponse, ApiError>;

    /// Get daily track records
    async fn get_daily_tracks(
        &self,
        start_date: Option<chrono::naive::NaiveDate>,
        end_date: Option<chrono::naive::NaiveDate>,
        topic_id: Option<i64>,
        context: &C) -> Result<GetDailyTracksResponse, ApiError>;

    /// Get a daily track record by ID
    async fn get_daily_track_by_id(
        &self,
        id: i64,
        context: &C) -> Result<GetDailyTrackByIdResponse, ApiError>;

    /// Create a new topic
    async fn create_topic(
        &self,
        body: models::CreateTopicRequest,
        context: &C) -> Result<CreateTopicResponse, ApiError>;

    /// Get all topics
    async fn get_topics(
        &self,
        parent_topic_id: Option<i64>,
        context: &C) -> Result<GetTopicsResponse, ApiError>;

    /// Get a topic by ID
    async fn get_topic_by_id(
        &self,
        id: i64,
        context: &C) -> Result<GetTopicByIdResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[cfg_attr(feature = "mock", automock)]
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {
    // The std::task::Context struct houses a reference to std::task::Waker with the lifetime <'a>.
    // Adding an anonymous lifetime `'a` to allow mockall to create a mock object with the right lifetimes.
    // This is needed because the compiler is unable to determine the lifetimes on F's trait bound
    // where F is the closure created by mockall. We use higher-rank trait bounds here to get around this.

    fn context(&self) -> &C;

    /// Create a new daily track record
    async fn create_daily_track(
        &self,
        body: models::CreateDailyTrackRequest,
        ) -> Result<CreateDailyTrackResponse, ApiError>;

    /// Get daily track records
    async fn get_daily_tracks(
        &self,
        start_date: Option<chrono::naive::NaiveDate>,
        end_date: Option<chrono::naive::NaiveDate>,
        topic_id: Option<i64>,
        ) -> Result<GetDailyTracksResponse, ApiError>;

    /// Get a daily track record by ID
    async fn get_daily_track_by_id(
        &self,
        id: i64,
        ) -> Result<GetDailyTrackByIdResponse, ApiError>;

    /// Create a new topic
    async fn create_topic(
        &self,
        body: models::CreateTopicRequest,
        ) -> Result<CreateTopicResponse, ApiError>;

    /// Get all topics
    async fn get_topics(
        &self,
        parent_topic_id: Option<i64>,
        ) -> Result<GetTopicsResponse, ApiError>;

    /// Get a topic by ID
    async fn get_topic_by_id(
        &self,
        id: i64,
        ) -> Result<GetTopicByIdResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Create a new daily track record
    async fn create_daily_track(
        &self,
        body: models::CreateDailyTrackRequest,
        ) -> Result<CreateDailyTrackResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_daily_track(body, &context).await
    }

    /// Get daily track records
    async fn get_daily_tracks(
        &self,
        start_date: Option<chrono::naive::NaiveDate>,
        end_date: Option<chrono::naive::NaiveDate>,
        topic_id: Option<i64>,
        ) -> Result<GetDailyTracksResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_daily_tracks(start_date, end_date, topic_id, &context).await
    }

    /// Get a daily track record by ID
    async fn get_daily_track_by_id(
        &self,
        id: i64,
        ) -> Result<GetDailyTrackByIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_daily_track_by_id(id, &context).await
    }

    /// Create a new topic
    async fn create_topic(
        &self,
        body: models::CreateTopicRequest,
        ) -> Result<CreateTopicResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_topic(body, &context).await
    }

    /// Get all topics
    async fn get_topics(
        &self,
        parent_topic_id: Option<i64>,
        ) -> Result<GetTopicsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_topics(parent_topic_id, &context).await
    }

    /// Get a topic by ID
    async fn get_topic_by_id(
        &self,
        id: i64,
        ) -> Result<GetTopicByIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_topic_by_id(id, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
