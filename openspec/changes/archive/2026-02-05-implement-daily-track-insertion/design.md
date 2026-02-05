## Context

The `create_daily_track` endpoint currently returns hardcoded data. The `Server` struct already has a database connection (`Arc<Mutex<MysqlConnection>>`), and the `DailyTrack` model has a `new()` method that creates instances with UUIDs. The Diesel schema is defined and ready to use.

## Goals / Non-Goals

**Goals:**
- Insert new daily track records into the database using Diesel
- Return the created record with its actual UUID-based ID
- Handle database errors gracefully

**Non-Goals:**
- Changing the API contract or response format
- Implementing validation beyond what's already in the request model
- Optimizing for performance (this is a straightforward insertion)

## Decisions

### Decision 1: Use Diesel's `insert_into` with `get_result`

We'll use Diesel's standard insertion pattern:
1. Create a `DailyTrack` instance using the `new()` method (which generates UUID)
2. Use `diesel::insert_into(daily_track::table).values(&new_record).execute(&conn)` to insert
3. Since we need the inserted record back, we may need to query it after insertion, or use `get_result` if Diesel supports it for MySQL

### Decision 2: Handle Type Conversions

The API model uses `i64` for `topic_id`, but the database uses `Vec<u8>` (UUID). We'll need to:
- Convert `i64` topic_id from the request to `Vec<u8>` for database insertion (or handle this mismatch)
- Convert the database `Vec<u8>` ID back to `i64` for the API response (or handle this mismatch)

**Note:** There's a type mismatch that needs investigation - the API expects `i64` but the DB uses UUIDs. This may require a conversion strategy or the API model may need adjustment. For now, we'll implement the database insertion and handle conversions as needed.

### Decision 3: Error Handling

Database errors will be caught and converted to `ApiError`. We'll use the existing error handling pattern in the codebase.

### Decision 4: Thread Safety

The connection is wrapped in `Arc<Mutex<MysqlConnection>>`, so we'll lock it before performing database operations to ensure thread safety.
