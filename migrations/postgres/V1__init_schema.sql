create table clients
(
    id                        varchar not null
        constraint clients_pk
            primary key,
    name                      varchar,
    enabled                   boolean not null,
    confidential              boolean not null,
    secret                    bytea,
    secret_kid                varchar,
    redirect_uris             varchar not null,
    post_logout_redirect_uris varchar,
    allowed_origins           varchar,
    flows_enabled             varchar not null,
    access_token_alg          varchar not null,
    id_token_alg              varchar not null,
    auth_code_lifetime        integer not null,
    access_token_lifetime     integer not null,
    scopes                    varchar not null,
    default_scopes            varchar not null,
    challenge                 varchar,
    force_mfa                 boolean not null,
    client_uri                varchar,
    contacts                  varchar
);

create index clients_client_uri_index
    on clients (client_uri);

create table groups
(
    id   varchar not null
        constraint groups_pk
            primary key,
    name varchar not null
);

create table jwks
(
    kid        varchar not null
        constraint jwks_pk
            primary key,
    created_at bigint  not null,
    signature  varchar not null,
    enc_key_id varchar not null,
    jwk        bytea   not null
);

create index jwks_signature_created_at_index
    on jwks (signature, created_at);

create table config
(
    id   varchar not null
        constraint config_pk
            primary key,
    data bytea
);

create table roles
(
    id   varchar not null
        constraint roles_pk
            primary key,
    name varchar not null
);

create table scopes
(
    id                  varchar not null
        constraint scopes_pk
            primary key,
    name                varchar not null,
    attr_include_access varchar,
    attr_include_id     varchar
);

create table user_attr_config
(
    name   varchar not null
        constraint user_attr_config_pk
            primary key,
    "desc" varchar
);

create table events
(
    id        varchar  not null,
    timestamp bigint   not null,
    typ       smallint not null,
    level     smallint not null,
    ip        varchar,
    data      bigint,
    text      varchar,
    constraint events_pk
        primary key (id, timestamp)
);

create index events_timestamp_index
    on events (timestamp desc);

create table api_keys
(
    name       varchar not null
        constraint api_keys_pk
            primary key,
    secret     bytea   not null,
    created    bigint  not null,
    expires    bigint,
    enc_key_id varchar not null,
    access     bytea   not null
);

create index api_keys_enc_key_id_index
    on api_keys (enc_key_id);

create index api_keys_expires_index
    on api_keys (expires);

create table clients_dyn
(
    id                         varchar not null
        constraint clients_dyn_pk
            primary key
        constraint clients_dyn_clients_id_fk
            references clients
            on update cascade on delete cascade,
    created                    bigint  not null,
    last_used                  bigint,
    registration_token         bytea   not null,
    token_endpoint_auth_method varchar not null
);

create index clients_dyn_last_used_index
    on clients_dyn (last_used);

create table auth_providers
(
    id                      varchar              not null
        constraint auth_providers_pk
            primary key,
    enabled                 boolean              not null,
    name                    varchar              not null,
    typ                     varchar              not null,
    issuer                  varchar              not null,
    authorization_endpoint  varchar              not null,
    token_endpoint          varchar              not null,
    userinfo_endpoint       varchar              not null,
    client_id               varchar              not null,
    secret                  bytea,
    scope                   varchar              not null,
    admin_claim_path        varchar,
    admin_claim_value       varchar,
    mfa_claim_path          varchar,
    mfa_claim_value         varchar,
    allow_insecure_requests boolean              not null,
    use_pkce                boolean              not null,
    root_pem                varchar,
    client_secret_basic     boolean default true not null,
    client_secret_post      boolean default true not null
);

create table users
(
    id                    varchar not null
        constraint users_pk
            primary key,
    email                 varchar not null
        constraint users_email
            unique,
    given_name            varchar not null,
    family_name           varchar,
    password              varchar,
    roles                 varchar not null,
    groups                varchar,
    enabled               boolean not null,
    email_verified        boolean not null,
    password_expires      bigint,
    created_at            bigint  not null,
    last_login            bigint,
    last_failed_login     bigint,
    failed_login_attempts bigint,
    language              varchar,
    webauthn_user_id      varchar
        constraint users_pk2
            unique,
    user_expires          bigint,
    auth_provider_id      varchar
        constraint users_auth_providers_id_fk
            references auth_providers
            on update cascade on delete set null,
    federation_uid        varchar,
    picture_id            varchar,
    constraint users_federation_key
        unique (auth_provider_id, federation_uid)
);

create index users_created_at_index
    on users (created_at);

create table magic_links
(
    id         varchar not null
        constraint magic_links_pk
            primary key,
    user_id    varchar not null
        references users
            on update cascade on delete cascade,
    csrf_token varchar not null,
    cookie     varchar,
    exp        bigint  not null,
    used       boolean not null,
    usage      varchar not null
);

create index magic_links_exp_index
    on magic_links (exp);

create index magic_links_user_id_index
    on magic_links (user_id);

