use chrono::{NaiveDate, NaiveDateTime};
use db_model::models::{
    ApiKey, DailyTrack, NewApiKey, NewDailyTrack, NewTopic, NewUser, Topic, User,
};
use db_model::schema;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::Error as DieselError;
use diesel::MysqlConnection;
use once_cell::sync::Lazy;
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::time::Duration;
use utils::snowflake::generate_snowflake_id;
use uuid::Uuid;

pub const API_KEY_PREFIX: &str = "dt_";

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<MysqlConnection>>;

pub static DB_POOL: Lazy<DbPool> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::builder()
        .max_size(8)
        .min_idle(Some(1))
        .connection_timeout(Duration::from_secs(10))
        .idle_timeout(Some(Duration::from_secs(300)))
        .max_lifetime(Some(Duration::from_secs(1800)))
        .build(ConnectionManager::<MysqlConnection>::new(url))
        .expect("failed to build MySQL connection pool")
});

pub fn get_topics(
    parent_topic_id: Option<i64>,
    user_id: Option<i64>,
) -> Result<Vec<Topic>, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    let mut query = schema::topic::dsl::topic
        .select(Topic::as_select())
        .into_boxed();
    if let Some(uid) = user_id {
        query = query.filter(schema::topic::dsl::user_id.eq(uid));
    }
    if let Some(parent_id) = parent_topic_id {
        query = query.filter(schema::topic::dsl::parent_topic_id.eq(parent_id));
    }

    query.load(&mut *connection)
}

pub fn create_topic(
    topic_name: String,
    display_color: String,
    parent_topic_id: Option<i64>,
    user_id: Option<i64>,
) -> Result<Topic, DieselError> {
    let mut connection = DB_POOL.get().unwrap();

    let mut query = schema::topic::dsl::topic
        .filter(schema::topic::topic_name.eq(&topic_name))
        .into_boxed();
    if let Some(ref uid) = user_id {
        query = query.filter(schema::topic::user_id.eq(uid.clone()));
    }
    let existing: Option<Topic> = query
        .select(Topic::as_select())
        .first(&mut *connection)
        .optional()?;

    if existing.is_some() {
        return Err(DieselError::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new(format!("Topic with name '{}' already exists", topic_name)),
        ));
    }

    let now = chrono::Utc::now().naive_utc();
    let id = generate_snowflake_id();

    let new_topic = NewTopic {
        id,
        topic_name,
        display_color,
        created_at: now,
        updated_at: None,
        parent_topic_id,
        user_id,
    };

    diesel::insert_into(schema::topic::table)
        .values(&new_topic)
        .execute(&mut *connection)?;

    schema::topic::dsl::topic
        .find(id)
        .select(Topic::as_select())
        .first(&mut *connection)
}

pub fn update_topic(
    id: i64,
    topic_name: String,
    display_color: String,
    user_id: i64,
) -> Result<Option<Topic>, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    let Some(existing_topic) = schema::topic::dsl::topic
        .filter(schema::topic::dsl::id.eq(id))
        .filter(schema::topic::dsl::user_id.eq(user_id))
        .select(Topic::as_select())
        .first(&mut *connection)
        .optional()?
    else {
        return Ok(None);
    };

    let duplicate_name_exists = schema::topic::dsl::topic
        .filter(schema::topic::dsl::user_id.eq(user_id))
        .filter(schema::topic::dsl::topic_name.eq(&topic_name))
        .filter(schema::topic::dsl::id.ne(id))
        .select(Topic::as_select())
        .first(&mut *connection)
        .optional()?
        .is_some();
    if duplicate_name_exists {
        return Err(DieselError::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new(format!("Topic with name '{}' already exists", topic_name)),
        ));
    }

    diesel::update(schema::topic::dsl::topic.find(existing_topic.id))
        .set((
            schema::topic::dsl::topic_name.eq(topic_name),
            schema::topic::dsl::display_color.eq(display_color),
            schema::topic::dsl::updated_at.eq(Some(chrono::Utc::now().naive_utc())),
        ))
        .execute(&mut *connection)?;

    schema::topic::dsl::topic
        .find(existing_topic.id)
        .select(Topic::as_select())
        .first(&mut *connection)
        .optional()
}

