CREATE TABLE api_keys (
    id BIGINT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    key_hash VARCHAR(64) NOT NULL UNIQUE,
    key_prefix VARCHAR(16) NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at DATETIME NULL,
    revoked_at DATETIME NULL,
    CONSTRAINT fk_api_keys_user FOREIGN KEY (user_id) REFERENCES users(id),
    INDEX idx_api_keys_user (user_id)
);
