ALTER TABLE auth_providers
    ADD jwks_endpoint VARCHAR;

CREATE INDEX auth_providers_issuer_index
    ON auth_providers (issuer);