pub fn get_topic_by_id(id: i64) -> Result<Option<Topic>, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    schema::topic::dsl::topic
        .find(id)
        .select(Topic::as_select())
        .first(&mut *connection)
        .optional()
}

pub fn get_topic_by_id_for_user(id: i64, user_id: i64) -> Result<Option<Topic>, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    schema::topic::dsl::topic
        .filter(schema::topic::dsl::id.eq(id))
        .filter(schema::topic::dsl::user_id.eq(user_id))
        .select(Topic::as_select())
        .first(&mut *connection)
        .optional()
}

pub fn get_daily_tracks(
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    topic_id: Option<i64>,
    user_id: Option<i64>,
) -> Result<Vec<DailyTrack>, DieselError> {
    let mut connection = DB_POOL.get().unwrap();

    let mut query = schema::daily_track::dsl::daily_track
        .select(DailyTrack::as_select())
        .into_boxed();
    if let Some(uid) = user_id {
        query = query.filter(schema::daily_track::dsl::user_id.eq(uid));
    }

    if let Some(start) = start_date {
        let start_dt = start.and_hms_opt(0, 0, 0).expect("valid start datetime");
        query = query.filter(schema::daily_track::start_time.ge(start_dt));
    }

    if let Some(end) = end_date {
        let end_dt = end.and_hms_opt(23, 59, 59).expect("valid end datetime");
        query = query.filter(schema::daily_track::start_time.le(end_dt));
    }

    if let Some(tid) = topic_id {
        query = query.filter(schema::daily_track::topic_id.eq(tid));
    }
    query.load(&mut *connection)
}

pub fn create_daily_track(
    start_time: NaiveDateTime,
    topic_id: Option<i64>,
    comment: Option<String>,
    user_id: Option<i64>,
) -> Result<DailyTrack, DieselError> {
    let mut connection = DB_POOL.get().unwrap();

    let mut query = schema::daily_track::dsl::daily_track
        .filter(schema::daily_track::start_time.eq(&start_time))
        .into_boxed();
    if let Some(ref uid) = user_id {
        query = query.filter(schema::daily_track::user_id.eq(uid.clone()));
    }
    let existing: Option<DailyTrack> = query
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

    let id = generate_snowflake_id();
    let new_track = NewDailyTrack::new(id, start_time, topic_id, comment, user_id);

    diesel::insert_into(schema::daily_track::table)
        .values(&new_track)
        .execute(&mut *connection)?;

    schema::daily_track::dsl::daily_track
        .find(id)
        .select(DailyTrack::as_select())
        .first(&mut *connection)
}

pub fn get_daily_track_by_id(id: i64, user_id: i64) -> Result<Option<DailyTrack>, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    schema::daily_track::dsl::daily_track
        .filter(schema::daily_track::dsl::id.eq(id))
        .filter(schema::daily_track::dsl::user_id.eq(user_id))
        .select(DailyTrack::as_select())
        .first(&mut *connection)
        .optional()
}

pub fn update_daily_track(
    id: i64,
    topic_id: i64,
    comment: Option<String>,
    user_id: i64,
) -> Result<Option<DailyTrack>, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    let Some(existing_track) = schema::daily_track::dsl::daily_track
        .filter(schema::daily_track::dsl::id.eq(id))
        .filter(schema::daily_track::dsl::user_id.eq(user_id))
        .select(DailyTrack::as_select())
        .first(&mut *connection)
        .optional()?
    else {
        return Ok(None);
    };

    diesel::update(schema::daily_track::dsl::daily_track.find(existing_track.id))
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

pub fn delete_daily_track(id: i64, user_id: i64) -> Result<bool, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    let Some(track) = schema::daily_track::dsl::daily_track
        .filter(schema::daily_track::dsl::id.eq(id))
        .filter(schema::daily_track::dsl::user_id.eq(user_id))
        .select(DailyTrack::as_select())
        .first(&mut *connection)
        .optional()?
    else {
        return Ok(false);
    };

    let deleted = diesel::delete(schema::daily_track::dsl::daily_track.find(track.id))
        .execute(&mut *connection)?;
    Ok(deleted > 0)
}

fn generate_verification_code() -> String {
    // Derive a 6-digit numeric code from a random UUID
    let bytes = Uuid::new_v4().into_bytes();
    let n = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    format!("{:06}", n % 1_000_000)
}

