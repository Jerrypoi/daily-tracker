use crate::Topic;
use chrono::{DateTime, TimeZone, Utc};
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

