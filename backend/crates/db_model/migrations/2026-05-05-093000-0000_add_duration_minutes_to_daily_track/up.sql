ALTER TABLE daily_track
    ADD COLUMN duration_minutes INT NOT NULL DEFAULT 30
    COMMENT 'Activity length in minutes (must be a positive multiple of 30)';
