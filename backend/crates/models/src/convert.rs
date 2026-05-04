use crate::Topic;
use crate::api_key::ApiKeyResponse;
use crate::daily_track::DailyTrack;
use chrono::{TimeZone, Utc};
use db_model;

/// Convert a `Vec<u8>` to `i64` using big-endian byte order.
/// Takes the first 8 bytes of the vector.
fn vec_u8_to_i64(bytes: &[u8]) -> i64 {
    let arr: [u8; 8] = bytes[..8].try_into().expect("id must be at least 8 bytes");
    i64::from_be_bytes(arr)
}

fn vec_u8_to_u16(bytes: &[u8]) -> u16 {
    // 
    let arr: [u8; 2] = bytes[..2].try_into().expect("id must be at least 2 bytes");
    u16::from_be_bytes(arr)
}

pub fn db_topic_to_topic(topic: &db_model::models::Topic) -> Topic {
    let id = vec_u8_to_u16(&topic.id);
    let topic_name = topic.topic_name.clone();
    let display_color = topic.display_color.clone();
    let created_at = Utc.from_utc_datetime(&topic.created_at);
    let updated_at = topic
        .updated_at
        .map(|dt| Utc.from_utc_datetime(&dt))
        .unwrap_or(created_at);
    let parent_topic_id = topic.parent_topic_id.as_deref().map(vec_u8_to_u16);

    Topic { id, topic_name, display_color, created_at, updated_at, parent_topic_id }
}

pub fn db_api_key_to_response(key: &db_model::models::ApiKey) -> ApiKeyResponse {
    ApiKeyResponse {
        id: vec_u8_to_u16(&key.id),
        name: key.name.clone(),
        key_prefix: key.key_prefix.clone(),
        created_at: Utc.from_utc_datetime(&key.created_at).to_rfc3339(),
        last_used_at: key
            .last_used_at
            .map(|dt| Utc.from_utc_datetime(&dt).to_rfc3339()),
    }
}

