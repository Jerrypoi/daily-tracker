## Why

The swagger.json defines 6 REST API endpoints (3 for Topics, 3 for DailyTracks), but only a partial `get_topics` handler exists in `handler.rs`. The remaining 5 endpoints are unimplemented, and the existing one doesn't conform to the spec (uses `POST` with a JSON body instead of `GET` with query parameters). The full API surface needs to be implemented so the frontend can manage topics and daily track records.

## What Changes

- **Fix `GET /topics`**: Refactor existing handler to accept query parameters (`parent_topic_id`) instead of a JSON body, matching the swagger spec.
- **Add `POST /topics`**: Create handler to accept a `CreateTopicRequest` JSON body and persist a new topic.
- **Add `GET /topics/{id}`**: Create handler to retrieve a single topic by its ID.
- **Add `GET /daily-tracks`**: Create handler with query parameters for filtering by `start_date`, `end_date`, and `topic_id`.
- **Add `POST /daily-tracks`**: Create handler to accept a `CreateDailyTrackRequest` JSON body and persist a new daily track record.
- **Add `GET /daily-tracks/{id}`**: Create handler to retrieve a single daily track record by its ID.
- **Add API models**: Add missing request/response structs for all endpoints in the `models` crate (DailyTrack API model, request/response types).
- **Add DB query functions**: Add missing database operations in the `storage/db` crate (`create_topic`, `get_topic_by_id`, `get_daily_tracks`, `create_daily_track`, `get_daily_track_by_id`).
- **Add error handling**: Implement structured `ErrorResponse` JSON responses matching the swagger spec (400, 404, 409, 500).
- **Register routes**: Update `main.rs` to register all 6 routes with correct HTTP methods and paths under `/api/v1`.

## Capabilities

### New Capabilities
- `topic-api`: HTTP endpoint handlers for Topic CRUD operations (`GET /topics`, `POST /topics`, `GET /topics/{id}`), including query/path parameter extraction, request validation, error responses, and route registration.
- `daily-track-api`: HTTP endpoint handlers for DailyTrack CRUD operations (`GET /daily-tracks`, `POST /daily-tracks`, `GET /daily-tracks/{id}`), including date range filtering, time slot validation (`:00`/`:30`), error responses, and route registration.
- `api-error-handling`: Structured error response pattern using the `ErrorResponse` model, with proper HTTP status codes (400, 404, 409, 500) and consistent error codes/messages across all endpoints.

### Modified Capabilities
- `create-daily-track-persistence`: The existing spec covers DB insertion for daily tracks. This change extends it with HTTP-layer concerns (request parsing, validation that `start_time` is at `:00` or `:30`, conflict detection for duplicate time periods).

## Impact

- **Code**: `handler.rs`, `main.rs`, `models` crate (new structs), `storage/db` crate (new query functions), `db_model` (model conversions)
- **APIs**: 6 endpoints under `/api/v1` — this is net-new API surface (the existing `POST /get-topics` route changes to `GET /topics`)
- **Dependencies**: May need `axum::extract::Query` and `axum::extract::Path` for parameter extraction (already available in axum 0.7)
- **Database**: Read/write queries against existing `topic` and `daily_track` tables (no schema changes needed — migrations already exist)
