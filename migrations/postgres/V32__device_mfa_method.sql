ALTER TABLE refresh_tokens_devices ADD COLUMN mfa_method varchar NOT NULL DEFAULT 'none';
