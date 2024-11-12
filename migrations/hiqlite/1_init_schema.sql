CREATE TABLE api_keys
(
    name       TEXT    NOT NULL
        CONSTRAINT api_keys_pk
            PRIMARY KEY,
    secret     BLOB    NOT NULL,
    created    INTEGER NOT NULL,
    expires    INTEGER,
    enc_key_id TEXT    NOT NULL,
    access     BLOB    NOT NULL
) STRICT;

CREATE INDEX api_keys_enc_key_id_index
    ON api_keys (enc_key_id);

CREATE INDEX api_keys_expires_index
    ON api_keys (expires);

CREATE TABLE auth_providers
(
    id                      TEXT    NOT NULL
        CONSTRAINT auth_providers_pk
            PRIMARY KEY,
    enabled                 INTEGER NOT NULL,
    name                    TEXT    NOT NULL,
    typ                     TEXT    NOT NULL,
    issuer                  TEXT    NOT NULL,
    authorization_endpoint  TEXT    NOT NULL,
    token_endpoint          TEXT    NOT NULL,
    userinfo_endpoint       TEXT    NOT NULL,
    client_id               TEXT    NOT NULL,
    secret                  BLOB,
    scope                   TEXT    NOT NULL,
    admin_claim_path        TEXT,
    admin_claim_value       TEXT,
    mfa_claim_path          TEXT,
    mfa_claim_value         TEXT,
    allow_insecure_requests INTEGER NOT NULL,
    use_pkce                INTEGER NOT NULL,
    root_pem                TEXT
) STRICT;

CREATE TABLE auth_provider_logos
(
    auth_provider_id TEXT NOT NULL
        CONSTRAINT auth_provider_logos_auth_providers_id_fk
            REFERENCES auth_providers
            ON UPDATE CASCADE ON DELETE CASCADE,
    res              TEXT NOT NULL,
    content_type     TEXT NOT NULL,
    data             BLOB NOT NULL,
    CONSTRAINT auth_provider_logos_pk
        PRIMARY KEY (auth_provider_id, res)
) STRICT;

CREATE TABLE clients
(
    id                        TEXT    NOT NULL
        CONSTRAINT clients_pk
            PRIMARY KEY,
    name                      TEXT,
    enabled                   INTEGER NOT NULL,
    confidential              INTEGER NOT NULL,
    secret                    BLOB,
    secret_kid                TEXT,
    redirect_uris             TEXT    NOT NULL,
    post_logout_redirect_uris TEXT,
    allowed_origins           TEXT,
    flows_enabled             TEXT    NOT NULL,
    access_token_alg          TEXT    NOT NULL,
    id_token_alg              TEXT    NOT NULL,
    auth_code_lifetime        INTEGER NOT NULL,
    access_token_lifetime     INTEGER NOT NULL,
    scopes                    TEXT    NOT NULL,
    default_scopes            TEXT    NOT NULL,
    challenge                 TEXT,
    force_mfa                 INTEGER NOT NULL,
    client_uri                TEXT,
    contacts                  TEXT
) STRICT;

CREATE INDEX clients_client_uri_index
    ON clients (client_uri);

CREATE TABLE client_logos
(
    client_id    TEXT NOT NULL
        CONSTRAINT client_logos_client_id_fk
            REFERENCES clients
            ON UPDATE CASCADE ON DELETE CASCADE,
    res          TEXT NOT NULL,
    content_type TEXT NOT NULL,
    data         BLOB NOT NULL,
    CONSTRAINT client_logos_pk
        PRIMARY KEY (client_id, res)
) STRICT;

CREATE TABLE clients_dyn
(
    id                         TEXT    NOT NULL
        CONSTRAINT clients_dyn_pk
            PRIMARY KEY
        CONSTRAINT clients_dyn_clients_id_fk
            REFERENCES clients
            ON UPDATE CASCADE ON DELETE CASCADE,
    created                    INTEGER NOT NULL,
    last_used                  INTEGER,
    registration_token         BLOB    NOT NULL,
    token_endpoint_auth_method TEXT    NOT NULL
) STRICT;

CREATE INDEX clients_dyn_last_used_index
    ON clients_dyn (last_used);

