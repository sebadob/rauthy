ALTER TABLE auth_providers
    ADD claim_mappings TEXT;

ALTER TABLE auth_providers
    ADD hide_login INTEGER DEFAULT 0 NOT NULL;
