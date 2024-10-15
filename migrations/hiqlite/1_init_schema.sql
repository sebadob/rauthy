create table api_keys
(
    name       TEXT    not null
        constraint api_keys_pk
            primary key,
    secret     BLOB    not null,
    created    INTEGER not null,
    expires    INTEGER,
    enc_key_id TEXT    not null,
    access     BLOB    not null
);

create index api_keys_enc_key_id_index
    on api_keys (enc_key_id);

create index api_keys_expires_index
    on api_keys (expires);

create table auth_providers
(
    id                      TEXT    not null
        constraint auth_providers_pk
            primary key,
    enabled                 INTEGER not null,
    name                    TEXT    not null,
    typ                     TEXT    not null,
    issuer                  TEXT    not null,
    authorization_endpoint  TEXT    not null,
    token_endpoint          TEXT    not null,
    userinfo_endpoint       TEXT    not null,
    client_id               TEXT    not null,
    secret                  BLOB,
    scope                   TEXT    not null,
    admin_claim_path        TEXT,
    admin_claim_value       TEXT,
    mfa_claim_path          TEXT,
    mfa_claim_value         TEXT,
    allow_insecure_requests INTEGER not null,
    use_pkce                INTEGER not null,
    root_pem                TEXT
);

create table auth_provider_logos
(
    auth_provider_id TEXT not null
        constraint auth_provider_logos_auth_providers_id_fk
            references auth_providers
            on update cascade on delete cascade,
    res              TEXT not null,
    content_type     TEXT not null,
    data             BLOB not null,
    constraint auth_provider_logos_pk
        primary key (auth_provider_id, res)
);

create table clients
(
    id                        TEXT    not null
        constraint clients_pk
            primary key,
    name                      TEXT,
    enabled                   INTEGER not null,
    confidential              INTEGER not null,
    secret                    BLOB,
    secret_kid                TEXT,
    redirect_uris             TEXT    not null,
    post_logout_redirect_uris TEXT,
    allowed_origins           TEXT,
    flows_enabled             TEXT    not null,
    access_token_alg          TEXT    not null,
    id_token_alg              TEXT    not null,
    auth_code_lifetime        INTEGER not null,
    access_token_lifetime     INTEGER not null,
    scopes                    TEXT    not null,
    default_scopes            TEXT    not null,
    challenge                 TEXT,
    force_mfa                 INTEGER not null,
    client_uri                TEXT,
    contacts                  TEXT
);

create table client_logos
(
    client_id    TEXT not null
        constraint client_logos_client_id_fk
            references clients
            on update cascade on delete cascade,
    res          TEXT not null,
    content_type TEXT not null,
    data         BLOB not null,
    constraint client_logos_pk
        primary key (client_id, res)
);

create table clients_dyn
(
    id                         TEXT    not null
        constraint clients_dyn_pk
            primary key
        constraint clients_dyn_clients_id_fk
            references clients
            on update cascade on delete cascade,
    created                    INTEGER not null,
    last_used                  INTEGER,
    registration_token         BLOB    not null,
    token_endpoint_auth_method TEXT    not null
);

create index clients_dyn_last_used_index
    on clients_dyn (last_used);

create table colors
(
    client_id TEXT not null
        constraint colors_pk
            primary key
        constraint colors_clients_id_fk
            references clients
            on update cascade on delete cascade,
    data      BLOB not null
);

create table config
(
    id   TEXT
        constraint config_pk
            primary key,
    data BLOB
);

create table events
(
    id        TEXT    not null,
    timestamp INTEGER not null,
    typ       INTEGER not null,
    level     INTEGER not null,
    ip        TEXT,
    data      INTEGER,
    TEXT      TEXT,
    constraint events_pk
        primary key (id, timestamp)
);

create index events_timestamp_index
    on events (timestamp desc);

create table groups
(
    id   TEXT not null
        constraint groups_pk
            primary key,
    name TEXT not null
);

create table jwks
(
    kid        TEXT    not null
        constraint jwks_pk
            primary key,
    created_at INTEGER not null,
    signature  TEXT    not null,
    enc_key_id TEXT    not null,
    jwk        BLOB    not null
);

create table roles
(
    id   TEXT not null
        constraint roles_pk
            primary key,
    name TEXT not null
);

create table scopes
(
    id                  TEXT not null
        constraint scopes_pk
            primary key,
    name                TEXT not null,
    attr_include_access TEXT,
    attr_include_id     TEXT
);

create table user_attr_config
(
    name TEXT not null
        constraint user_attr_config_pk
            primary key,
    desc TEXT
);

