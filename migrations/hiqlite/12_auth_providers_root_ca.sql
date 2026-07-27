PRAGMA FOREIGN_KEYS = OFF;

CREATE TABLE auth_providers_dg_tmp
(
    id                     TEXT              NOT NULL
        CONSTRAINT auth_providers_pk
            PRIMARY KEY,
    enabled                INTEGER           NOT NULL,
    name                   TEXT              NOT NULL,
    typ                    TEXT              NOT NULL,
    issuer                 TEXT              NOT NULL,
    authorization_endpoint TEXT              NOT NULL,
    token_endpoint         TEXT              NOT NULL,
    userinfo_endpoint      TEXT              NOT NULL,
    client_id              TEXT              NOT NULL,
    secret                 BLOB,
    scope                  TEXT              NOT NULL,
    admin_claim_path       TEXT,
    admin_claim_value      TEXT,
    mfa_claim_path         TEXT,
    mfa_claim_value        TEXT,
    use_pkce               INTEGER           NOT NULL,
    client_secret_basic    INTEGER default 1 NOT NULL,
    client_secret_post     INTEGER default 1 NOT NULL,
    jwks_endpoint          TEXT
) STRICT;

INSERT INTO auth_providers_dg_tmp(id, enabled, name, typ, issuer, authorization_endpoint, token_endpoint,
                                  userinfo_endpoint, client_id, secret, scope, admin_claim_path, admin_claim_value,
                                  mfa_claim_path, mfa_claim_value, use_pkce, client_secret_basic, client_secret_post,
                                  jwks_endpoint)
SELECT id,
       enabled,
       name,
       typ,
       issuer,
       authorization_endpoint,
       token_endpoint,
       userinfo_endpoint,
       client_id,
       secret,
       scope,
       admin_claim_path,
       admin_claim_value,
       mfa_claim_path,
       mfa_claim_value,
       use_pkce,
       client_secret_basic,
       client_secret_post,
       jwks_endpoint
FROM auth_providers;

DROP TABLE auth_providers;

ALTER TABLE auth_providers_dg_tmp
    RENAME TO auth_providers;

CREATE INDEX auth_providers_issuer_index
    ON auth_providers (issuer);

PRAGMA FOREIGN_KEYS = ON;
