# Daily tracker tech design 
This is the tech design for daily tracker. 
## DB schema
The table and sql below define the basic database schema
### Topic
| column | type | comment |
| ------ | ---- | ------- |
| id     | bigint | unique id, primary key |
| topic_name   | name of this field| eg. playing, unique|
| created_at | DateTime | time when this entry was created default now. |
| updated_at | DateTime | time when this entry was updated. on update now. | 
| parent_topic_id | bigint | nullable. parent topic of current topic. foreign key |
```SQL 
CREATE TABLE IF NOT EXISTS topic (
    id BIGINT PRIMARY KEY,                -- unique identifier
    topic_name VARCHAR(255) NOT NULL UNIQUE,    -- topic name, eg. playing
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,  -- entry creation time
    updated_at DATETIME ON UPDATE CURRENT_TIMESTAMP,         -- last update time
    parent_topic_id BIGINT,               -- parent topic, nullable
    FOREIGN KEY (parent_topic_id) REFERENCES topic(id)
);
```

### Daily track 
| column | type | comment | 
| ---    | ---- | ------- |
| id | bigint | uuid, primary key|
| start_time | DateTime | start time of this time period. inclusive. must be in either 0 minute or 30 minute of an hour. not null. By default a time period lasts 30 minutes |
| created_at | DateTime | time this entry was created, not null. default now | 
| updated_at| DateTime | time this entry was updated. on update set to now.|
| topic_id | bigint | foreign key to the topic table. indicate what am i doing during this time period.

| comment | text | any extra comment for this time period. | 

```SQL
CREATE TABLE IF NOT EXISTS daily_track (
    id BIGINT PRIMARY KEY,                -- unique identifier
    start_time DATETIME NOT NULL,         -- period start, must be :00 or :30
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,  -- entry creation time
    updated_at DATETIME ON UPDATE CURRENT_TIMESTAMP,         -- last update time
    topic_id BIGINT,                      -- what activity during this period
    comment TEXT,                         -- optional notes
    FOREIGN KEY (topic_id) REFERENCES topic(id),
    INDEX idx_start_time(`start_time`)
);
```






