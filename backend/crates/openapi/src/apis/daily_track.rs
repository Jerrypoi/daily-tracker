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
pub enum CreateDailyTrackResponse {
    /// Daily track record created successfully
    Status201_DailyTrackRecordCreatedSuccessfully
    (models::DailyTrack)
    ,
    /// Invalid input (e.g., start_time not at :00 or :30)
    Status400_InvalidInput
    (models::ErrorResponse)
    ,
    /// Referenced topic not found
    Status404_ReferencedTopicNotFound
    (models::ErrorResponse)
    ,
    /// A record already exists for this time period
    Status409_ARecordAlreadyExistsForThisTimePeriod
    (models::ErrorResponse)
    ,
    /// Internal server error
    Status500_InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetDailyTrackByIdResponse {
    /// Successful operation
    Status200_SuccessfulOperation
    (models::DailyTrack)
    ,
    /// Daily track record not found
    Status404_DailyTrackRecordNotFound
    (models::ErrorResponse)
    ,
    /// Internal server error
    Status500_InternalServerError
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetDailyTracksResponse {
    /// Successful operation
    Status200_SuccessfulOperation
    (Vec<models::DailyTrack>)
    ,
    /// Invalid date format or parameters
    Status400_InvalidDateFormatOrParameters
    (models::ErrorResponse)
    ,
    /// Internal server error
    Status500_InternalServerError
    (models::ErrorResponse)
}




/// DailyTrack
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait DailyTrack<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Create a new daily track record.
    ///
    /// CreateDailyTrack - POST /api/v1/daily-tracks
    async fn create_daily_track(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
            body: &models::CreateDailyTrackRequest,
    ) -> Result<CreateDailyTrackResponse, E>;

    /// Get a daily track record by ID.
    ///
    /// GetDailyTrackById - GET /api/v1/daily-tracks/{id}
    async fn get_daily_track_by_id(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetDailyTrackByIdPathParams,
    ) -> Result<GetDailyTrackByIdResponse, E>;

    /// Get daily track records.
    ///
    /// GetDailyTracks - GET /api/v1/daily-tracks
    async fn get_daily_tracks(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::GetDailyTracksQueryParams,
    ) -> Result<GetDailyTracksResponse, E>;
}