CREATE TABLE colors
(
    client_id TEXT NOT NULL
        CONSTRAINT colors_pk
            PRIMARY KEY
        CONSTRAINT colors_clients_id_fk
            REFERENCES clients
            ON UPDATE CASCADE ON DELETE CASCADE,
    data      BLOB NOT NULL
) STRICT;

CREATE TABLE config
(
    id   TEXT
        CONSTRAINT config_pk
            PRIMARY KEY,
    data BLOB
) STRICT;

CREATE TABLE events
(
    id        TEXT    NOT NULL,
    timestamp INTEGER NOT NULL,
    typ       INTEGER NOT NULL,
    level     INTEGER NOT NULL,
    ip        TEXT,
    data      INTEGER,
    text      TEXT,
    CONSTRAINT events_pk
        PRIMARY KEY (id, timestamp)
) STRICT;

CREATE INDEX events_timestamp_index
    ON events (timestamp desc);

CREATE TABLE groups
(
    id   TEXT NOT NULL
        CONSTRAINT groups_pk
            PRIMARY KEY,
    name TEXT NOT NULL
) STRICT;

CREATE TABLE jwks
(
    kid        TEXT    NOT NULL
        CONSTRAINT jwks_pk
            PRIMARY KEY,
    created_at INTEGER NOT NULL,
    signature  TEXT    NOT NULL,
    enc_key_id TEXT    NOT NULL,
    jwk        BLOB    NOT NULL
) STRICT;

CREATE INDEX jwks_signature_created_at_index
    ON jwks (signature, created_at);

CREATE TABLE roles
(
    id   TEXT NOT NULL
        CONSTRAINT roles_pk
            PRIMARY KEY,
    name TEXT NOT NULL
) STRICT;

CREATE TABLE scopes
(
    id                  TEXT NOT NULL
        CONSTRAINT scopes_pk
            PRIMARY KEY,
    name                TEXT NOT NULL,
    attr_include_access TEXT,
    attr_include_id     TEXT
) STRICT;

CREATE TABLE user_attr_config
(
    name TEXT NOT NULL
        CONSTRAINT user_attr_config_pk
            PRIMARY KEY,
    desc TEXT
) STRICT;

CREATE TABLE users
(
    id                    TEXT              NOT NULL
        CONSTRAINT users_pk
            PRIMARY KEY,
    email                 TEXT              NOT NULL
        CONSTRAINT users_email
            UNIQUE,
    given_name            TEXT              NOT NULL,
    family_name           TEXT              NOT NULL,
    password              TEXT,
    roles                 TEXT              NOT NULL,
    groups                TEXT,
    enabled               INTEGER           NOT NULL,
    email_verified        INTEGER           NOT NULL,
    password_expires      INTEGER,
    created_at            INTEGER           NOT NULL,
    last_login            INTEGER,
    last_failed_login     INTEGER,
    failed_login_attempts INTEGER,
    language              TEXT DEFAULT 'en' NOT NULL,
    webauthn_user_id      TEXT,
    user_expires          INTEGER,
    auth_provider_id      TEXT
        CONSTRAINT users_auth_providers_id_fk
            REFERENCES auth_providers
            ON UPDATE CASCADE ON DELETE set NULL,
    federation_uid        TEXT,
    CONSTRAINT users_federation_key
        UNIQUE (auth_provider_id, federation_uid)
) STRICT;

CREATE TABLE devices
(
    id          TEXT    NOT NULL
        CONSTRAINT devices_pk
            PRIMARY KEY,
    client_id   TEXT    NOT NULL
        CONSTRAINT devices_clients_id_fk
            REFERENCES clients
            ON UPDATE CASCADE ON DELETE CASCADE,
    user_id     TEXT
        CONSTRAINT devices_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    created     INTEGER NOT NULL,
    access_exp  INTEGER NOT NULL,
    refresh_exp INTEGER,
    peer_ip     TEXT    NOT NULL,
    name        TEXT    NOT NULL
) STRICT;

CREATE INDEX devices_access_exp_refresh_exp_index
    ON devices (access_exp, refresh_exp);

CREATE TABLE magic_links
(
    id         TEXT    NOT NULL
        CONSTRAINT magic_links_pk
            PRIMARY KEY,
    user_id    TEXT    NOT NULL
        REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    csrf_token TEXT    NOT NULL,
    cookie     TEXT,
    exp        INTEGER NOT NULL,
    used       INTEGER NOT NULL,
    usage      TEXT    NOT NULL
) STRICT;

