# daily-track-api Specification

## Purpose
HTTP endpoint handlers for DailyTrack CRUD operations (GET /daily-tracks, POST /daily-tracks, GET /daily-tracks/{id}), including date range filtering, time slot validation (:00/:30), error responses, and route registration.

## Requirements

### Requirement: List daily track records

The system SHALL expose a `GET /api/v1/daily-tracks` endpoint that returns daily track records as a JSON array. It SHALL accept optional query parameters `start_date`, `end_date`, and `topic_id` for filtering.

#### Scenario: List all daily tracks without filters

- **WHEN** a `GET /api/v1/daily-tracks` request is received with no query parameters
- **THEN** the system SHALL return HTTP 200 with a JSON array of all `DailyTrack` objects
- **AND** each `DailyTrack` SHALL include `id`, `start_time`, `created_at`, `updated_at`, `topic_id`, and optionally `comment`

#### Scenario: Filter by date range

- **WHEN** a `GET /api/v1/daily-tracks?start_date=2026-01-01&end_date=2026-01-31` request is received
- **THEN** the system SHALL return HTTP 200 with only records whose `start_time` falls within the inclusive date range

#### Scenario: Filter by topic

- **WHEN** a `GET /api/v1/daily-tracks?topic_id=42` request is received
- **THEN** the system SHALL return HTTP 200 with only records whose `topic_id` matches the provided value

#### Scenario: Combined filters

- **WHEN** a `GET /api/v1/daily-tracks` request is received with multiple filter parameters
- **THEN** the system SHALL apply all provided filters using AND logic

#### Scenario: Invalid date format

- **WHEN** a `GET /api/v1/daily-tracks` request is received with a `start_date` or `end_date` that is not in `YYYY-MM-DD` format
- **THEN** the system SHALL return HTTP 400 with an `ErrorResponse` containing error code `VALIDATION_ERROR`

#### Scenario: Empty result

- **WHEN** a `GET /api/v1/daily-tracks` request is received and no records match the filters
- **THEN** the system SHALL return HTTP 200 with an empty JSON array `[]`

#### Scenario: Database error during list

- **WHEN** a database error occurs while retrieving daily track records
- **THEN** the system SHALL return HTTP 500 with an `ErrorResponse` body

### Requirement: Create daily track record

The system SHALL expose a `POST /api/v1/daily-tracks` endpoint that creates a new daily track record from a JSON request body containing `start_time`, `topic_id`, and an optional `comment`.

#### Scenario: Successful creation

- **WHEN** a `POST /api/v1/daily-tracks` request is received with a valid `CreateDailyTrackRequest`
- **AND** `start_time` is at `:00` or `:30` minutes of an hour
- **THEN** the system SHALL insert a new `DailyTrack` record into the database
- **AND** the record SHALL be assigned a unique UUID-based ID
- **AND** `created_at` SHALL be set to the current timestamp
- **AND** the system SHALL return HTTP 201 with the created `DailyTrack` as JSON

#### Scenario: Invalid start time

- **WHEN** a `POST /api/v1/daily-tracks` request is received with a `start_time` whose minutes are not `:00` or `:30`
- **THEN** the system SHALL return HTTP 400 with an `ErrorResponse` containing message `"start_time must be at :00 or :30 minutes"`

#### Scenario: Referenced topic not found

- **WHEN** a `POST /api/v1/daily-tracks` request is received with a `topic_id` that does not exist in the `topic` table
- **THEN** the system SHALL return HTTP 404 with an `ErrorResponse` containing error code `NOT_FOUND`

#### Scenario: Duplicate time period

- **WHEN** a `POST /api/v1/daily-tracks` request is received with a `start_time` for which a record already exists
- **THEN** the system SHALL return HTTP 409 with an `ErrorResponse` containing error code `CONFLICT`

#### Scenario: Database error during creation

- **WHEN** a database error occurs while inserting the daily track record
- **THEN** the system SHALL return HTTP 500 with an `ErrorResponse` body

### Requirement: Get daily track record by ID

The system SHALL expose a `GET /api/v1/daily-tracks/{id}` endpoint that returns a single daily track record by its ID.

#### Scenario: Record found

- **WHEN** a `GET /api/v1/daily-tracks/{id}` request is received with a valid ID that exists in the database
- **THEN** the system SHALL return HTTP 200 with the matching `DailyTrack` as JSON

#### Scenario: Record not found

- **WHEN** a `GET /api/v1/daily-tracks/{id}` request is received with an ID that does not exist
- **THEN** the system SHALL return HTTP 404 with an `ErrorResponse` containing error code `NOT_FOUND`

#### Scenario: Database error during lookup

- **WHEN** a database error occurs while retrieving the daily track record
- **THEN** the system SHALL return HTTP 500 with an `ErrorResponse` body
