use diesel::prelude::*;
use chrono::NaiveDateTime;
use chrono::Utc;
use uuid::Uuid;

pub const DEFAULT_TOPIC_DISPLAY_COLOR: &str = "#3b82f6";



#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::topic)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Topic {
    pub id: Vec<u8>,
    pub topic_name: String,
    pub display_color: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub parent_topic_id: Option<Vec<u8>>,
    pub user_id: Option<Vec<u8>>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::daily_track)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct DailyTrack {
    pub id: Vec<u8>,
    pub start_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub topic_id: Option<Vec<u8>>,
    pub comment: Option<String>,
    pub user_id: Option<Vec<u8>>,
}

impl DailyTrack {
    pub fn new(start_time: NaiveDateTime, topic_id: Option<Vec<u8>>, comment: Option<String>, user_id: Option<Vec<u8>>) -> Self {
        let id = Uuid::new_v4();

        let id = id.as_bytes().to_vec();
        Self {
            start_time,
            topic_id,
            comment,
            user_id,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            id: id,
        }   
    }
}

impl std::fmt::Display for DailyTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uuid_bytes = self.id.as_slice().try_into().unwrap_or_default();

        let uuid = Uuid::from_bytes(uuid_bytes);
        write!(f, "id: {}, start_time: {}, comment: {}", uuid.to_string(), self.start_time, self.comment.as_ref().unwrap_or(&String::new()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daily_track_new_sets_expected_fields() {
        let start_time = Utc::now().naive_utc();
        let topic_id = Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        let comment = Some("test comment".to_string());
        let user_id = None;

        let track = DailyTrack::new(start_time, topic_id.clone(), comment.clone(), user_id.clone());

        // id should be a UUID stored as 16 bytes
        assert_eq!(track.id.len(), 16);

        // simple field passthroughs
        assert_eq!(track.start_time, start_time);
        assert_eq!(track.topic_id, topic_id);
        assert_eq!(track.comment, comment);

        // created_at should be set to "now" (roughly) – just assert it is not the default
        // and is >= start_time to avoid relying on exact timing.
        assert!(track.created_at >= start_time);
        assert!(track.updated_at.is_none());
    }

    #[test]
    fn daily_track_display_includes_id_and_comment() {
        let start_time = Utc::now().naive_utc();
        let comment = Some("display test".to_string());
        let track = DailyTrack::new(start_time, None, comment.clone(), None);

        let rendered = track.to_string();

        // The string representation should at least include the start_time and comment.
        assert!(rendered.contains(&start_time.to_string()));
        assert!(rendered.contains(comment.as_ref().unwrap()));

        // And it should contain the "id:" prefix from the Display implementation.
        assert!(rendered.contains("id:"));
    }

    #[test]
    fn daily_track_display_no_comment() {
        let start_time = Utc::now().naive_utc();
        let track = DailyTrack::new(start_time, None, None, None);
        let rendered = track.to_string();
        // When comment is None, it should display empty string for comment
        assert!(rendered.contains("comment: "));
    }

    #[test]
    fn daily_track_new_with_user_id() {
        let start_time = Utc::now().naive_utc();
        let user_id = Some(vec![10, 20, 30]);
        let track = DailyTrack::new(start_time, None, None, user_id.clone());
        assert_eq!(track.user_id, user_id);
    }

    #[test]
    fn daily_track_new_with_topic_id() {
        let start_time = Utc::now().naive_utc();
        let topic_id = Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        let track = DailyTrack::new(start_time, topic_id.clone(), None, None);
        assert_eq!(track.topic_id, topic_id);
    }

    #[test]
    fn daily_track_id_is_uuid_length() {
        let start_time = Utc::now().naive_utc();
        let track = DailyTrack::new(start_time, None, None, None);
        assert_eq!(track.id.len(), 16); // UUID v4 = 16 bytes
    }

    #[test]
    fn daily_track_new_unique_ids() {
        let start_time = Utc::now().naive_utc();
        let track1 = DailyTrack::new(start_time, None, None, None);
        let track2 = DailyTrack::new(start_time, None, None, None);
        assert_ne!(track1.id, track2.id);
    }

    #[test]
    fn default_topic_display_color_is_valid_hex() {
        assert_eq!(DEFAULT_TOPIC_DISPLAY_COLOR, "#3b82f6");
        assert_eq!(DEFAULT_TOPIC_DISPLAY_COLOR.len(), 7);
        assert!(DEFAULT_TOPIC_DISPLAY_COLOR.starts_with('#'));
    }
}

#[derive(Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::schema::api_keys)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ApiKey {
    pub id: Vec<u8>,
    pub user_id: Vec<u8>,
    pub key_hash: String,
    pub key_prefix: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
    pub revoked_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: Vec<u8>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: bool,
    pub verification_code: Option<String>,
    pub verification_code_expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