CREATE INDEX magic_links_exp_index
    ON magic_links (exp);

CREATE INDEX magic_links_user_id_index
    ON magic_links (user_id);

CREATE TABLE passkeys
(
    user_id         TEXT    NOT NULL
        CONSTRAINT passkeys_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    name            TEXT    NOT NULL,
    passkey_user_id TEXT    NOT NULL,
    passkey         TEXT    NOT NULL,
    credential_id   BLOB    NOT NULL,
    registered      INTEGER NOT NULL,
    last_used       INTEGER NOT NULL,
    user_verified   INTEGER,
    CONSTRAINT passkeys_pk
        PRIMARY KEY (user_id, name)
) STRICT;

CREATE UNIQUE INDEX passkeys_credential_id_index
    ON passkeys (credential_id);

CREATE INDEX passkeys_passkey_user_id_index
    ON passkeys (passkey_user_id);

CREATE TABLE recent_passwords
(
    user_id   TEXT NOT NULL
        CONSTRAINT recent_passwords_pk
            PRIMARY KEY
        REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    passwords TEXT NOT NULL
) STRICT;

CREATE TABLE refresh_tokens
(
    id      TEXT                  NOT NULL
        CONSTRAINT refresh_tokens_pk
            PRIMARY KEY,
    user_id TEXT                  NOT NULL
        REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    nbf     INTEGER               NOT NULL,
    exp     INTEGER               NOT NULL,
    scope   TEXT,
    is_mfa  INTEGER DEFAULT false NOT NULL
) STRICT;

CREATE INDEX refresh_tokens_exp_index
    ON refresh_tokens (exp);

CREATE INDEX refresh_tokens_user_id_index
    ON refresh_tokens (user_id);

CREATE TABLE refresh_tokens_devices
(
    id        TEXT    NOT NULL
        CONSTRAINT refresh_tokens_devices_pk
            PRIMARY KEY,
    device_id TEXT    NOT NULL
        CONSTRAINT refresh_tokens_devices_devices_id_fk
            REFERENCES devices
            ON UPDATE CASCADE ON DELETE CASCADE,
    user_id   TEXT    NOT NULL
        CONSTRAINT refresh_tokens_users_user_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    nbf       INTEGER NOT NULL,
    exp       INTEGER NOT NULL,
    scope     TEXT
) STRICT;

CREATE INDEX refresh_tokens_devices_exp_index
    ON refresh_tokens_devices (exp);

CREATE TABLE sessions
(
    id         TEXT    NOT NULL
        CONSTRAINT sessions_pk
            PRIMARY KEY,
    csrf_token TEXT    NOT NULL,
    user_id    TEXT
        REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    roles      TEXT,
    groups     TEXT,
    is_mfa     INTEGER NOT NULL,
    state      TEXT    NOT NULL,
    exp        INTEGER NOT NULL,
    last_seen  INTEGER NOT NULL,
    remote_ip  TEXT
) STRICT;

CREATE INDEX sessions_exp_index
    ON sessions (exp);

CREATE TABLE user_attr_values
(
    user_id TEXT NOT NULL
        CONSTRAINT user_attr_values_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    key     TEXT NOT NULL
        REFERENCES user_attr_config
            ON UPDATE CASCADE ON DELETE CASCADE,
    value   BLOB NOT NULL,
    CONSTRAINT user_attr_values_pk
        PRIMARY KEY (user_id, key)
) STRICT;

CREATE INDEX users_created_at_index
    ON users (created_at);

CREATE UNIQUE INDEX users_email_uindex
    ON users (email);

CREATE UNIQUE INDEX users_webauthn_user_id_index
    ON users (webauthn_user_id);

CREATE TABLE users_values
(
    id        TEXT NOT NULL
        CONSTRAINT users_values_pk
            PRIMARY KEY
        CONSTRAINT users_values_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    birthdate TEXT,
    phone     TEXT,
    street    TEXT,
    zip       INTEGER,
    city      TEXT,
    country   TEXT
) STRICT;

CREATE TABLE webids
(
    user_id        TEXT    NOT NULL
        CONSTRAINT webids_pk
            PRIMARY KEY
        CONSTRAINT webids_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    custom_triples TEXT,
    expose_email   INTEGER NOT NULL
) STRICT;

