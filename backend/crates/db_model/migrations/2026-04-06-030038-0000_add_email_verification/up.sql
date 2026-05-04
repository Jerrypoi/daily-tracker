ALTER TABLE users
  ADD COLUMN email VARCHAR(255) NOT NULL DEFAULT '' AFTER username,
  ADD COLUMN email_verified TINYINT(1) NOT NULL DEFAULT 0 AFTER password_hash,
  ADD COLUMN verification_code TEXT AFTER email_verified,
  ADD COLUMN verification_code_expires_at DATETIME AFTER verification_code;
