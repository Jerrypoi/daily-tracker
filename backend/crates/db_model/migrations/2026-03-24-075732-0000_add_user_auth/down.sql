ALTER TABLE daily_track DROP FOREIGN KEY fk_daily_track_user;
ALTER TABLE topic DROP FOREIGN KEY fk_topic_user;
ALTER TABLE daily_track DROP COLUMN user_id;
ALTER TABLE topic DROP COLUMN user_id;
DROP TABLE users;
