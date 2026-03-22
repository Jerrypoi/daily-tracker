use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub id: u16,
    pub topic_name: String,
    pub display_color: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_topic_id: Option<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTopicRequest {
    pub topic_name: String,
    pub parent_topic_id: Option<u16>,
    pub display_color: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTopicRequest {
    pub topic_name: String,
    pub display_color: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetTopicsParams {
    pub parent_topic_id: Option<u16>,
}
