ALTER TABLE auth_providers
    ADD auto_onboarding INTEGER DEFAULT 1 NOT NULL;

ALTER TABLE auth_providers
    ADD auto_link INTEGER DEFAULT 0 NOT NULL;
