## ADDED Requirements

### Requirement: List topics

The system SHALL expose a `GET /api/v1/topics` endpoint that returns all topics as a JSON array. It SHALL accept an optional `parent_topic_id` query parameter to filter topics by parent.

#### Scenario: List all topics without filter

- **WHEN** a `GET /api/v1/topics` request is received with no query parameters
- **THEN** the system SHALL return HTTP 200 with a JSON array of all `Topic` objects
- **AND** each `Topic` SHALL include `id`, `topic_name`, `created_at`, `updated_at`, and optionally `parent_topic_id`

#### Scenario: List topics filtered by parent

- **WHEN** a `GET /api/v1/topics?parent_topic_id=123` request is received
- **THEN** the system SHALL return HTTP 200 with a JSON array containing only topics whose `parent_topic_id` matches the provided value

#### Scenario: List topics returns empty array when none exist

- **WHEN** a `GET /api/v1/topics` request is received and no topics exist in the database
- **THEN** the system SHALL return HTTP 200 with an empty JSON array `[]`

#### Scenario: Database error during list

- **WHEN** a database error occurs while retrieving topics
- **THEN** the system SHALL return HTTP 500 with an `ErrorResponse` body

### Requirement: Create topic

The system SHALL expose a `POST /api/v1/topics` endpoint that creates a new topic from a JSON request body containing `topic_name` and an optional `parent_topic_id`.

#### Scenario: Successful topic creation

- **WHEN** a `POST /api/v1/topics` request is received with a valid `CreateTopicRequest` containing `topic_name`
- **THEN** the system SHALL insert a new `Topic` record into the database
- **AND** the record SHALL be assigned a unique UUID-based ID
- **AND** `created_at` SHALL be set to the current timestamp
- **AND** the system SHALL return HTTP 201 with the created `Topic` as JSON

#### Scenario: Create topic with parent

- **WHEN** a `POST /api/v1/topics` request is received with `topic_name` and a valid `parent_topic_id`
- **THEN** the system SHALL create the topic with the specified parent association

#### Scenario: Missing topic name

- **WHEN** a `POST /api/v1/topics` request is received without `topic_name` or with an empty `topic_name`
- **THEN** the system SHALL return HTTP 400 with an `ErrorResponse` containing error code `VALIDATION_ERROR`

#### Scenario: Duplicate topic name

- **WHEN** a `POST /api/v1/topics` request is received with a `topic_name` that already exists
- **THEN** the system SHALL return HTTP 409 with an `ErrorResponse` containing error code `CONFLICT`

#### Scenario: Database error during creation

- **WHEN** a database error occurs while inserting the topic
- **THEN** the system SHALL return HTTP 500 with an `ErrorResponse` body

### Requirement: Get topic by ID

The system SHALL expose a `GET /api/v1/topics/{id}` endpoint that returns a single topic by its ID.

#### Scenario: Topic found

- **WHEN** a `GET /api/v1/topics/{id}` request is received with a valid ID that exists in the database
- **THEN** the system SHALL return HTTP 200 with the matching `Topic` as JSON

#### Scenario: Topic not found

- **WHEN** a `GET /api/v1/topics/{id}` request is received with an ID that does not exist
- **THEN** the system SHALL return HTTP 404 with an `ErrorResponse` containing error code `NOT_FOUND`

#### Scenario: Database error during lookup

- **WHEN** a database error occurs while retrieving the topic
- **THEN** the system SHALL return HTTP 500 with an `ErrorResponse` body
