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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn topic_serializes_with_parent() {
        let topic = Topic {
            id: 42,
            topic_name: "work".to_string(),
            display_color: "#3b82f6".to_string(),
            created_at: Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 1, 16, 10, 0, 0).unwrap(),
            parent_topic_id: Some(10),
        };
        let json = serde_json::to_string(&topic).unwrap();
        assert!(json.contains("\"id\":42"));
        assert!(json.contains("\"topic_name\":\"work\""));
        assert!(json.contains("\"display_color\":\"#3b82f6\""));
        assert!(json.contains("\"parent_topic_id\":10"));
    }

    #[test]
    fn topic_serializes_without_parent_skips_field() {
        let topic = Topic {
            id: 1,
            topic_name: "play".to_string(),
            display_color: "#ff0000".to_string(),
            created_at: Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap(),
            parent_topic_id: None,
        };
        let json = serde_json::to_string(&topic).unwrap();
        // skip_serializing_if means parent_topic_id should be absent
        assert!(!json.contains("parent_topic_id"));
    }

    #[test]
    fn topic_roundtrip_serialization() {
        let topic = Topic {
            id: 99,
            topic_name: "exercise".to_string(),
            display_color: "#00ff00".to_string(),
            created_at: Utc.with_ymd_and_hms(2026, 3, 1, 8, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 3, 2, 8, 0, 0).unwrap(),
            parent_topic_id: Some(5),
        };
        let json = serde_json::to_string(&topic).unwrap();
        let deserialized: Topic = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, 99);
        assert_eq!(deserialized.topic_name, "exercise");
        assert_eq!(deserialized.parent_topic_id, Some(5));
    }

    #[test]
    fn create_topic_request_deserializes() {
        let json = r##"{"topic_name":"sleep","parent_topic_id":3,"display_color":"#112233"}"##;
        let req: CreateTopicRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.topic_name, "sleep");
        assert_eq!(req.parent_topic_id, Some(3));
        assert_eq!(req.display_color, Some("#112233".to_string()));
    }

    #[test]
    fn create_topic_request_optional_fields() {
        let json = r#"{"topic_name":"reading"}"#;
        let req: CreateTopicRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.topic_name, "reading");
        assert_eq!(req.parent_topic_id, None);
        assert_eq!(req.display_color, None);
    }

    #[test]
    fn update_topic_request_deserializes() {
        let json = r##"{"topic_name":"coding","display_color":"#abcdef"}"##;
        let req: UpdateTopicRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.topic_name, "coding");
        assert_eq!(req.display_color, "#abcdef");
    }

    #[test]
    fn get_topics_params_deserializes_with_parent() {
        let json = r#"{"parent_topic_id":7}"#;
        let params: GetTopicsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.parent_topic_id, Some(7));
    }

    #[test]
    fn get_topics_params_deserializes_without_parent() {
        let json = r#"{}"#;
        let params: GetTopicsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.parent_topic_id, None);
    }
}