pub fn db_daily_track_to_daily_track(track: &db_model::models::DailyTrack) -> DailyTrack {
    let id = vec_u8_to_u16(&track.id);
    let start_time = Utc.from_utc_datetime(&track.start_time);
    let created_at = Utc.from_utc_datetime(&track.created_at);
    let updated_at = track
        .updated_at
        .map(|dt| Utc.from_utc_datetime(&dt))
        .unwrap_or(created_at);
    let topic_id = track
        .topic_id
        .as_deref()
        .map(vec_u8_to_u16)
        .unwrap_or(0);
    let comment = track.comment.clone();

    DailyTrack { id, start_time, created_at, updated_at, topic_id, comment }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn vec_u8_to_i64_converts_big_endian() {
        let bytes = 42i64.to_be_bytes().to_vec();
        assert_eq!(vec_u8_to_i64(&bytes), 42);
    }

    #[test]
    fn vec_u8_to_i64_negative_value() {
        let bytes = (-1i64).to_be_bytes().to_vec();
        assert_eq!(vec_u8_to_i64(&bytes), -1);
    }

    #[test]
    fn vec_u8_to_i64_uses_only_first_8_bytes() {
        // 16-byte UUID-sized input — only first 8 bytes matter
        let mut bytes = 99i64.to_be_bytes().to_vec();
        bytes.extend_from_slice(&[0xFF; 8]);
        assert_eq!(bytes.len(), 16);
        assert_eq!(vec_u8_to_i64(&bytes), 99);
    }

    #[test]
    #[should_panic]
    fn vec_u8_to_i64_panics_on_short_input() {
        vec_u8_to_i64(&[1, 2, 3]);
    }

    #[test]
    fn db_topic_to_topic_converts_all_fields() {
        let naive_created = NaiveDateTime::parse_from_str(
            "2026-01-15 10:30:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();
        let naive_updated = NaiveDateTime::parse_from_str(
            "2026-01-16 12:00:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();

        // Build 16-byte IDs where first 2 bytes encode the u16 value
        let mut id_bytes = 123u16.to_be_bytes().to_vec();
        id_bytes.extend_from_slice(&[0; 14]); // pad to 16 bytes
        let mut parent_id_bytes = 456u16.to_be_bytes().to_vec();
        parent_id_bytes.extend_from_slice(&[0; 14]);

        let db_topic = db_model::models::Topic {
            id: id_bytes,
            topic_name: "working".to_string(),
            display_color: "#3b82f6".to_string(),
            created_at: naive_created,
            updated_at: Some(naive_updated),
            parent_topic_id: Some(parent_id_bytes),
            user_id: Some(vec![1, 2, 3]),
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
        let naive_created = NaiveDateTime::parse_from_str(
            "2026-02-01 08:00:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();

        let mut id_bytes = 1u16.to_be_bytes().to_vec();
        id_bytes.extend_from_slice(&[0; 14]);

        let db_topic = db_model::models::Topic {
            id: id_bytes,
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

    // --- vec_u8_to_u16 tests ---

    #[test]
    fn vec_u8_to_u16_converts_big_endian() {
        let bytes = 42u16.to_be_bytes().to_vec();
        assert_eq!(vec_u8_to_u16(&bytes), 42);
    }

    #[test]
    fn vec_u8_to_u16_max_value() {
        let bytes = u16::MAX.to_be_bytes().to_vec();
        assert_eq!(vec_u8_to_u16(&bytes), u16::MAX);
    }

    #[test]
    fn vec_u8_to_u16_from_longer_slice() {
        // 16-byte UUID — only first 2 bytes matter
        let mut bytes = 500u16.to_be_bytes().to_vec();
        bytes.extend_from_slice(&[0xFF; 14]);
        assert_eq!(vec_u8_to_u16(&bytes), 500);
    }

    #[test]
    #[should_panic]
    fn vec_u8_to_u16_panics_on_short_input() {
        vec_u8_to_u16(&[1]);
    }

    #[test]
    #[should_panic]
    fn vec_u8_to_u16_panics_on_empty() {
        vec_u8_to_u16(&[]);
    }

    // --- db_daily_track_to_daily_track tests ---

    #[test]
    fn db_daily_track_to_daily_track_converts_all_fields() {
        let naive_created = NaiveDateTime::parse_from_str(
            "2026-01-15 10:00:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();
        let naive_updated = NaiveDateTime::parse_from_str(
            "2026-01-15 10:30:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();
        let naive_start = NaiveDateTime::parse_from_str(
            "2026-01-15 09:30:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();

        let db_track = db_model::models::DailyTrack {
            id: 77u16.to_be_bytes().to_vec().into_iter()
                .chain(std::iter::repeat(0).take(14)).collect(),
            start_time: naive_start,
            created_at: naive_created,
            updated_at: Some(naive_updated),
            topic_id: Some(10u16.to_be_bytes().to_vec().into_iter()
                .chain(std::iter::repeat(0).take(14)).collect()),
            comment: Some("test comment".to_string()),
            user_id: Some(vec![1, 2, 3]),
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
        let naive_now = NaiveDateTime::parse_from_str(
            "2026-01-15 10:00:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();

        let db_track = db_model::models::DailyTrack {
            id: 1u16.to_be_bytes().to_vec().into_iter()
                .chain(std::iter::repeat(0).take(14)).collect(),
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
        // updated_at should fall back to created_at
        assert_eq!(track.updated_at, Utc.from_utc_datetime(&naive_now));
    }

    // --- vec_u8_to_i64 additional tests ---

    #[test]
    fn vec_u8_to_i64_zero() {
        let bytes = 0i64.to_be_bytes().to_vec();
        assert_eq!(vec_u8_to_i64(&bytes), 0);
    }

    #[test]
    fn vec_u8_to_i64_max() {
        let bytes = i64::MAX.to_be_bytes().to_vec();
        assert_eq!(vec_u8_to_i64(&bytes), i64::MAX);
    }

    #[test]
    fn vec_u8_to_i64_min() {
        let bytes = i64::MIN.to_be_bytes().to_vec();
        assert_eq!(vec_u8_to_i64(&bytes), i64::MIN);
    }

    // --- db_api_key_to_response tests ---

    #[test]
    fn db_api_key_to_response_converts_all_fields() {
        let created = NaiveDateTime::parse_from_str(
            "2026-04-25 10:00:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();
        let last_used = NaiveDateTime::parse_from_str(
            "2026-04-25 11:30:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();

        let mut id_bytes = 99u16.to_be_bytes().to_vec();
        id_bytes.extend_from_slice(&[0; 14]);

        let key = db_model::models::ApiKey {
            id: id_bytes,
            user_id: vec![1; 16],
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
        assert_eq!(resp.created_at, Utc.from_utc_datetime(&created).to_rfc3339());
        assert_eq!(
            resp.last_used_at,
            Some(Utc.from_utc_datetime(&last_used).to_rfc3339())
        );
    }

    #[test]
    fn db_api_key_to_response_omits_last_used_when_unused() {
        let created = NaiveDateTime::parse_from_str(
            "2026-04-25 10:00:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();

        let mut id_bytes = 1u16.to_be_bytes().to_vec();
        id_bytes.extend_from_slice(&[0; 14]);

        let key = db_model::models::ApiKey {
            id: id_bytes,
            user_id: vec![0; 16],
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
        // The response shape must not contain the secret hash. Roundtrip via JSON
        // and confirm the hash is absent.
        let created = NaiveDateTime::parse_from_str(
            "2026-04-25 10:00:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();

        let mut id_bytes = 1u16.to_be_bytes().to_vec();
        id_bytes.extend_from_slice(&[0; 14]);

        let key = db_model::models::ApiKey {
            id: id_bytes,
            user_id: vec![0; 16],
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

