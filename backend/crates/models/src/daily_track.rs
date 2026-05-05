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
pub struct UpdateDailyTrackRequest {
    pub topic_id: i64,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetDailyTracksParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub topic_id: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn daily_track_serializes_with_comment() {
        let track = DailyTrack {
            id: 1,
            start_time: Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap(),
            created_at: Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap(),
            topic_id: 42,
            comment: Some("working on feature".to_string()),
        };
        let json = serde_json::to_string(&track).unwrap();
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"topic_id\":42"));
        assert!(json.contains("\"comment\":\"working on feature\""));
    }

    #[test]
    fn daily_track_serializes_without_comment_skips_field() {
        let track = DailyTrack {
            id: 1,
            start_time: Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap(),
            created_at: Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap(),
            topic_id: 42,
            comment: None,
        };
        let json = serde_json::to_string(&track).unwrap();
        assert!(!json.contains("comment"));
    }

    #[test]
    fn daily_track_roundtrip() {
        let track = DailyTrack {
            id: 55,
            start_time: Utc.with_ymd_and_hms(2026, 3, 1, 14, 30, 0).unwrap(),
            created_at: Utc.with_ymd_and_hms(2026, 3, 1, 14, 30, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 3, 1, 15, 0, 0).unwrap(),
            topic_id: 10,
            comment: Some("test".to_string()),
        };
        let json = serde_json::to_string(&track).unwrap();
        let deser: DailyTrack = serde_json::from_str(&json).unwrap();
        assert_eq!(deser.id, 55);
        assert_eq!(deser.topic_id, 10);
        assert_eq!(deser.comment, Some("test".to_string()));
    }

    #[test]
    fn create_daily_track_request_deserializes() {
        let json = r#"{"start_time":"2026-01-15T10:00:00Z","topic_id":5,"comment":"hello"}"#;
        let req: CreateDailyTrackRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.topic_id, 5);
        assert_eq!(req.comment, Some("hello".to_string()));
    }

    #[test]
    fn create_daily_track_request_without_comment() {
        let json = r#"{"start_time":"2026-01-15T10:00:00Z","topic_id":5}"#;
        let req: CreateDailyTrackRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.topic_id, 5);
        assert_eq!(req.comment, None);
    }

    #[test]
    fn update_daily_track_request_deserializes() {
        let json = r#"{"topic_id":3,"comment":"updated"}"#;
        let req: UpdateDailyTrackRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.topic_id, 3);
        assert_eq!(req.comment, Some("updated".to_string()));
    }

    #[test]
    fn get_daily_tracks_params_full() {
        let json = r#"{"start_date":"2026-01-01","end_date":"2026-01-31","topic_id":2}"#;
        let params: GetDailyTracksParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.start_date, Some("2026-01-01".to_string()));
        assert_eq!(params.end_date, Some("2026-01-31".to_string()));
        assert_eq!(params.topic_id, Some(2));
    }

    #[test]
    fn get_daily_tracks_params_empty() {
        let json = r#"{}"#;
        let params: GetDailyTracksParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.start_date, None);
        assert_eq!(params.end_date, None);
        assert_eq!(params.topic_id, None);
    }
}
