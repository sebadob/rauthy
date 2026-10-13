ALTER TABLE refresh_tokens_devices ADD COLUMN mfa_method TEXT NOT NULL DEFAULT 'none';
