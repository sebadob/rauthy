ALTER TABLE scopes
    ADD COLUMN IF NOT EXISTS claims_at_root bool DEFAULT false NOT NULL;
