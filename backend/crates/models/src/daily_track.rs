use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DailyTrack {
    pub id: i64,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub topic_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDailyTrackRequest {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub topic_id: i64,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetDailyTracksParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub topic_id: Option<i64>,
}
