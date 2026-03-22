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

fn binary_to_u16(bytes: &[u8]) -> Option<u16> {
    if bytes.len() < 2 {
        return None;
    }
    Some(u16::from_be_bytes([bytes[0], bytes[1]]))
}

pub fn get_topics(parent_topic_id: Option<u16>) -> Result<Vec<Topic>, DieselError> {
    let mut connection = DB_CONNECTION.lock().unwrap();
    let mut topics: Vec<Topic> = schema::topic::dsl::topic
        .select(Topic::as_select())
        .load(&mut *connection)?;

    if let Some(parent_id) = parent_topic_id {
        topics.retain(|t| {
            t.parent_topic_id
                .as_deref()
                .and_then(binary_to_u16)
                .map(|pid| pid == parent_id)
                .unwrap_or(false)
        });
    }

    Ok(topics)
}

pub fn create_topic(
    topic_name: String,
    parent_topic_id: Option<Vec<u8>>,
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

    let new_topic = Topic {
        id: id.clone(),
        topic_name,
        created_at: now,
        updated_at: None,
        parent_topic_id,
    };

    diesel::insert_into(schema::topic::table)
        .values(&new_topic)
        .execute(&mut *connection)?;

    Ok(new_topic)
}

pub fn get_topic_by_id(id: u16) -> Result<Option<Topic>, DieselError> {
    let mut connection = DB_CONNECTION.lock().unwrap();
    let topics: Vec<Topic> = schema::topic::dsl::topic
        .select(Topic::as_select())
        .load(&mut *connection)?;

    Ok(topics.into_iter().find(|t| {
        binary_to_u16(&t.id).map(|public_id| public_id == id).unwrap_or(false)
    }))
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

    if topic_id.is_some() {
        query = query.filter(schema::daily_track::topic_id.is_not_null());
    }
    let mut tracks: Vec<DailyTrack> = query.load(&mut *connection)?;
    if let Some(tid) = topic_id {
        tracks.retain(|t| {
            t.topic_id
                .as_deref()
                .and_then(binary_to_u16)
                .map(|id| id == tid)
                .unwrap_or(false)
        });
    }
    Ok(tracks)
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
    let tracks: Vec<DailyTrack> = schema::daily_track::dsl::daily_track
        .select(DailyTrack::as_select())
        .load(&mut *connection)?;

    Ok(tracks.into_iter().find(|t| {
        binary_to_u16(&t.id).map(|public_id| public_id == id as u16).unwrap_or(false)
    }))
}

pub fn update_daily_track(
    id: u16,
    topic_id: Vec<u8>,
    comment: Option<String>,
) -> Result<Option<DailyTrack>, DieselError> {
    let mut connection = DB_CONNECTION.lock().unwrap();
    let tracks: Vec<DailyTrack> = schema::daily_track::dsl::daily_track
        .select(DailyTrack::as_select())
        .load(&mut *connection)?;
    let Some(existing_track) = tracks.into_iter().find(|t| {
        binary_to_u16(&t.id).map(|public_id| public_id == id).unwrap_or(false)
    }) else {
        return Ok(None);
    };

    diesel::update(schema::daily_track::dsl::daily_track.find(existing_track.id.clone()))
        .set((
            schema::daily_track::dsl::topic_id.eq(Some(topic_id)),
            schema::daily_track::dsl::comment.eq(comment),
            schema::daily_track::dsl::updated_at.eq(Some(chrono::Utc::now().naive_utc())),
        ))
        .execute(&mut *connection)?;

    schema::daily_track::dsl::daily_track
        .find(existing_track.id)
        .select(DailyTrack::as_select())
        .first(&mut *connection)
        .optional()
}

pub fn delete_daily_track(id: u16) -> Result<bool, DieselError> {
    let mut connection = DB_CONNECTION.lock().unwrap();
    let tracks: Vec<DailyTrack> = schema::daily_track::dsl::daily_track
        .select(DailyTrack::as_select())
        .load(&mut *connection)?;
    let Some(track) = tracks.into_iter().find(|t| {
        binary_to_u16(&t.id).map(|public_id| public_id == id).unwrap_or(false)
    }) else {
        return Ok(false);
    };

    let deleted = diesel::delete(schema::daily_track::dsl::daily_track.find(track.id))
        .execute(&mut *connection)?;
    Ok(deleted > 0)
}
