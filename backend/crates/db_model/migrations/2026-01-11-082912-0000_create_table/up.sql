-- Your SQL goes here
CREATE TABLE IF NOT EXISTS topic (
    id BINARY(16) PRIMARY KEY,                -- unique identifier
    topic_name VARCHAR(255) NOT NULL UNIQUE,    -- topic name, eg. playing
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,  -- entry creation time
    updated_at DATETIME ON UPDATE CURRENT_TIMESTAMP,         -- last update time
    parent_topic_id BINARY(16),               -- parent topic, nullable
    FOREIGN KEY (parent_topic_id) REFERENCES topic(id)
);

CREATE TABLE IF NOT EXISTS daily_track (
    id BINARY(16) PRIMARY KEY,                -- unique identifier
    start_time DATETIME NOT NULL,         -- period start, must be :00 or :30
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,  -- entry creation time
    updated_at DATETIME ON UPDATE CURRENT_TIMESTAMP,         -- last update time
    topic_id BINARY(16),                      -- what activity during this period
    comment TEXT,                         -- optional notes
    FOREIGN KEY (topic_id) REFERENCES topic(id),
    INDEX idx_start_time(`start_time`)
);

