use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub id: u16,
    pub topic_name: String,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_topic_id: Option<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTopicRequest {
    pub topic_name: String,
    pub parent_topic_id: Option<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct GetTopicsParams {
    pub parent_topic_id: Option<u16>,
}
