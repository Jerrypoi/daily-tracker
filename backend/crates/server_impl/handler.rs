use axum::Json;
use axum::extract::{Extension, Path, Query};
use axum::http::StatusCode;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::NaiveDate;
use db_model::models::DEFAULT_TOPIC_DISPLAY_COLOR;
use models::*;

// --- Validation Helpers ---
pub(crate) fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.splitn(2, '@').collect();
    if parts.len() != 2 {
        return false;
    }
    let (local, domain) = (parts[0], parts[1]);
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    // Domain must contain at least one dot and no consecutive dots
    if !domain.contains('.') || domain.contains("..") {
        return false;
    }
    // Domain parts must not be empty
    domain.split('.').all(|part| !part.is_empty())
}

// --- Topic Handlers ---
pub(crate) fn is_valid_hex_color(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 7 || bytes[0] != b'#' {
        return false;
    }

    bytes[1..].iter().all(|b| b.is_ascii_hexdigit())
}

pub async fn get_topics(
    Extension(user_id): Extension<i64>,
    Query(params): Query<GetTopicsParams>,
) -> Result<Json<Vec<Topic>>, ApiError> {
    let topics = db::get_topics(params.parent_topic_id, Some(user_id)).map_err(|e| {
        log::error!("Failed to retrieve topics: {}", e);
        ApiError::InternalServerError("Failed to retrieve topics".to_string())
    })?;

    let topics: Vec<Topic> = topics.iter().map(|t| db_topic_to_topic(t)).collect();
    Ok(Json(topics))
}

pub async fn create_topic(
    Extension(user_id): Extension<i64>,
    Json(req): Json<CreateTopicRequest>,
) -> Result<(StatusCode, Json<Topic>), ApiError> {
    if req.topic_name.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "topic_name is required and cannot be empty".to_string(),
        ));
    }

    let parent_topic_id = if let Some(parent_id) = req.parent_topic_id {
        let parent = db::get_topic_by_id_for_user(parent_id, user_id).map_err(|e| {
            log::error!("Failed to verify parent topic: {}", e);
            ApiError::InternalServerError("Failed to verify parent topic".to_string())
        })?;
        let parent = parent.ok_or_else(|| {
            ApiError::NotFound(format!("Parent topic with id {} not found", parent_id))
        })?;
        Some(parent.id)
    } else {
        None
    };

    let display_color = req
        .display_color
        .unwrap_or_else(|| DEFAULT_TOPIC_DISPLAY_COLOR.to_string());
    if !is_valid_hex_color(&display_color) {
        return Err(ApiError::BadRequest(
            "display_color must be a valid hex color like #3b82f6".to_string(),
        ));
    }

    let db_topic = db::create_topic(
        req.topic_name,
        display_color,
        parent_topic_id,
        Some(user_id),
    )
    .map_err(|e| match e {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        ) => ApiError::Conflict("Topic name already exists".to_string()),
        _ => {
            log::error!("Failed to create topic: {}", e);
            ApiError::InternalServerError("Failed to create topic".to_string())
        }
    })?;

    Ok((StatusCode::CREATED, Json(db_topic_to_topic(&db_topic))))
}

pub async fn get_topic_by_id(
    Extension(user_id): Extension<i64>,
    Path(id): Path<i64>,
) -> Result<Json<Topic>, ApiError> {
    let topic = db::get_topic_by_id_for_user(id, user_id).map_err(|e| {
        log::error!("Failed to retrieve topic: {}", e);
        ApiError::InternalServerError("Failed to retrieve topic".to_string())
    })?;

    match topic {
        Some(t) => Ok(Json(db_topic_to_topic(&t))),
        None => Err(ApiError::NotFound(format!(
            "Topic with id {} not found",
            id
        ))),
    }
}

pub async fn update_topic(
    Extension(user_id): Extension<i64>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateTopicRequest>,
) -> Result<Json<Topic>, ApiError> {
    let topic_name = req.topic_name.trim().to_string();
    if topic_name.is_empty() {
        return Err(ApiError::BadRequest(
            "topic_name is required and cannot be empty".to_string(),
        ));
    }
    if !is_valid_hex_color(&req.display_color) {
        return Err(ApiError::BadRequest(
            "display_color must be a valid hex color like #3b82f6".to_string(),
        ));
    }

    let updated =
        db::update_topic(id, topic_name, req.display_color, user_id).map_err(|e| match e {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => ApiError::Conflict("Topic name already exists".to_string()),
            _ => {
                log::error!("Failed to update topic: {}", e);
                ApiError::InternalServerError("Failed to update topic".to_string())
            }
        })?;

    match updated {
        Some(topic) => Ok(Json(db_topic_to_topic(&topic))),
        None => Err(ApiError::NotFound(format!(
            "Topic with id {} not found",
            id
        ))),
    }
}

