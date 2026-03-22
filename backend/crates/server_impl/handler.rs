use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDate;
use db_model::models::DEFAULT_TOPIC_DISPLAY_COLOR;
use models::*;

// --- Topic Handlers ---
fn is_valid_hex_color(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 7 || bytes[0] != b'#' {
        return false;
    }

    bytes[1..].iter().all(|b| b.is_ascii_hexdigit())
}

pub async fn get_topics(
    Query(params): Query<GetTopicsParams>,
) -> Result<Json<Vec<Topic>>, ApiError> {
    let topics = db::get_topics(params.parent_topic_id)
        .map_err(|e| ApiError::InternalServerError(format!("Failed to retrieve topics: {}", e)))?;

    let topics: Vec<Topic> = topics.iter().map(|t| db_topic_to_topic(t)).collect();
    Ok(Json(topics))
}

pub async fn create_topic(
    Json(req): Json<CreateTopicRequest>,
) -> Result<(StatusCode, Json<Topic>), ApiError> {
    if req.topic_name.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "topic_name is required and cannot be empty".to_string(),
        ));
    }

    let parent_topic_id_binary = if let Some(parent_id) = req.parent_topic_id {
        let parent = db::get_topic_by_id(parent_id)
            .map_err(|e| ApiError::InternalServerError(format!("Failed to verify parent topic: {}", e)))?;
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

    let db_topic = db::create_topic(req.topic_name, display_color, parent_topic_id_binary).map_err(|e| {
        match e {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => ApiError::Conflict("Topic name already exists".to_string()),
            _ => ApiError::InternalServerError(format!("Failed to create topic: {}", e)),
        }
    })?;

    Ok((StatusCode::CREATED, Json(db_topic_to_topic(&db_topic))))
}

pub async fn get_topic_by_id(Path(id): Path<u16>) -> Result<Json<Topic>, ApiError> {
    let topic = db::get_topic_by_id(id)
        .map_err(|e| ApiError::InternalServerError(format!("Failed to retrieve topic: {}", e)))?;

    match topic {
        Some(t) => Ok(Json(db_topic_to_topic(&t))),
        None => Err(ApiError::NotFound(format!("Topic with id {} not found", id))),
    }
}

pub async fn update_topic(
    Path(id): Path<u16>,
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

    let updated = db::update_topic(id, topic_name, req.display_color).map_err(|e| match e {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        ) => ApiError::Conflict("Topic name already exists".to_string()),
        _ => ApiError::InternalServerError(format!("Failed to update topic: {}", e)),
    })?;

    match updated {
        Some(topic) => Ok(Json(db_topic_to_topic(&topic))),
        None => Err(ApiError::NotFound(format!("Topic with id {} not found", id))),
    }
}

// --- DailyTrack Handlers ---

pub async fn get_daily_tracks(
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

    let tracks = db::get_daily_tracks(start_date, end_date, params.topic_id)
        .map_err(|e| {
            ApiError::InternalServerError(format!("Failed to retrieve daily tracks: {}", e))
        })?;

    let tracks: Vec<DailyTrack> = tracks.iter().map(|t| db_daily_track_to_daily_track(t)).collect();
    Ok(Json(tracks))
}

pub async fn create_daily_track(
    Json(req): Json<CreateDailyTrackRequest>,
) -> Result<(StatusCode, Json<DailyTrack>), ApiError> {
    let minutes = req.start_time.format("%M").to_string();
    if minutes != "00" && minutes != "30" {
        return Err(ApiError::BadRequest(
            "start_time must be at :00 or :30 minutes".to_string(),
        ));
    }

    let topic = db::get_topic_by_id(req.topic_id).map_err(|e| {
        ApiError::InternalServerError(format!("Failed to verify topic: {}", e))
    })?;

    if topic.is_none() {
        return Err(ApiError::NotFound(format!(
            "Topic with id {} not found",
            req.topic_id
        )));
    }

    let start_time_naive = req.start_time.naive_utc();
    let topic_id_binary = topic.map(|t| t.id);

    let db_track =
        db::create_daily_track(start_time_naive, topic_id_binary, req.comment).map_err(|e| {
            match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => ApiError::Conflict(
                    "A record already exists for this time period".to_string(),
                ),
                _ => ApiError::InternalServerError(format!(
                    "Failed to create daily track: {}",
                    e
                )),
            }
        })?;

    Ok((
        StatusCode::CREATED,
        Json(db_daily_track_to_daily_track(&db_track)),
    ))
}

pub async fn get_daily_track_by_id(Path(id): Path<i64>) -> Result<Json<DailyTrack>, ApiError> {
    let track = db::get_daily_track_by_id(id).map_err(|e| {
        ApiError::InternalServerError(format!("Failed to retrieve daily track: {}", e))
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
    Path(id): Path<u16>,
    Json(req): Json<UpdateDailyTrackRequest>,
) -> Result<Json<DailyTrack>, ApiError> {
    let topic = db::get_topic_by_id(req.topic_id).map_err(|e| {
        ApiError::InternalServerError(format!("Failed to verify topic: {}", e))
    })?;

    let Some(topic) = topic else {
        return Err(ApiError::NotFound(format!(
            "Topic with id {} not found",
            req.topic_id
        )));
    };

    let track = db::update_daily_track(id, topic.id, req.comment).map_err(|e| {
        ApiError::InternalServerError(format!("Failed to update daily track: {}", e))
    })?;

    match track {
        Some(t) => Ok(Json(db_daily_track_to_daily_track(&t))),
        None => Err(ApiError::NotFound(format!(
            "Daily track with id {} not found",
            id
        ))),
    }
}

pub async fn delete_daily_track(Path(id): Path<u16>) -> Result<StatusCode, ApiError> {
    let deleted = db::delete_daily_track(id).map_err(|e| {
        ApiError::InternalServerError(format!("Failed to delete daily track: {}", e))
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
