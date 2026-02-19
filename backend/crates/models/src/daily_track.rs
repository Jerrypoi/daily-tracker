use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct DailyTrack {
    pub id: u16,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub start_time: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub topic_id: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDailyTrackRequest {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub topic_id: u16,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetDailyTracksParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub topic_id: Option<u16>,
}
