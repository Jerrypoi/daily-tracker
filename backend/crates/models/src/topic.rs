use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Topic {
    /// Unique identifier for the topic
    #[serde(rename = "id")]
    pub id: i64,

    /// Name of the topic (e.g., 'playing', 'working')
    #[serde(rename = "topic_name")]
    // #[validate(custom(function = "check_xss_string"))]
    pub topic_name: String,

    /// Timestamp when the topic was created
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime::<chrono::Utc>,

    /// Timestamp when the topic was last updated
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime::<chrono::Utc>,

    /// ID of the parent topic (null for root-level topics)
    #[serde(rename = "parent_topic_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_topic_id: Option<i64>,
}


#[derive(Serialize, Deserialize)]
pub struct BaseResponse {
    pub status_code: i32,
    pub status_message: String,
}

#[derive(Serialize, Deserialize)]
pub struct BaseRequest {
    pub headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetTopicsResponse {
    pub topics: Option<Vec<Topic>>,
    pub base_response: BaseResponse,
}

#[derive(Serialize, Deserialize)]
pub struct GetTopicsRequest {
    pub parent_topic_id: Option<i64>,
    pub base_request: BaseRequest,
}

impl BaseResponse {

    pub fn new() -> Self {
        Self { status_code: 0, status_message: "success".to_string() }
    }
}
