ALTER TABLE sessions ADD COLUMN mfa_method TEXT NOT NULL DEFAULT 'none';
UPDATE sessions SET mfa_method = 'mfa' WHERE is_mfa = true;

ALTER TABLE refresh_tokens ADD COLUMN mfa_method TEXT NOT NULL DEFAULT 'none';
UPDATE refresh_tokens SET mfa_method = 'mfa' WHERE is_mfa = true;
