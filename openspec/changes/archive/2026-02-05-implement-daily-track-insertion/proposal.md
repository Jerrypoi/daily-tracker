## Why

The `create_daily_track` endpoint currently returns a hardcoded response with `id: 0` instead of persisting records to the database. This prevents the daily tracking functionality from working and makes the API non-functional for its primary use case.

## What Changes

- Replace the hardcoded response with a real database insertion using Diesel
- Return the created record with its actual database ID (UUID)
- Handle proper error cases (database errors, connection issues)
- Convert between API model types (`i64` topic_id) and database model types (`Vec<u8>` UUID) as needed

## Capabilities

### New Capabilities
- `create-daily-track-persistence`: The API endpoint now persists daily track records to the database and returns the created record with its real UUID-based ID

### Modified Capabilities
<!-- No existing capability requirements are changing - this is implementing missing functionality -->

## Impact

- `backend/crates/server_impl/server.rs`: Implement database insertion logic in `create_daily_track` method
- May require type conversion utilities between API models (`i64` topic_id) and database models (`Vec<u8>` UUID)
