use chrono::{NaiveDate, NaiveDateTime};
use db_model::establish_connection;
use db_model::models::{DailyTrack, Topic};
use db_model::schema;
use diesel::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use uuid::Uuid;

pub static DB_CONNECTION: Lazy<Mutex<MysqlConnection>> =
    Lazy::new(|| Mutex::new(establish_connection()));

pub fn i64_to_binary(val: i64) -> Vec<u8> {
    val.to_be_bytes().to_vec()
}

pub fn u16_to_binary(val: u16) -> Vec<u8> {
    val.to_be_bytes().to_vec()
}

pub fn get_topics(parent_topic_id: Option<u16>) -> Result<Vec<Topic>, DieselError> {
    let mut connection = DB_CONNECTION.lock().unwrap();

    if let Some(parent_topic_id) = parent_topic_id {
        schema::topic::dsl::topic
            .filter(schema::topic::parent_topic_id.eq(u16_to_binary(parent_topic_id)))
            .select(Topic::as_select())
            .load(&mut *connection)
    } else {
        schema::topic::dsl::topic
            .select(Topic::as_select())
            .load(&mut *connection)
    }
}

pub fn create_topic(
    topic_name: String,
    parent_topic_id: Option<u16>,
) -> Result<Topic, DieselError> {
    let mut connection = DB_CONNECTION.lock().unwrap();

    let existing: Option<Topic> = schema::topic::dsl::topic
        .filter(schema::topic::topic_name.eq(&topic_name))
        .select(Topic::as_select())
        .first(&mut *connection)
        .optional()?;

    if existing.is_some() {
        return Err(DieselError::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new(format!("Topic with name '{}' already exists", topic_name)),
        ));
    }

    let id = Uuid::new_v4().as_bytes().to_vec();
    let now = chrono::Utc::now().naive_utc();
    let parent_id_binary = parent_topic_id.map(u16_to_binary);

    let new_topic = Topic {
        id: id.clone(),
        topic_name,
        created_at: now,
        updated_at: None,
        parent_topic_id: parent_id_binary,
    };

    diesel::insert_into(schema::topic::table)
        .values(&new_topic)
        .execute(&mut *connection)?;

    Ok(new_topic)
}

pub fn get_topic_by_id(id: u16) -> Result<Option<Topic>, DieselError> {
    let mut connection = DB_CONNECTION.lock().unwrap();
    let binary_id = u16_to_binary(id);

    schema::topic::dsl::topic
        .filter(schema::topic::id.eq(binary_id))
        .select(Topic::as_select())
        .first(&mut *connection)
        .optional()
}

pub fn get_daily_tracks(
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    topic_id: Option<u16>,
) -> Result<Vec<DailyTrack>, DieselError> {
    let mut connection = DB_CONNECTION.lock().unwrap();

    let mut query = schema::daily_track::dsl::daily_track
        .select(DailyTrack::as_select())
        .into_boxed();

    if let Some(start) = start_date {
        let start_dt = start
            .and_hms_opt(0, 0, 0)
            .expect("valid start datetime");
        query = query.filter(schema::daily_track::start_time.ge(start_dt));
    }

    if let Some(end) = end_date {
        let end_dt = end
            .and_hms_opt(23, 59, 59)
            .expect("valid end datetime");
        query = query.filter(schema::daily_track::start_time.le(end_dt));
    }

    if let Some(tid) = topic_id {
        query = query.filter(schema::daily_track::topic_id.eq(u16_to_binary(tid)));
    }

    query.load(&mut *connection)
}

pub fn create_daily_track(
    start_time: NaiveDateTime,
    topic_id: Option<Vec<u8>>,
    comment: Option<String>,
) -> Result<DailyTrack, DieselError> {
    let mut connection = DB_CONNECTION.lock().unwrap();

    let existing: Option<DailyTrack> = schema::daily_track::dsl::daily_track
        .filter(schema::daily_track::start_time.eq(&start_time))
        .select(DailyTrack::as_select())
        .first(&mut *connection)
        .optional()?;

    if existing.is_some() {
        return Err(DieselError::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new(format!(
                "A record already exists for time period {}",
                start_time
            )),
        ));
    }

    let new_track = DailyTrack::new(start_time, topic_id, comment);

    diesel::insert_into(schema::daily_track::table)
        .values(&new_track)
        .execute(&mut *connection)?;

    Ok(new_track)
}

pub fn get_daily_track_by_id(id: i64) -> Result<Option<DailyTrack>, DieselError> {
    let mut connection = DB_CONNECTION.lock().unwrap();
    let binary_id = i64_to_binary(id);

    schema::daily_track::dsl::daily_track
        .filter(schema::daily_track::id.eq(binary_id))
        .select(DailyTrack::as_select())
        .first(&mut *connection)
        .optional()
}
