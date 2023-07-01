create table webauthn
(
    id      varchar not null
        constraint webauthn_pk
            primary key,
    passkey varchar not null
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
    family_name           varchar not null,
    password              varchar,
    roles                 varchar not null,
    groups                varchar,
    enabled               boolean not null,
    email_verified        boolean not null,
    password_expires      bigint,
    created_at            bigint  not null,
    last_login            bigint,
    last_failed_login     bigint,
    failed_login_attempts int4,
    mfa_app               varchar,
    sec_key_1             varchar
        references webauthn,
    sec_key_2             varchar
        references webauthn
);

create unique index users_email_uindex
    on users (email);

create table clients
(
    id                        varchar not null
        constraint clients_pk
            primary key,
    name                      varchar,
    enabled                   bool    not null,
    confidential              bool    not null,
    secret                    bytea,
    secret_kid                varchar,
    redirect_uris             varchar not null,
    post_logout_redirect_uris varchar,
    allowed_origins           varchar,
    flows_enabled             varchar not null,
    access_token_alg          varchar not null,
    id_token_alg              varchar not null,
    refresh_token             bool    not null,
    auth_code_lifetime        int4    not null,
    access_token_lifetime     int4    not null,
    scopes                    varchar not null,
    default_scopes            varchar not null,
    challenge                 varchar
);

create table groups
(
    id   varchar not null
        constraint groups_pk
            primary key,
    name varchar
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

create table magic_links
(
    id         varchar not null
        constraint magic_links_pk
            primary key,
    user_id    varchar not null
        references users
            on delete cascade
            on update cascade,
    csrf_token varchar not null,
    cookie     varchar,
    exp        bigint  not null,
    used       bool
);

create index magic_links_exp_index
    on magic_links (exp);

create index magic_links_user_id_index
    on magic_links (user_id);

create table mfa_apps
(
    app_id varchar not null
        constraint mfa_apps_pk
            primary key,
    email  varchar not null,
    secret varchar not null
);

create table config
(
    id   varchar
        constraint config_pk
            primary key,
    data bytea
);

create table recent_passwords
(
    user_id   varchar not null
        references users
            on delete cascade
            on update cascade
        constraint recent_passwords_pk
            primary key,
    passwords varchar
);

create table refresh_tokens
(
    id      varchar not null
        constraint refresh_tokens_pk
            primary key,
    user_id varchar not null
        references users
            on delete cascade
            on update cascade,
    nbf     bigint  not null,
    exp     bigint  not null,
    scope   varchar
);

create index refresh_tokens_exp_index
    on refresh_tokens (exp);

create index refresh_tokens_user_id_index
    on refresh_tokens (user_id);

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

create table user_attr_values
(
    user_id varchar not null,
    key     varchar
        references user_attr_config
            on delete cascade
            on update cascade,
    value   bytea,
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
            on delete cascade
            on update cascade,
    roles      varchar,
    groups     varchar,
    is_mfa     bool    not null,
    state      varchar not null,
    exp        bigint  not null,
    last_seen  bigint  not null
);

create index sessions_exp_index
    on sessions (exp);
