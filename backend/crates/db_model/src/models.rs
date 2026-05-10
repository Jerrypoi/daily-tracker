use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;

pub const DEFAULT_TOPIC_DISPLAY_COLOR: &str = "#3b82f6";

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::topic)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Topic {
    pub id: i64,
    pub topic_name: String,
    pub display_color: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub parent_topic_id: Option<i64>,
    pub user_id: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::topic)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewTopic {
    pub id: i64,
    pub topic_name: String,
    pub display_color: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub parent_topic_id: Option<i64>,
    pub user_id: Option<i64>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::daily_track)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct DailyTrack {
    pub id: i64,
    pub start_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub topic_id: Option<i64>,
    pub comment: Option<String>,
    pub user_id: Option<i64>,
    pub duration_minutes: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::daily_track)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewDailyTrack {
    pub id: i64,
    pub start_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub topic_id: Option<i64>,
    pub comment: Option<String>,
    pub user_id: Option<i64>,
    pub duration_minutes: i32,
}

impl NewDailyTrack {
    pub fn new(
        id: i64,
        start_time: NaiveDateTime,
        topic_id: Option<i64>,
        comment: Option<String>,
        user_id: Option<i64>,
        duration_minutes: i32,
    ) -> Self {
        Self {
            id,
            start_time,
            topic_id,
            comment,
            user_id,
            duration_minutes,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}

impl std::fmt::Display for DailyTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, start_time: {}, comment: {}",
            self.id,
            self.start_time,
            self.comment.as_ref().unwrap_or(&String::new())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daily_track_new_sets_expected_fields() {
        let start_time = Utc::now().naive_utc();
        let topic_id = Some(42);
        let comment = Some("test comment".to_string());
        let user_id = None;

        let id = 123;
        let track = NewDailyTrack::new(id, start_time, topic_id, comment.clone(), user_id, 30);

        assert_eq!(track.id, id);
        assert_eq!(track.start_time, start_time);
        assert_eq!(track.topic_id, topic_id);
        assert_eq!(track.comment, comment);
        assert_eq!(track.duration_minutes, 30);

        // created_at should be set to "now" (roughly) – just assert it is not the default
        // and is >= start_time to avoid relying on exact timing.
        assert!(track.created_at >= start_time);
        assert!(track.updated_at.is_none());
    }

    #[test]
    fn daily_track_display_includes_id_and_comment() {
        let start_time = Utc::now().naive_utc();
        let comment = Some("display test".to_string());
        let track = DailyTrack {
            id: 123,
            start_time,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            topic_id: None,
            comment: comment.clone(),
            user_id: None,
            duration_minutes: 30,
        };

        let rendered = track.to_string();

        assert!(rendered.contains(&start_time.to_string()));
        assert!(rendered.contains(comment.as_ref().unwrap()));

        assert!(rendered.contains("id:"));
    }

    #[test]
    fn daily_track_display_no_comment() {
        let start_time = Utc::now().naive_utc();
        let track = DailyTrack {
            id: 123,
            start_time,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            topic_id: None,
            comment: None,
            user_id: None,
            duration_minutes: 30,
        };
        let rendered = track.to_string();
        assert!(rendered.contains("comment: "));
    }

    #[test]
    fn daily_track_new_with_user_id() {
        let start_time = Utc::now().naive_utc();
        let user_id = Some(10);
        let track = NewDailyTrack::new(123, start_time, None, None, user_id, 30);
        assert_eq!(track.user_id, user_id);
    }

    #[test]
    fn daily_track_new_with_topic_id() {
        let start_time = Utc::now().naive_utc();
        let topic_id = Some(1);
        let track = NewDailyTrack::new(123, start_time, topic_id, None, None, 30);
        assert_eq!(track.topic_id, topic_id);
    }

    #[test]
    fn daily_track_new_with_duration_minutes() {
        let start_time = Utc::now().naive_utc();
        let track = NewDailyTrack::new(123, start_time, None, None, None, 90);
        assert_eq!(track.duration_minutes, 90);
    }

    #[test]
    fn default_topic_display_color_is_valid_hex() {
        assert_eq!(DEFAULT_TOPIC_DISPLAY_COLOR, "#3b82f6");
        assert_eq!(DEFAULT_TOPIC_DISPLAY_COLOR.len(), 7);
        assert!(DEFAULT_TOPIC_DISPLAY_COLOR.starts_with('#'));
    }
}

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::api_keys)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ApiKey {
    pub id: i64,
    pub user_id: i64,
    pub key_hash: String,
    pub key_prefix: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
    pub revoked_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = crate::schema::api_keys)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewApiKey {
    pub id: i64,
    pub user_id: i64,
    pub key_hash: String,
    pub key_prefix: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
    pub revoked_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: bool,
    pub verification_code: Option<String>,
    pub verification_code_expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewUser {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: bool,
    pub verification_code: Option<String>,
    pub verification_code_expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
