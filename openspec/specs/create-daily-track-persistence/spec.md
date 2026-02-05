# create-daily-track-persistence Specification

## Purpose
TBD - created by archiving change implement-daily-track-insertion. Update Purpose after archive.
## Requirements
### Requirement: Persist Daily Track Records

When a daily track record is created via the API, the system SHALL persist it to the database and return the created record with its actual database ID.

#### Scenario: Successful Creation

- **WHEN** a valid `CreateDailyTrackRequest` is received with `start_time`, `topic_id`, and optional `comment`
- **THEN** a new `DailyTrack` record is inserted into the `daily_track` table
- **AND** the record is assigned a unique UUID-based ID
- **AND** `created_at` is set to the current timestamp
- **AND** `updated_at` is initially set to `None` (or current timestamp if required)
- **AND** the response contains the created `DailyTrack` with the actual database ID

#### Scenario: Database Error Handling

- **WHEN** a database error occurs during insertion (e.g., connection failure, constraint violation)
- **THEN** an appropriate `ApiError` is returned
- **AND** no partial record is created in the database

#### Scenario: Type Conversion

- **WHEN** converting between API model types (`i64` topic_id) and database model types (`Vec<u8>` UUID)
- **THEN** the conversion preserves data integrity
- **AND** invalid conversions result in appropriate error responses

