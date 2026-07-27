ALTER TABLE auth_providers
    DROP COLUMN allow_insecure_requests;

ALTER TABLE auth_providers
    DROP COLUMN root_pem;
