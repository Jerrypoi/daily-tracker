# api-error-handling Specification

## Purpose
Structured error response pattern using the ErrorResponse model, with proper HTTP status codes (400, 404, 409, 500) and consistent error codes/messages across all endpoints.

## Requirements

### Requirement: Structured error responses

All API endpoints SHALL return errors as JSON objects conforming to the `ErrorResponse` schema with fields `error` (error code string) and `message` (human-readable description).

#### Scenario: Bad request error

- **WHEN** a request fails validation (missing required field, invalid format, invalid time slot)
- **THEN** the system SHALL return HTTP 400 with `ErrorResponse` containing `error: "VALIDATION_ERROR"` and a descriptive `message`

#### Scenario: Not found error

- **WHEN** a requested resource does not exist in the database
- **THEN** the system SHALL return HTTP 404 with `ErrorResponse` containing `error: "NOT_FOUND"` and a `message` identifying the resource type and ID

#### Scenario: Conflict error

- **WHEN** a creation request conflicts with an existing record (duplicate topic name, duplicate time period)
- **THEN** the system SHALL return HTTP 409 with `ErrorResponse` containing `error: "CONFLICT"` and a `message` describing the conflict

#### Scenario: Internal server error

- **WHEN** an unexpected database or system error occurs
- **THEN** the system SHALL return HTTP 500 with `ErrorResponse` containing `error: "INTERNAL_ERROR"` and a generic `message`
- **AND** the system SHALL NOT expose internal error details (e.g., SQL errors) in the response

### Requirement: ApiError type implements IntoResponse

The `ApiError` enum SHALL implement axum's `IntoResponse` trait so that handlers can return `Result<Json<T>, ApiError>` directly. Each variant SHALL map to a specific HTTP status code and produce the `ErrorResponse` JSON body.

#### Scenario: ApiError::BadRequest maps to HTTP 400

- **WHEN** a handler returns `ApiError::BadRequest` with a message
- **THEN** the HTTP response SHALL have status code 400
- **AND** the body SHALL be a JSON `ErrorResponse` with `error: "VALIDATION_ERROR"`

#### Scenario: ApiError::NotFound maps to HTTP 404

- **WHEN** a handler returns `ApiError::NotFound` with a message
- **THEN** the HTTP response SHALL have status code 404
- **AND** the body SHALL be a JSON `ErrorResponse` with `error: "NOT_FOUND"`

#### Scenario: ApiError::Conflict maps to HTTP 409

- **WHEN** a handler returns `ApiError::Conflict` with a message
- **THEN** the HTTP response SHALL have status code 409
- **AND** the body SHALL be a JSON `ErrorResponse` with `error: "CONFLICT"`

#### Scenario: ApiError::InternalServerError maps to HTTP 500

- **WHEN** a handler returns `ApiError::InternalServerError` with a message
- **THEN** the HTTP response SHALL have status code 500
- **AND** the body SHALL be a JSON `ErrorResponse` with `error: "INTERNAL_ERROR"`
