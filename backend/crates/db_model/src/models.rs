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