// --- DailyTrack Handlers ---

pub async fn get_daily_tracks(
    Extension(user_id): Extension<i64>,
    Query(params): Query<GetDailyTracksParams>,
) -> Result<Json<Vec<DailyTrack>>, ApiError> {
    let start_date = params
        .start_date
        .map(|s| {
            NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|_| {
                ApiError::BadRequest(format!(
                    "Invalid start_date format '{}'. Expected YYYY-MM-DD",
                    s
                ))
            })
        })
        .transpose()?;

    let end_date = params
        .end_date
        .map(|s| {
            NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|_| {
                ApiError::BadRequest(format!(
                    "Invalid end_date format '{}'. Expected YYYY-MM-DD",
                    s
                ))
            })
        })
        .transpose()?;

    let tracks = db::get_daily_tracks(start_date, end_date, params.topic_id, Some(user_id))
        .map_err(|e| {
            log::error!("Failed to retrieve daily tracks: {}", e);
            ApiError::InternalServerError("Failed to retrieve daily tracks".to_string())
        })?;

    let tracks: Vec<DailyTrack> = tracks
        .iter()
        .map(|t| db_daily_track_to_daily_track(t))
        .collect();
    Ok(Json(tracks))
}

pub async fn create_daily_track(
    Extension(user_id): Extension<i64>,
    Json(req): Json<CreateDailyTrackRequest>,
) -> Result<(StatusCode, Json<DailyTrack>), ApiError> {
    let minutes = req.start_time.format("%M").to_string();
    if minutes != "00" && minutes != "30" {
        return Err(ApiError::BadRequest(
            "start_time must be at :00 or :30 minutes".to_string(),
        ));
    }

    let topic = db::get_topic_by_id_for_user(req.topic_id, user_id).map_err(|e| {
        log::error!("Failed to verify topic: {}", e);
        ApiError::InternalServerError("Failed to verify topic".to_string())
    })?;

    if topic.is_none() {
        return Err(ApiError::NotFound(format!(
            "Topic with id {} not found",
            req.topic_id
        )));
    }

    let start_time_naive = req.start_time.naive_utc();
    let topic_id = topic.map(|t| t.id);

    let db_track = db::create_daily_track(start_time_naive, topic_id, req.comment, Some(user_id))
        .map_err(|e| match e {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        ) => ApiError::Conflict("A record already exists for this time period".to_string()),
        _ => {
            log::error!("Failed to create daily track: {}", e);
            ApiError::InternalServerError("Failed to create daily track".to_string())
        }
    })?;

    Ok((
        StatusCode::CREATED,
        Json(db_daily_track_to_daily_track(&db_track)),
    ))
}

pub async fn get_daily_track_by_id(
    Extension(user_id): Extension<i64>,
    Path(id): Path<i64>,
) -> Result<Json<DailyTrack>, ApiError> {
    let track = db::get_daily_track_by_id(id, user_id).map_err(|e| {
        log::error!("Failed to retrieve daily track: {}", e);
        ApiError::InternalServerError("Failed to retrieve daily track".to_string())
    })?;

    match track {
        Some(t) => Ok(Json(db_daily_track_to_daily_track(&t))),
        None => Err(ApiError::NotFound(format!(
            "Daily track with id {} not found",
            id
        ))),
    }
}

pub async fn update_daily_track(
    Extension(user_id): Extension<i64>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateDailyTrackRequest>,
) -> Result<Json<DailyTrack>, ApiError> {
    let topic = db::get_topic_by_id_for_user(req.topic_id, user_id).map_err(|e| {
        log::error!("Failed to verify topic: {}", e);
        ApiError::InternalServerError("Failed to verify topic".to_string())
    })?;

    let Some(topic) = topic else {
        return Err(ApiError::NotFound(format!(
            "Topic with id {} not found",
            req.topic_id
        )));
    };

    let track = db::update_daily_track(id, topic.id, req.comment, user_id).map_err(|e| {
        log::error!("Failed to update daily track: {}", e);
        ApiError::InternalServerError("Failed to update daily track".to_string())
    })?;

    match track {
        Some(t) => Ok(Json(db_daily_track_to_daily_track(&t))),
        None => Err(ApiError::NotFound(format!(
            "Daily track with id {} not found",
            id
        ))),
    }
}

