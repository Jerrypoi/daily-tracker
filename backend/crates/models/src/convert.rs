use crate::Topic;
use crate::api_key::ApiKeyResponse;
use crate::daily_track::DailyTrack;
use chrono::{TimeZone, Utc};
use db_model;

pub fn db_topic_to_topic(topic: &db_model::models::Topic) -> Topic {
    let created_at = Utc.from_utc_datetime(&topic.created_at);
    let updated_at = topic
        .updated_at
        .map(|dt| Utc.from_utc_datetime(&dt))
        .unwrap_or(created_at);

    Topic {
        id: topic.id,
        topic_name: topic.topic_name.clone(),
        display_color: topic.display_color.clone(),
        created_at,
        updated_at,
        parent_topic_id: topic.parent_topic_id,
    }
}

pub fn db_api_key_to_response(key: &db_model::models::ApiKey) -> ApiKeyResponse {
    ApiKeyResponse {
        id: key.id,
        name: key.name.clone(),
        key_prefix: key.key_prefix.clone(),
        created_at: Utc.from_utc_datetime(&key.created_at).to_rfc3339(),
        last_used_at: key
            .last_used_at
            .map(|dt| Utc.from_utc_datetime(&dt).to_rfc3339()),
    }
}

pub fn db_daily_track_to_daily_track(track: &db_model::models::DailyTrack) -> DailyTrack {
    let start_time = Utc.from_utc_datetime(&track.start_time);
    let created_at = Utc.from_utc_datetime(&track.created_at);
    let updated_at = track
        .updated_at
        .map(|dt| Utc.from_utc_datetime(&dt))
        .unwrap_or(created_at);

    DailyTrack {
        id: track.id,
        start_time,
        created_at,
        updated_at,
        topic_id: track.topic_id.unwrap_or(0),
        comment: track.comment.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn db_topic_to_topic_converts_all_fields() {
        let naive_created =
            NaiveDateTime::parse_from_str("2026-01-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let naive_updated =
            NaiveDateTime::parse_from_str("2026-01-16 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let db_topic = db_model::models::Topic {
            id: 123,
            topic_name: "working".to_string(),
            display_color: "#3b82f6".to_string(),
            created_at: naive_created,
            updated_at: Some(naive_updated),
            parent_topic_id: Some(456),
            user_id: Some(1),
        };

        let topic = db_topic_to_topic(&db_topic);

        assert_eq!(topic.id, 123);
        assert_eq!(topic.topic_name, "working");
        assert_eq!(topic.display_color, "#3b82f6");
        assert_eq!(topic.created_at, Utc.from_utc_datetime(&naive_created));
        assert_eq!(topic.updated_at, Utc.from_utc_datetime(&naive_updated));
        assert_eq!(topic.parent_topic_id, Some(456));
    }

    #[test]
    fn db_topic_to_topic_updated_at_falls_back_to_created_at() {
        let naive_created =
            NaiveDateTime::parse_from_str("2026-02-01 08:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let db_topic = db_model::models::Topic {
            id: 1,
            topic_name: "playing".to_string(),
            display_color: "#3b82f6".to_string(),
            created_at: naive_created,
            updated_at: None,
            parent_topic_id: None,
            user_id: None,
        };

        let topic = db_topic_to_topic(&db_topic);

        assert_eq!(topic.id, 1);
        assert_eq!(topic.updated_at, Utc.from_utc_datetime(&naive_created));
        assert_eq!(topic.parent_topic_id, None);
    }

    #[test]
    fn db_daily_track_to_daily_track_converts_all_fields() {
        let naive_created =
            NaiveDateTime::parse_from_str("2026-01-15 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let naive_updated =
            NaiveDateTime::parse_from_str("2026-01-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let naive_start =
            NaiveDateTime::parse_from_str("2026-01-15 09:30:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let db_track = db_model::models::DailyTrack {
            id: 77,
            start_time: naive_start,
            created_at: naive_created,
            updated_at: Some(naive_updated),
            topic_id: Some(10),
            comment: Some("test comment".to_string()),
            user_id: Some(1),
        };

        let track = db_daily_track_to_daily_track(&db_track);

        assert_eq!(track.id, 77);
        assert_eq!(track.topic_id, 10);
        assert_eq!(track.comment, Some("test comment".to_string()));
        assert_eq!(track.start_time, Utc.from_utc_datetime(&naive_start));
        assert_eq!(track.updated_at, Utc.from_utc_datetime(&naive_updated));
    }

    #[test]
    fn db_daily_track_no_topic_defaults_to_zero() {
        let naive_now =
            NaiveDateTime::parse_from_str("2026-01-15 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let db_track = db_model::models::DailyTrack {
            id: 1,
            start_time: naive_now,
            created_at: naive_now,
            updated_at: None,
            topic_id: None,
            comment: None,
            user_id: None,
        };

        let track = db_daily_track_to_daily_track(&db_track);

        assert_eq!(track.topic_id, 0);
        assert_eq!(track.comment, None);
        assert_eq!(track.updated_at, Utc.from_utc_datetime(&naive_now));
    }

    #[test]
    fn db_api_key_to_response_converts_all_fields() {
        let created =
            NaiveDateTime::parse_from_str("2026-04-25 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let last_used =
            NaiveDateTime::parse_from_str("2026-04-25 11:30:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let key = db_model::models::ApiKey {
            id: 99,
            user_id: 1,
            key_hash: "deadbeef".to_string(),
            key_prefix: "dt_a1b2c3d4".to_string(),
            name: "ci-bot".to_string(),
            created_at: created,
            last_used_at: Some(last_used),
            revoked_at: None,
        };

        let resp = db_api_key_to_response(&key);

        assert_eq!(resp.id, 99);
        assert_eq!(resp.name, "ci-bot");
        assert_eq!(resp.key_prefix, "dt_a1b2c3d4");
        assert_eq!(
            resp.created_at,
            Utc.from_utc_datetime(&created).to_rfc3339()
        );
        assert_eq!(
            resp.last_used_at,
            Some(Utc.from_utc_datetime(&last_used).to_rfc3339())
        );
    }

    #[test]
    fn db_api_key_to_response_omits_last_used_when_unused() {
        let created =
            NaiveDateTime::parse_from_str("2026-04-25 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let key = db_model::models::ApiKey {
            id: 1,
            user_id: 1,
            key_hash: "h".to_string(),
            key_prefix: "dt_pfx".to_string(),
            name: "fresh".to_string(),
            created_at: created,
            last_used_at: None,
            revoked_at: None,
        };

        let resp = db_api_key_to_response(&key);
        assert!(resp.last_used_at.is_none());
    }

    #[test]
    fn db_api_key_to_response_does_not_leak_hash() {
        let created =
            NaiveDateTime::parse_from_str("2026-04-25 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let key = db_model::models::ApiKey {
            id: 1,
            user_id: 1,
            key_hash: "SECRET_HASH_VALUE_DO_NOT_LEAK".to_string(),
            key_prefix: "dt_pfx".to_string(),
            name: "n".to_string(),
            created_at: created,
            last_used_at: None,
            revoked_at: None,
        };

        let resp = db_api_key_to_response(&key);
        let json = serde_json::to_string(&resp).unwrap();
        assert!(!json.contains("SECRET_HASH_VALUE_DO_NOT_LEAK"));
    }
}
