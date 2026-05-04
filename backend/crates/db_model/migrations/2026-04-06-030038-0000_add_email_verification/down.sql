ALTER TABLE users
  DROP COLUMN email,
  DROP COLUMN email_verified,
  DROP COLUMN verification_code,
  DROP COLUMN verification_code_expires_at;
