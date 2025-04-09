ALTER TABLE auth_provider_logos
    ADD updated BIGINT DEFAULT 0 NOT NULL;

ALTER TABLE client_logos
    ADD updated BIGINT DEFAULT 0 NOT NULL;