pub fn create_user(
    username: String,
    email: String,
    password_hash: String,
) -> Result<(User, String), DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    let now = chrono::Utc::now().naive_utc();
    let id = generate_snowflake_id();
    let code = generate_verification_code();
    let expires_at = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(30))
        .expect("valid expiry timestamp")
        .naive_utc();

    let new_user = NewUser {
        id,
        username,
        email,
        password_hash,
        email_verified: false,
        verification_code: Some(code.clone()),
        verification_code_expires_at: Some(expires_at),
        created_at: now,
        updated_at: None,
    };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(&mut *connection)?;
    let user = schema::users::dsl::users
        .find(id)
        .select(User::as_select())
        .first(&mut *connection)?;

    Ok((user, code))
}

pub fn get_user_by_username(username: &str) -> Result<Option<User>, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    schema::users::dsl::users
        .filter(schema::users::username.eq(username))
        .select(User::as_select())
        .first(&mut *connection)
        .optional()
}

pub fn get_user_by_email(email: &str) -> Result<Option<User>, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    schema::users::dsl::users
        .filter(schema::users::email.eq(email))
        .select(User::as_select())
        .first(&mut *connection)
        .optional()
}

/// Validates `code` against the stored verification code for `email`.
/// On success, marks the user as verified and clears the code. Returns
/// `false` when the code is wrong or expired without touching the database.
pub fn verify_email_code(email: &str, code: &str) -> Result<bool, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    let now = chrono::Utc::now().naive_utc();

    let user: Option<User> = schema::users::dsl::users
        .filter(schema::users::email.eq(email))
        .select(User::as_select())
        .first(&mut *connection)
        .optional()?;

    let Some(user) = user else {
        return Ok(false);
    };

    if user.email_verified {
        return Ok(true); // already verified — idempotent
    }

    let stored_code = match user.verification_code.as_ref() {
        Some(c) => c,
        None => return Ok(false),
    };

    let expires_at = match user.verification_code_expires_at {
        Some(t) => t,
        None => return Ok(false),
    };

    if stored_code != code || now > expires_at {
        return Ok(false);
    }

    // Mark as verified and remove the one-time code
    diesel::update(schema::users::dsl::users.find(user.id))
        .set((
            schema::users::dsl::email_verified.eq(true),
            schema::users::dsl::verification_code.eq::<Option<String>>(None),
            schema::users::dsl::verification_code_expires_at.eq::<Option<NaiveDateTime>>(None),
            schema::users::dsl::updated_at.eq(Some(now)),
        ))
        .execute(&mut *connection)?;

    Ok(true)
}

pub fn hash_api_key(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    hex::encode(digest)
}

/// Generates a new random API key token of the form `dt_<32 hex chars>`.
pub fn generate_api_key_token() -> String {
    let mut bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    format!("{}{}", API_KEY_PREFIX, hex::encode(bytes))
}

/// Inserts a new API key for `user_id` and returns (record, plaintext token).
/// The plaintext token is only available at creation time.
pub fn create_api_key(user_id: i64, name: String) -> Result<(ApiKey, String), DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    let token = generate_api_key_token();
    let key_hash = hash_api_key(&token);
    // Show first 10 chars (e.g. "dt_abcdef1") so users can identify keys later.
    let key_prefix: String = token.chars().take(10).collect();

    let now = chrono::Utc::now().naive_utc();
    let id = generate_snowflake_id();

    let record = NewApiKey {
        id,
        user_id,
        key_hash,
        key_prefix,
        name,
        created_at: now,
        last_used_at: None,
        revoked_at: None,
    };

    diesel::insert_into(schema::api_keys::table)
        .values(&record)
        .execute(&mut *connection)?;
    let record = schema::api_keys::dsl::api_keys
        .find(id)
        .select(ApiKey::as_select())
        .first(&mut *connection)?;

    Ok((record, token))
}

pub fn list_api_keys_for_user(user_id: i64) -> Result<Vec<ApiKey>, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    schema::api_keys::dsl::api_keys
        .filter(schema::api_keys::dsl::user_id.eq(user_id))
        .filter(schema::api_keys::dsl::revoked_at.is_null())
        .select(ApiKey::as_select())
        .load(&mut *connection)
}