create table recent_passwords
(
    user_id   varchar not null
        constraint recent_passwords_pk
            primary key
        references users
            on update cascade on delete cascade,
    passwords varchar not null
);

create table refresh_tokens
(
    id      varchar               not null
        constraint refresh_tokens_pk
            primary key,
    user_id varchar               not null
        references users
            on update cascade on delete cascade,
    nbf     bigint                not null,
    exp     bigint                not null,
    scope   varchar,
    is_mfa  boolean default false not null
);

create index refresh_tokens_exp_index
    on refresh_tokens (exp);

create index refresh_tokens_user_id_index
    on refresh_tokens (user_id);

create table user_attr_values
(
    user_id varchar not null
        constraint user_attr_values_users_id_fk
            references users
            on update cascade on delete cascade,
    key     varchar not null
        references user_attr_config
            on update cascade on delete cascade,
    value   bytea   not null,
    constraint user_attr_values_pk
        primary key (user_id, key)
);

create table sessions
(
    id         varchar not null
        constraint sessions_pk
            primary key,
    csrf_token varchar not null,
    user_id    varchar
        references users
            on update cascade on delete cascade,
    roles      varchar,
    groups     varchar,
    is_mfa     boolean not null,
    state      varchar not null,
    exp        bigint  not null,
    last_seen  bigint  not null,
    remote_ip  varchar
);

create index sessions_exp_index
    on sessions (exp);

create table passkeys
(
    user_id         varchar not null
        constraint passkeys_users_id_fk
            references users
            on update cascade on delete cascade,
    name            varchar not null,
    passkey_user_id varchar not null
        constraint passkeys_users_webauthn_user_id_fk
            references users (webauthn_user_id)
            on update cascade on delete cascade,
    passkey         varchar not null,
    credential_id   bytea   not null,
    registered      bigint  not null,
    last_used       bigint  not null,
    user_verified   boolean,
    constraint passkeys_pk
        primary key (user_id, name)
);

create unique index passkeys_credential_id_uindex
    on passkeys (credential_id);

create index passkeys_passkey_user_id_uindex
    on passkeys (passkey_user_id);

create table webids
(
    user_id        varchar not null
        constraint webids_pk
            primary key
        constraint webids_users_id_fk
            references users
            on update cascade on delete cascade,
    custom_triples varchar,
    expose_email   boolean not null
);

create table users_values
(
    id        varchar not null
        constraint users_values_pk
            primary key
        constraint users_values_users_id_fk
            references users
            on update cascade on delete cascade,
    birthdate varchar,
    phone     varchar,
    street    varchar,
    zip       integer,
    city      varchar,
    country   varchar
);

create table auth_provider_logos
(
    auth_provider_id varchar          not null
        constraint auth_provider_logos_auth_providers_id_fk
            references auth_providers
            on update cascade on delete cascade,
    res              varchar          not null,
    content_type     varchar          not null,
    data             bytea            not null,
    updated          bigint default 0 not null,
    constraint auth_provider_logos_pk
        primary key (auth_provider_id, res)
);

create table client_logos
(
    client_id    varchar          not null
        constraint client_logos_client_id_fk
            references clients
            on update cascade on delete cascade,
    res          varchar          not null,
    content_type varchar          not null,
    data         bytea            not null,
    updated      bigint default 0 not null,
    constraint client_logos_pk
        primary key (client_id, res)
);

create table devices
(
    id          varchar not null
        constraint devices_pk
            primary key,
    client_id   varchar not null
        constraint devices_clients_id_fk
            references clients
            on update cascade on delete cascade,
    user_id     varchar
        constraint devices_users_id_fk
            references users
            on update cascade on delete cascade,
    created     bigint  not null,
    access_exp  bigint  not null,
    refresh_exp bigint,
    peer_ip     varchar not null,
    name        varchar not null
);

create index devices_access_exp_refresh_exp_index
    on devices (access_exp, refresh_exp);

create table refresh_tokens_devices
(
    id        varchar not null
        constraint refresh_tokens_devices_pk
            primary key,
    device_id varchar not null
        constraint refresh_tokens_devices_devices_id_fk
            references devices
            on update cascade on delete cascade,
    user_id   varchar not null
        constraint refresh_tokens_users_user_id_fk
            references users
            on update cascade on delete cascade,
    nbf       bigint  not null,
    exp       bigint  not null,
    scope     varchar
);

create index refresh_tokens_devices_exp_index
    on refresh_tokens_devices (exp);

create table themes
(
    client_id     varchar not null
        constraint themes_pk
            primary key
        constraint themes_clients_id_fk
            references clients
            on update cascade on delete cascade,
    last_update   bigint  not null,
    version       integer not null,
    light         bytea   not null,
    dark          bytea   not null,
    border_radius varchar not null
);

create table pictures
(
    id           varchar not null
        constraint pictures_pk
            primary key,
    content_type varchar not null,
    storage      varchar not null,
    data         bytea
);
