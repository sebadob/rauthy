ALTER TABLE auth_providers
    ADD client_secret_basic INTEGER DEFAULT 1 NOT NULL;
ALTER TABLE auth_providers
    ADD client_secret_post INTEGER DEFAULT 1 NOT NULL;