/// Looks up an active API key by its plaintext token. Bumps `last_used_at` on
/// success. Returns the owning `user_id` when valid.
pub fn lookup_api_key(token: &str) -> Result<Option<i64>, DieselError> {
    let key_hash = hash_api_key(token);
    let mut connection = DB_POOL.get().unwrap();
    let key: Option<ApiKey> = schema::api_keys::dsl::api_keys
        .filter(schema::api_keys::dsl::key_hash.eq(&key_hash))
        .filter(schema::api_keys::dsl::revoked_at.is_null())
        .select(ApiKey::as_select())
        .first(&mut *connection)
        .optional()?;

    let Some(key) = key else { return Ok(None) };

    let now = chrono::Utc::now().naive_utc();
    let _ = diesel::update(schema::api_keys::dsl::api_keys.find(key.id))
        .set(schema::api_keys::dsl::last_used_at.eq(Some(now)))
        .execute(&mut *connection);

    Ok(Some(key.user_id))
}

/// Marks the user's API key as revoked. Returns
/// `true` when a row was updated.
pub fn revoke_api_key(id: i64, user_id: i64) -> Result<bool, DieselError> {
    let mut connection = DB_POOL.get().unwrap();
    let Some(key) = schema::api_keys::dsl::api_keys
        .filter(schema::api_keys::dsl::id.eq(id))
        .filter(schema::api_keys::dsl::user_id.eq(user_id))
        .filter(schema::api_keys::dsl::revoked_at.is_null())
        .select(ApiKey::as_select())
        .first(&mut *connection)
        .optional()?
    else {
        return Ok(false);
    };

    let now = chrono::Utc::now().naive_utc();
    let updated = diesel::update(schema::api_keys::dsl::api_keys.find(key.id))
        .set(schema::api_keys::dsl::revoked_at.eq(Some(now)))
        .execute(&mut *connection)?;

    Ok(updated > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- generate_verification_code tests ---

    #[test]
    fn verification_code_is_six_digits() {
        let code = generate_verification_code();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn verification_code_within_range() {
        // Run multiple times to check range
        for _ in 0..100 {
            let code = generate_verification_code();
            let num: u32 = code.parse().unwrap();
            assert!(num < 1_000_000);
        }
    }

    #[test]
    fn verification_codes_are_not_all_same() {
        // Statistical: generate a few and ensure they're not all identical
        let codes: Vec<String> = (0..10).map(|_| generate_verification_code()).collect();
        let first = &codes[0];
        assert!(
            codes.iter().any(|c| c != first),
            "all 10 generated codes were identical: {}",
            first
        );
    }

    // --- API key helper tests ---

    #[test]
    fn hash_api_key_is_deterministic() {
        let token = "dt_abcdef1234567890abcdef1234567890";
        assert_eq!(hash_api_key(token), hash_api_key(token));
    }

    #[test]
    fn hash_api_key_is_64_hex_chars() {
        let hash = hash_api_key("dt_test");
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn hash_api_key_differs_for_different_inputs() {
        assert_ne!(hash_api_key("dt_a"), hash_api_key("dt_b"));
    }

    #[test]
    fn hash_api_key_matches_known_sha256() {
        // sha256("") in hex
        assert_eq!(
            hash_api_key(""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn generate_api_key_token_has_dt_prefix() {
        let token = generate_api_key_token();
        assert!(token.starts_with(API_KEY_PREFIX));
    }

    #[test]
    fn generate_api_key_token_length() {
        // "dt_" (3) + 16 random bytes hex-encoded (32) = 35 chars
        let token = generate_api_key_token();
        assert_eq!(token.len(), 3 + 32);
    }

    #[test]
    fn generate_api_key_token_is_hex_after_prefix() {
        let token = generate_api_key_token();
        let body = token.strip_prefix(API_KEY_PREFIX).unwrap();
        assert!(body.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn generate_api_key_tokens_are_unique() {
        // Statistical: 100 random tokens should not all collide.
        let tokens: Vec<String> = (0..100).map(|_| generate_api_key_token()).collect();
        let mut sorted = tokens.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), tokens.len(), "duplicate token generated");
    }

    #[test]
    fn api_key_prefix_constant() {
        assert_eq!(API_KEY_PREFIX, "dt_");
    }
}
