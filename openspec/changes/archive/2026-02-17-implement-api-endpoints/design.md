## Context

The backend is an Axum 0.7 server with a Diesel 2.2 MySQL ORM layer. The codebase is organized into workspace crates:

- `server_impl` — binary entry point, handlers, routing (currently one route: `POST /get-topics`)
- `models` — API-level request/response structs, with a `convert` module bridging DB models to API models
- `db` (storage/db) — query functions using a global `Mutex<MysqlConnection>`
- `db_model` — Diesel schema definitions and DB-level structs (`Topic`, `DailyTrack`)

Database tables use `BINARY(16)` UUID columns for IDs, while the API exposes `i64` IDs (first 8 bytes of the UUID, big-endian). The `DailyTrack` DB model and table already exist with all needed columns. Only `get_topics` has a DB query function; no DailyTrack queries exist yet.

## Goals / Non-Goals

**Goals:**
- Implement all 6 endpoints from `swagger.json` with correct HTTP methods, paths, and parameter handling
- Use RESTful conventions: `GET` with query/path params, `POST` with JSON bodies
- Return structured `ErrorResponse` JSON for all error cases (400, 404, 409, 500)
- Add all missing API models (DailyTrack, request/response types) and DB query functions
- Register routes under the `/api/v1` base path

**Non-Goals:**
- Connection pooling (keep the existing global `Mutex<MysqlConnection>` for now)
- Authentication/authorization (existing auth code is commented out; not in scope)
- Swagger UI integration (out of scope for this change)
- Pagination support (not in the swagger spec)
- Database schema changes (tables and migrations already exist)

## Decisions

### 1. Error handling: `ApiError` enum with `IntoResponse`

Create an `ApiError` enum in the `models` crate that maps to HTTP status codes and serializes as the swagger `ErrorResponse` (`{ "error": "...", "message": "..." }`). Implement axum's `IntoResponse` trait so handlers can return `Result<Json<T>, ApiError>`.

**Variants**: `BadRequest`, `NotFound`, `Conflict`, `InternalServerError`.

**Why over alternatives:**
- Returning bare `StatusCode` (current approach) loses error details — the swagger spec requires structured JSON error bodies.
- A trait object or anyhow-based approach adds complexity without benefit at this scale.

### 2. Parameter extraction: axum extractors

- `GET /topics?parent_topic_id=X` → `axum::extract::Query<GetTopicsParams>` with `Option<i64>` field
- `GET /topics/{id}` → `axum::extract::Path<i64>`
- `GET /daily-tracks?start_date=X&end_date=Y&topic_id=Z` → `axum::extract::Query<GetDailyTracksParams>` with all `Option` fields
- `GET /daily-tracks/{id}` → `axum::extract::Path<i64>`
- `POST` endpoints → `axum::Json<CreateTopicRequest>` / `axum::Json<CreateDailyTrackRequest>`

**Why**: This is idiomatic axum. Query params are deserialized directly into typed structs. Path params map to a single `i64`.

### 3. Refactor existing `get_topics` handler

The current handler uses `Json<GetTopicsRequest>` with a `BaseRequest` headers map. Refactor to use `Query<GetTopicsParams>` with just `parent_topic_id: Option<i64>`. Drop the `BaseResponse` wrapper from the response — return `Json<Vec<Topic>>` directly to match the swagger spec (which returns an array, not a wrapped object).

**Why**: The current implementation doesn't match the swagger spec. `GET` requests shouldn't have JSON bodies.

### 4. Route registration under `/api/v1`

Use axum's `Router::nest("/api/v1", api_routes)` to group all routes under the base path, then attach the request logger middleware at the outer level.

```
/api/v1
  GET  /topics             → get_topics
  POST /topics             → create_topic
  GET  /topics/:id         → get_topic_by_id
  GET  /daily-tracks       → get_daily_tracks
  POST /daily-tracks       → create_daily_track
  GET  /daily-tracks/:id   → get_daily_track_by_id
```

### 5. DB query layer: return `Result` instead of `unwrap`

Current `get_topics` calls `.unwrap()` on Diesel results, which panics on DB errors. All new (and refactored) DB functions will return `Result<T, diesel::result::Error>` so handlers can map DB errors to `ApiError::InternalServerError`.

### 6. DailyTrack API model and conversion

Add a `DailyTrack` API struct in the `models` crate (mirroring the existing `Topic` API struct) and a `db_daily_track_to_daily_track` conversion function in `convert.rs`. The same `vec_u8_to_i64` helper is reused for ID conversion.

### 7. DailyTrack time validation

The swagger spec requires `start_time` to be at `:00` or `:30` minutes. Validate this in the handler before calling the DB layer. Return `ApiError::BadRequest` with message `"start_time must be at :00 or :30 minutes"` if invalid.

### 8. Date filtering for `GET /daily-tracks`

Query params `start_date` and `end_date` are `Option<String>` in format `YYYY-MM-DD`. Parse them with `chrono::NaiveDate::parse_from_str`. The DB query uses Diesel's `.filter()` with `>=` / `<=` on the `start_time` column (comparing date portion). Return `ApiError::BadRequest` on invalid date formats.

## Risks / Trade-offs

- **Global Mutex connection**: All requests serialize on a single DB connection. This is fine for development but will bottleneck under load. → Mitigated by declaring connection pooling as a non-goal; can be addressed as a separate change.
- **ID truncation**: Converting 16-byte UUIDs to `i64` uses only the first 8 bytes, which loses uniqueness guarantees. → This is the existing pattern and changing it is out of scope. All existing code relies on this convention.
- **Endianness inconsistency**: `convert.rs` uses `i64::from_be_bytes` for reading IDs, but `db.rs::i64_to_binary` uses `to_le_bytes` for writing. → Fix `i64_to_binary` to use `to_be_bytes` for consistency. This is a latent bug that would cause lookups by ID to fail.
- **No transaction support**: Create operations are single inserts without explicit transactions. → Acceptable for single-row inserts; revisit if batch operations are added later.
