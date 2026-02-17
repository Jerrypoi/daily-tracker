use db_model::establish_connection;
use diesel::MysqlConnection;
use diesel::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static DB_CONNECTION: Lazy<Mutex<MysqlConnection>> =
    Lazy::new(|| Mutex::new(establish_connection()));

pub fn i64_to_binary(i64: i64) -> Vec<u8> {
    i64.to_le_bytes().to_vec()
}

pub fn get_topics(parent_topic_id: Option<i64>) -> Vec<db_model::models::Topic> {
    let mut connection = DB_CONNECTION.lock().unwrap();

    if let Some(parent_topic_id) = parent_topic_id {
        db_model::schema::topic::dsl::topic
            .filter(db_model::schema::topic::parent_topic_id.eq(i64_to_binary(parent_topic_id)))
            .select(db_model::models::Topic::as_select())
            .load(&mut *connection)
            .unwrap()
    } else {
        db_model::schema::topic::dsl::topic
            .select(db_model::models::Topic::as_select())
            .load(&mut *connection)
            .unwrap()
    }
}