create table users
(
    id                    TEXT              not null
        constraint users_pk
            primary key,
    email                 TEXT              not null
        constraint users_email
            unique,
    given_name            TEXT              not null,
    family_name           TEXT              not null,
    password              TEXT,
    roles                 TEXT              not null,
    groups                TEXT,
    enabled               INTEGER           not null,
    email_verified        INTEGER           not null,
    password_expires      INTEGER,
    created_at            INTEGER           not null,
    last_login            INTEGER,
    last_failed_login     INTEGER,
    failed_login_attempts INTEGER,
    language              TEXT default 'en' not null,
    webauthn_user_id      TEXT,
    user_expires          INTEGER,
    auth_provider_id      TEXT
        constraint users_auth_providers_id_fk
            references auth_providers
            on update cascade on delete set null,
    federation_uid        TEXT,
    constraint users_federation_key
        unique (auth_provider_id, federation_uid)
);

create table devices
(
    id          TEXT    not null
        constraint devices_pk
            primary key,
    client_id   TEXT    not null
        constraint devices_clients_id_fk
            references clients
            on update cascade on delete cascade,
    user_id     TEXT
        constraint devices_users_id_fk
            references users
            on update cascade on delete cascade,
    created     INTEGER not null,
    access_exp  INTEGER not null,
    refresh_exp INTEGER,
    peer_ip     TEXT    not null,
    name        TEXT    not null
);

create index devices_access_exp_refresh_exp_index
    on devices (access_exp, refresh_exp);

create table magic_links
(
    id         TEXT    not null
        constraint magic_links_pk
            primary key,
    user_id    TEXT    not null
        references users
            on update cascade on delete cascade,
    csrf_token TEXT    not null,
    cookie     TEXT,
    exp        INTEGER not null,
    used       INTEGER not null,
    usage      TEXT    not null
);

create index magic_links_exp_index
    on magic_links (exp);

create index magic_links_user_id_index
    on magic_links (user_id);

create table passkeys
(
    user_id         TEXT    not null
        constraint passkeys_users_id_fk
            references users
            on update cascade on delete cascade,
    name            TEXT    not null,
    passkey_user_id TEXT    not null,
    passkey         TEXT    not null,
    credential_id   BLOB    not null,
    registered      INTEGER not null,
    last_used       INTEGER not null,
    user_verified   INTEGER,
    constraint passkeys_pk
        primary key (user_id, name)
);

create unique index passkeys_credential_id_index
    on passkeys (credential_id);

create index passkeys_passkey_user_id_index
    on passkeys (passkey_user_id);

create table recent_passwords
(
    user_id   TEXT not null
        constraint recent_passwords_pk
            primary key
        references users
            on update cascade on delete cascade,
    passwords TEXT not null
);

create table refresh_tokens
(
    id      TEXT                  not null
        constraint refresh_tokens_pk
            primary key,
    user_id TEXT                  not null
        references users
            on update cascade on delete cascade,
    nbf     INTEGER               not null,
    exp     INTEGER               not null,
    scope   TEXT,
    is_mfa  INTEGER default false not null
);

create index refresh_tokens_exp_index
    on refresh_tokens (exp);

create index refresh_tokens_user_id_index
    on refresh_tokens (user_id);

create table refresh_tokens_devices
(
    id        TEXT    not null
        constraint refresh_tokens_devices_pk
            primary key,
    device_id TEXT    not null
        constraint refresh_tokens_devices_devices_id_fk
            references devices
            on update cascade on delete cascade,
    user_id   TEXT    not null
        constraint refresh_tokens_users_user_id_fk
            references users
            on update cascade on delete cascade,
    nbf       INTEGER not null,
    exp       INTEGER not null,
    scope     TEXT
);

create index refresh_tokens_devices_exp_index
    on refresh_tokens_devices (exp);

create table sessions
(
    id         TEXT    not null
        constraint sessions_pk
            primary key,
    csrf_token TEXT    not null,
    user_id    TEXT
        references users
            on update cascade on delete cascade,
    roles      TEXT,
    groups     TEXT,
    is_mfa     INTEGER not null,
    state      TEXT    not null,
    exp        INTEGER not null,
    last_seen  INTEGER not null,
    remote_ip  TEXT
);

create index sessions_exp_index
    on sessions (exp);

create table user_attr_values
(
    user_id TEXT not null
        constraint user_attr_values_users_id_fk
            references users
            on update cascade on delete cascade,
    key     TEXT not null
        references user_attr_config
            on update cascade on delete cascade,
    value   BLOB not null,
    constraint user_attr_values_pk
        primary key (user_id, key)
);

create index users_created_at_index
    on users (created_at);

create unique index users_email_uindex
    on users (email);

create unique index users_webauthn_user_id_index
    on users (webauthn_user_id);

create table users_values
(
    id        TEXT not null
        constraint users_values_pk
            primary key
        constraint users_values_users_id_fk
            references users
            on update cascade on delete cascade,
    birthdate TEXT,
    phone     TEXT,
    street    TEXT,
    zip       INTEGER,
    city      TEXT,
    country   TEXT
);

create table webids
(
    user_id        TEXT    not null
        constraint webids_pk
            primary key
        constraint webids_users_id_fk
            references users
            on update cascade on delete cascade,
    custom_triples TEXT,
    expose_email   INTEGER not null
);

