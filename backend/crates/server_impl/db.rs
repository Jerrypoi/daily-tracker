use db_model::establish_connection;
use diesel::MysqlConnection;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use diesel::prelude::*;

pub static DB_CONNECTION: Lazy<Mutex<MysqlConnection>> = Lazy::new(|| Mutex::new(establish_connection()));


pub fn get_topics( parent_topic_id: Option<i64>) -> Vec<db_model::models::Topic> {
    let connection = DB_CONNECTION.lock().unwrap();
    let resuts = db_model::schema::topic::dsl::topic
    // .filter(db_model::schema::topic::parent_topic_id.eq(parent_topic_id))
    .select(db_model::models::Topic::as_select())
    .load(connection)
    .unwrap();
    resuts
}
