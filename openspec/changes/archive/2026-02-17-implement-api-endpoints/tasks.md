## 1. Error Handling Foundation

- [x] 1.1 Add `ErrorResponse` struct to `models` crate with `error` and `message` fields (Serialize, Deserialize)
- [x] 1.2 Add `ApiError` enum to `models` crate with variants: `BadRequest(String)`, `NotFound(String)`, `Conflict(String)`, `InternalServerError(String)`
- [x] 1.3 Implement `axum::response::IntoResponse` for `ApiError`, mapping each variant to the correct HTTP status code and `ErrorResponse` JSON body
- [x] 1.4 Add `axum` as a dependency to the `models` crate Cargo.toml (needed for `IntoResponse` impl)

## 2. API Models

- [x] 2.1 Add `DailyTrack` API struct to `models` crate with fields: `id: i64`, `start_time: DateTime<Utc>`, `created_at: DateTime<Utc>`, `updated_at: DateTime<Utc>`, `topic_id: i64`, `comment: Option<String>`
- [x] 2.2 Add `CreateTopicRequest` struct with `topic_name: String` and `parent_topic_id: Option<i64>`
- [x] 2.3 Add `CreateDailyTrackRequest` struct with `start_time: DateTime<Utc>`, `topic_id: i64`, `comment: Option<String>`
- [x] 2.4 Add `GetTopicsParams` struct (for Query extraction) with `parent_topic_id: Option<i64>`
- [x] 2.5 Add `GetDailyTracksParams` struct (for Query extraction) with `start_date: Option<String>`, `end_date: Option<String>`, `topic_id: Option<i64>`
- [x] 2.6 Add `db_daily_track_to_daily_track` conversion function in `convert.rs` using existing `vec_u8_to_i64` helper
- [x] 2.7 Remove or deprecate old `GetTopicsRequest`, `GetTopicsResponse`, `BaseRequest`, `BaseResponse` types that no longer match the swagger spec

## 3. Bug Fix and DB Query Layer

- [x] 3.1 Fix `i64_to_binary` in `storage/db/src/db.rs` to use `to_be_bytes()` instead of `to_le_bytes()` for endianness consistency with `convert.rs`
- [x] 3.2 Refactor `get_topics` to return `Result<Vec<Topic>, diesel::result::Error>` instead of unwrapping
- [x] 3.3 Add `create_topic(topic_name: String, parent_topic_id: Option<i64>) -> Result<Topic, diesel::result::Error>` to `db` crate
- [x] 3.4 Add `get_topic_by_id(id: i64) -> Result<Option<Topic>, diesel::result::Error>` to `db` crate
- [x] 3.5 Add `get_daily_tracks(start_date: Option<NaiveDate>, end_date: Option<NaiveDate>, topic_id: Option<i64>) -> Result<Vec<DailyTrack>, diesel::result::Error>` to `db` crate
- [x] 3.6 Add `create_daily_track(start_time: NaiveDateTime, topic_id: Option<Vec<u8>>, comment: Option<String>) -> Result<DailyTrack, diesel::result::Error>` to `db` crate
- [x] 3.7 Add `get_daily_track_by_id(id: i64) -> Result<Option<DailyTrack>, diesel::result::Error>` to `db` crate

## 4. Topic Endpoint Handlers

- [x] 4.1 Refactor `get_topics` handler to use `Query<GetTopicsParams>` extractor and return `Result<Json<Vec<Topic>>, ApiError>`
- [x] 4.2 Implement `create_topic` handler: accept `Json<CreateTopicRequest>`, validate `topic_name` is non-empty, call `db::create_topic`, return HTTP 201 with created `Topic` or appropriate `ApiError`
- [x] 4.3 Implement `get_topic_by_id` handler: accept `Path<i64>`, call `db::get_topic_by_id`, return `Topic` or `ApiError::NotFound`

## 5. DailyTrack Endpoint Handlers

- [x] 5.1 Implement `get_daily_tracks` handler: accept `Query<GetDailyTracksParams>`, parse and validate date strings, call `db::get_daily_tracks`, return `Vec<DailyTrack>` or `ApiError::BadRequest` for invalid dates
- [x] 5.2 Implement `create_daily_track` handler: accept `Json<CreateDailyTrackRequest>`, validate `start_time` is at `:00` or `:30`, call `db::create_daily_track`, return HTTP 201 with created `DailyTrack` or appropriate `ApiError`
- [x] 5.3 Implement `get_daily_track_by_id` handler: accept `Path<i64>`, call `db::get_daily_track_by_id`, return `DailyTrack` or `ApiError::NotFound`

## 6. Route Registration

- [x] 6.1 Update `register_routes()` in `main.rs` to use `Router::nest("/api/v1", ...)` with all 6 routes using correct HTTP methods (`get`, `post`) and paths (`/topics`, `/topics/:id`, `/daily-tracks`, `/daily-tracks/:id`)
- [x] 6.2 Remove the old `POST /get-topics` route