pub async fn delete_daily_track(
    Extension(user_id): Extension<i64>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let deleted = db::delete_daily_track(id, user_id).map_err(|e| {
        log::error!("Failed to delete daily track: {}", e);
        ApiError::InternalServerError("Failed to delete daily track".to_string())
    })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound(format!(
            "Daily track with id {} not found",
            id
        )))
    }
}

pub async fn register(
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    // Email format validation
    let email = req.email.trim();
    if email.is_empty() || !is_valid_email(email) {
        return Err(ApiError::BadRequest(
            "A valid email address is required".to_string(),
        ));
    }

    if req.password.len() < 8 {
        return Err(ApiError::BadRequest(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    let password_hash = hash(&req.password, DEFAULT_COST).map_err(|e| {
        log::error!("Failed to hash password: {}", e);
        ApiError::InternalServerError("Registration failed".to_string())
    })?;

    let (user, code) =
        db::create_user(req.username, req.email, password_hash).map_err(|e| match e {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => ApiError::Conflict("Username or email already exists".to_string()),
            _ => {
                log::error!("Database error during registration: {}", e);
                ApiError::InternalServerError("Registration failed".to_string())
            }
        })?;

    // Send verification email in the background
    let email_to = user.email.clone();
    let email_username = user.username.clone();
    let email_code = code.clone();
    let log_id = logging::current_log_id();
    tokio::spawn(logging::LOG_ID.scope(log_id, async move {
        if let Err(e) =
            crate::email::send_verification_email(&email_to, &email_username, &email_code).await
        {
            log::error!("Failed to send verification email to {}: {}", email_to, e);
        } else {
            log::info!("Verification email sent to {}", email_to);
        }
    }));

    Ok((
        StatusCode::CREATED,
        Json(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            email_verified: user.email_verified,
        }),
    ))
}

pub async fn verify_email(
    Json(req): Json<VerifyEmailRequest>,
) -> Result<Json<VerifyEmailResponse>, ApiError> {
    let verified = db::verify_email_code(&req.email, &req.code).map_err(|e| {
        log::error!("Database error during email verification: {}", e);
        ApiError::InternalServerError("Verification failed".to_string())
    })?;

    if !verified {
        return Err(ApiError::BadRequest(
            "Invalid or expired verification code".to_string(),
        ));
    }

    Ok(Json(VerifyEmailResponse {
        message: "Email verified successfully".to_string(),
    }))
}

pub async fn login(Json(req): Json<LoginRequest>) -> Result<Json<TokenResponse>, ApiError> {
    let user = db::get_user_by_username(&req.username)
        .map_err(|e| {
            log::error!("Database error during login: {}", e);
            ApiError::InternalServerError("Login failed".to_string())
        })?
        .ok_or_else(|| ApiError::Unauthorized("Invalid username or password".to_string()))?;

    let valid = verify(&req.password, &user.password_hash).map_err(|e| {
        log::error!("Error verifying password: {}", e);
        ApiError::InternalServerError("Login failed".to_string())
    })?;

    if !valid {
        return Err(ApiError::Unauthorized(
            "Invalid username or password".to_string(),
        ));
    }

    if !user.email_verified {
        return Err(ApiError::Forbidden(
            "Email address not verified. Please check your email for the verification code."
                .to_string(),
        ));
    }

    let token = crate::server_auth::create_jwt(user.id).map_err(|e| {
        log::error!("Error generating token: {}", e);
        ApiError::InternalServerError("Login failed".to_string())
    })?;

    Ok(Json(TokenResponse { token }))
}

// --- API Key Handlers ---

pub async fn list_api_keys(
    Extension(user_id): Extension<i64>,
) -> Result<Json<Vec<ApiKeyResponse>>, ApiError> {
    let keys = db::list_api_keys_for_user(user_id).map_err(|e| {
        log::error!("Failed to list API keys: {}", e);
        ApiError::InternalServerError("Failed to list API keys".to_string())
    })?;

    let keys: Vec<ApiKeyResponse> = keys.iter().map(db_api_key_to_response).collect();
    Ok(Json(keys))
}

pub async fn create_api_key(
    Extension(user_id): Extension<i64>,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<CreateApiKeyResponse>), ApiError> {
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Err(ApiError::BadRequest(
            "name is required and cannot be empty".to_string(),
        ));
    }

    let (record, token) = db::create_api_key(user_id, name).map_err(|e| {
        log::error!("Failed to create API key: {}", e);
        ApiError::InternalServerError("Failed to create API key".to_string())
    })?;

    let response = CreateApiKeyResponse {
        id: db_api_key_to_response(&record).id,
        name: record.name,
        key_prefix: record.key_prefix,
        token,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn revoke_api_key(
    Extension(user_id): Extension<i64>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let revoked = db::revoke_api_key(id, user_id).map_err(|e| {
        log::error!("Failed to revoke API key: {}", e);
        ApiError::InternalServerError("Failed to revoke API key".to_string())
    })?;

    if revoked {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound(format!(
            "API key with id {} not found",
            id
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- is_valid_email tests ---

    #[test]
    fn email_valid_standard() {
        assert!(is_valid_email("user@example.com"));
    }

    #[test]
    fn email_valid_subdomain() {
        assert!(is_valid_email("user@mail.example.com"));
    }

    #[test]
    fn email_valid_plus_addressing() {
        assert!(is_valid_email("user+tag@example.com"));
    }

    #[test]
    fn email_valid_dots_in_local() {
        assert!(is_valid_email("first.last@example.com"));
    }

    #[test]
    fn email_invalid_empty() {
        assert!(!is_valid_email(""));
    }

    #[test]
    fn email_invalid_no_at() {
        assert!(!is_valid_email("userexample.com"));
    }

    #[test]
    fn email_invalid_no_local() {
        assert!(!is_valid_email("@example.com"));
    }

    #[test]
    fn email_invalid_no_domain() {
        assert!(!is_valid_email("user@"));
    }

    #[test]
    fn email_invalid_no_tld() {
        assert!(!is_valid_email("user@example"));
    }

    #[test]
    fn email_invalid_consecutive_dots_in_domain() {
        assert!(!is_valid_email("user@example..com"));
    }

    #[test]
    fn email_invalid_trailing_dot_in_domain() {
        assert!(!is_valid_email("user@example.com."));
    }

    #[test]
    fn email_invalid_leading_dot_in_domain() {
        assert!(!is_valid_email("user@.example.com"));
    }

    #[test]
    fn email_invalid_only_at_signs() {
        assert!(!is_valid_email("@@"));
    }

    #[test]
    fn email_invalid_at_only() {
        assert!(!is_valid_email("@"));
    }

    // --- is_valid_hex_color tests ---

    #[test]
    fn hex_color_valid_lowercase() {
        assert!(is_valid_hex_color("#3b82f6"));
    }

    #[test]
    fn hex_color_valid_uppercase() {
        assert!(is_valid_hex_color("#FF00AA"));
    }

    #[test]
    fn hex_color_valid_mixed_case() {
        assert!(is_valid_hex_color("#aAbBcC"));
    }

    #[test]
    fn hex_color_valid_black() {
        assert!(is_valid_hex_color("#000000"));
    }

    #[test]
    fn hex_color_valid_white() {
        assert!(is_valid_hex_color("#ffffff"));
    }

    #[test]
    fn hex_color_invalid_no_hash() {
        assert!(!is_valid_hex_color("3b82f6"));
    }

    #[test]
    fn hex_color_invalid_short() {
        assert!(!is_valid_hex_color("#fff"));
    }

    #[test]
    fn hex_color_invalid_too_long() {
        assert!(!is_valid_hex_color("#3b82f6a"));
    }

    #[test]
    fn hex_color_invalid_non_hex_chars() {
        assert!(!is_valid_hex_color("#gghhii"));
    }

    #[test]
    fn hex_color_invalid_empty() {
        assert!(!is_valid_hex_color(""));
    }

    #[test]
    fn hex_color_invalid_hash_only() {
        assert!(!is_valid_hex_color("#"));
    }
}
