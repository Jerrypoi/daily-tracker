use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub id: i64,
    pub topic_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_topic_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTopicRequest {
    pub topic_name: String,
    pub parent_topic_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct GetTopicsParams {
    pub parent_topic_id: Option<i64>,
}
