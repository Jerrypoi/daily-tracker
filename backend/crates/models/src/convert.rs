use crate::Topic;
use chrono::{TimeZone, Utc};
use db_model;

/// Convert a `Vec<u8>` to `i64` using big-endian byte order.
/// Takes the first 8 bytes of the vector.
fn vec_u8_to_i64(bytes: &[u8]) -> i64 {
    let arr: [u8; 8] = bytes[..8].try_into().expect("id must be at least 8 bytes");
    i64::from_be_bytes(arr)
}

pub fn db_topic_to_topic(topic: db_model::models::Topic) -> Topic {
    let id = vec_u8_to_i64(&topic.id);
    let topic_name = topic.topic_name;
    let created_at = Utc.from_utc_datetime(&topic.created_at);
    let updated_at = topic
        .updated_at
        .map(|dt| Utc.from_utc_datetime(&dt))
        .unwrap_or(created_at);
    let parent_topic_id = topic.parent_topic_id.as_deref().map(vec_u8_to_i64);

    Topic { id, topic_name, created_at, updated_at, parent_topic_id }
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
    #[should_panic(expected = "id must be at least 8 bytes")]
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

        let db_topic = db_model::models::Topic {
            id: 123i64.to_be_bytes().to_vec(),
            topic_name: "working".to_string(),
            created_at: naive_created,
            updated_at: Some(naive_updated),
            parent_topic_id: Some(456i64.to_be_bytes().to_vec()),
        };

        let topic = db_topic_to_topic(db_topic);

        assert_eq!(topic.id, 123);
        assert_eq!(topic.topic_name, "working");
        assert_eq!(topic.created_at, Utc.from_utc_datetime(&naive_created));
        assert_eq!(topic.updated_at, Utc.from_utc_datetime(&naive_updated));
        assert_eq!(topic.parent_topic_id, Some(456));
    }

    #[test]
    fn db_topic_to_topic_updated_at_falls_back_to_created_at() {
        let naive_created = NaiveDateTime::parse_from_str(
            "2026-02-01 08:00:00", "%Y-%m-%d %H:%M:%S"
        ).unwrap();

        let db_topic = db_model::models::Topic {
            id: 1i64.to_be_bytes().to_vec(),
            topic_name: "playing".to_string(),
            created_at: naive_created,
            updated_at: None,
            parent_topic_id: None,
        };

        let topic = db_topic_to_topic(db_topic);

        assert_eq!(topic.updated_at, Utc.from_utc_datetime(&naive_created));
        assert_eq!(topic.parent_topic_id, None);
    }
}

