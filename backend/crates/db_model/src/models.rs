use diesel::prelude::*;
use chrono::NaiveDateTime;
use chrono::Utc;
use uuid::Uuid;




#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::topic)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Topic {
    pub id: Vec<u8>,
    pub topic_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub parent_topic_id: Option<Vec<u8>>,
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
}

impl DailyTrack {
    pub fn new(start_time: NaiveDateTime, topic_id: Option<Vec<u8>>, comment: Option<String>) -> Self {
        let id = Uuid::new_v4();


        let id = id.as_bytes().to_vec();
        Self {
            start_time,
            topic_id,
            comment,
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

        let track = DailyTrack::new(start_time, topic_id.clone(), comment.clone());

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
        let track = DailyTrack::new(start_time, None, comment.clone());

        let rendered = track.to_string();

        // The string representation should at least include the start_time and comment.
        assert!(rendered.contains(&start_time.to_string()));
        assert!(rendered.contains(comment.as_ref().unwrap()));

        // And it should contain the "id:" prefix from the Display implementation.
        assert!(rendered.contains("id:"));
    }
}

