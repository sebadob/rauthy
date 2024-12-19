ALTER TABLE auth_providers
    ADD client_secret_basic BOOLEAN DEFAULT true NOT NULL;
ALTER TABLE auth_providers
    ADD client_secret_post BOOLEAN DEFAULT true NOT NULL;