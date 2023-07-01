create table webauthn
(
    id      varchar(36) not null
        constraint webauthn_pk
        primary key,
    passkey varchar(1024) not null
);

create table users
(
    id                    varchar(36)  not null
        constraint users_pk
        primary key,
    email                 varchar(64)  not null
        constraint users_email
        unique,
    given_name            varchar(128) not null,
    family_name           varchar(128) not null,
    password              varchar(128),
    roles                 varchar(128) not null,
    groups                varchar(128),
    enabled               boolean      not null,
    email_verified        boolean      not null,
    password_expires      int,
    created_at            int          not null,
    last_login            int,
    last_failed_login     int,
    failed_login_attempts int,
    mfa_app               varchar(36),
    sec_key_1             varchar(36)
        references webauthn,
    sec_key_2             varchar(36)
        references webauthn
);

create unique index users_email_uindex
    on users (email);

create table clients
(
    id                        varchar(36) not null
        constraint clients_pk
        primary key,
    name                      varchar(128),
    enabled                   bool         not null,
    confidential              bool         not null,
    secret                    blob,
    secret_kid                varchar(24),
    redirect_uris             varchar(255) not null,
    post_logout_redirect_uris varchar(255),
    allowed_origins           varchar(255),
    flows_enabled             varchar(128) not null,
    access_token_alg          varchar(8)  not null,
    id_token_alg              varchar(8)  not null,
    refresh_token             bool         not null,
    auth_code_lifetime        integer      not null,
    access_token_lifetime     integer      not null,
    scopes                    varchar(128) not null,
    default_scopes            varchar(128) not null,
    challenge                 varchar(16)
);

create table groups
(
    id   varchar(36) not null
        constraint groups_pk
        primary key,
    name varchar(32)
);

create table jwks
(
    kid        varchar(36) not null
        constraint jwks_pk
        primary key,
    created_at integer     not null,
    signature  blob        not null,
    enc_key_id varchar(24) not null,
    jwk        blob        not null
);

create table magic_links
(
    id         varchar(64) not null
        constraint magic_links_pk
        primary key,
    user_id    varchar(36) not null
        references users
        on delete cascade
        on update cascade,
    csrf_token varchar(48) not null,
    cookie     varchar(48),
    exp        integer     not null,
    used       bool
);

create index magic_links_exp_index
    on magic_links (exp);

create index magic_links_user_id_index
    on magic_links (user_id);

create table mfa_apps
(
    app_id varchar(64) not null
        constraint mfa_apps_pk
        primary key,
    email  varchar(64) not null,
    secret varchar(64) not null
);

create table config
(
    id   varchar(32)
        constraint config_pk
        primary key,
    data blob
);

create table recent_passwords
(
    user_id  varchar(36) not null
        references users
            on delete cascade
            on update cascade
        constraint recent_passwords_pk
        primary key,
    passwords varchar(1024)
);

create table refresh_tokens
(
    id      varchar(36) not null
        constraint refresh_tokens_pk
        primary key,
    user_id varchar(36) not null
        references users
            on delete cascade
            on update cascade,
    nbf     integer     not null,
    exp     integer     not null,
    scope   varchar(128)
);

create index refresh_tokens_exp_index
    on refresh_tokens (exp);

create index refresh_tokens_user_id_index
    on refresh_tokens (user_id);

create table roles
(
    id   varchar(36)  not null
        constraint roles_pk
        primary key,
    name varchar(32) not null
);

create table scopes
(
    id                  varchar(36) not null
        constraint scopes_pk
        primary key,
    name                varchar(32) not null,
    attr_include_access varchar(128),
    attr_include_id     varchar(128)
);

create table user_attr_config
(
    name varchar(128) not null
        constraint user_attr_config_pk
        primary key,
    desc varchar(128)
);

create table user_attr_values
(
    user_id varchar(36) not null,
    key     varchar(32)
        references user_attr_config
            on delete cascade
            on update cascade,
    value   blob,
    constraint user_attr_values_pk
        primary key (user_id, key)
);

create table sessions
(
    id         varchar(36) not null
        constraint sessions_pk
        primary key,
    csrf_token varchar(32) not null,
    user_id    varchar(36)
        references users
            on delete cascade
            on update cascade,
    roles      varchar(255),
    groups     varchar(255),
    is_mfa     bool        not null,
    state      varchar(8)  not null,
    exp        int         not null,
    last_seen  int         not null
);

create index sessions_exp_index
    on sessions (exp);
