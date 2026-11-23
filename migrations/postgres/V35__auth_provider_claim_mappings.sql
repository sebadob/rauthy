ALTER TABLE auth_providers
    ADD claim_mappings VARCHAR;

ALTER TABLE auth_providers
    ADD hide_login bool DEFAULT false NOT NULL;
