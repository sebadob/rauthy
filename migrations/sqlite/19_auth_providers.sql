-- create ne auth_providers table for upstream auth providers

create table auth_providers
(
    id                      varchar not null
        constraint auth_providers_pk
            primary key,
    name                    varchar not null,
    typ                     varchar not null,
    issuer                  varchar not null,
    authorization_endpoint  varchar not null,
    token_endpoint          varchar not null,
    userinfo_endpoint       varchar not null,
    client_id               varchar not null,
    secret                  blob,
    scope                   varchar not null,
    admin_claim_path        varchar,
    admin_claim_value       varchar,
    allow_insecure_requests bool    not null,
    use_pkce                bool    not null,
    root_pem                varchar,
    logo                    blob,
    logo_type               varchar
);

-- modify users table with fk to auth_providers

alter table users
    rename to users_old;

drop index users_email_uindex;
drop index users_webauthn_user_id_index;

create table users
(
    id                    varchar              not null
        constraint users_pk
            primary key,
    email                 varchar              not null
        constraint users_email
            unique,
    given_name            varchar              not null,
    family_name           varchar              not null,
    password              varchar,
    roles                 varchar              not null,
    groups                varchar,
    enabled               boolean              not null,
    email_verified        boolean              not null,
    password_expires      int,
    created_at            int                  not null,
    last_login            int,
    last_failed_login     int,
    failed_login_attempts int,
    language              varchar default 'en' not null,
    webauthn_user_id      varchar,
    user_expires          int,
    auth_provider_id      varchar
        constraint users_auth_providers_id_fk
            references auth_providers
            on update cascade on delete set null,
    federation_uid        varchar,
    constraint users_federation_key
        unique (auth_provider_id, federation_uid)
);

create unique index users_email_uindex
    on users (email);

create unique index users_webauthn_user_id_index
    on users (webauthn_user_id);

insert into users(id, email, given_name, family_name, password, roles, groups, enabled, email_verified,
                  password_expires, created_at, last_login, last_failed_login, failed_login_attempts, language,
                  webauthn_user_id, user_expires)
select id,
       email,
       given_name,
       family_name,
       password,
       roles,
       groups,
       enabled,
       email_verified,
       password_expires,
       created_at,
       last_login,
       last_failed_login,
       failed_login_attempts,
       language,
       webauthn_user_id,
       user_expires
from users_old;

-- recreate all tables with foreign keys to users

alter table magic_links
    rename to magic_links_old;

drop index magic_links_exp_index;
drop index magic_links_user_id_index;

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
    exp        integer not null,
    used       bool    not null,
    usage      varchar not null
);

create index magic_links_exp_index
    on magic_links (exp);

create index magic_links_user_id_index
    on magic_links (user_id);

insert into magic_links(id, user_id, csrf_token, cookie, exp, used, usage)
select id, user_id, csrf_token, cookie, exp, used, usage
from magic_links_old;

drop table magic_links_old;

--

alter table recent_passwords
    rename to recent_passwords_old;

create table recent_passwords
(
    user_id   varchar not null
        references users
            on delete cascade
            on update cascade
        constraint recent_passwords_pk
            primary key,
    passwords varchar not null
);

insert into recent_passwords(user_id, passwords)
select user_id, passwords
from recent_passwords_old;

drop table recent_passwords_old;

--

alter table refresh_tokens
    rename to refresh_tokens_old;

drop index refresh_tokens_exp_index;
drop index refresh_tokens_user_id_index;

create table refresh_tokens
(
    id      varchar            not null
        constraint refresh_tokens_pk
            primary key,
    user_id varchar            not null
        references users
            on delete cascade
            on update cascade,
    nbf     integer            not null,
    exp     integer            not null,
    scope   varchar,
    is_mfa  bool default false not null
);

create index refresh_tokens_exp_index
    on refresh_tokens (exp);

create index refresh_tokens_user_id_index
    on refresh_tokens (user_id);

insert into refresh_tokens(id, user_id, nbf, exp, scope, is_mfa)
select id, user_id, nbf, exp, scope, is_mfa
from refresh_tokens_old;

drop table refresh_tokens_old;

--

alter table sessions
    rename to sessions_old;

drop index sessions_exp_index;

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
    is_mfa     bool    not null,
    state      varchar not null,
    exp        int     not null,
    last_seen  int     not null,
    remote_ip  varchar
);

create index sessions_exp_index
    on sessions (exp);

insert into sessions(id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen, remote_ip)
select id,
       csrf_token,
       user_id,
       roles,
       groups,
       is_mfa,
       state,
       exp,
       last_seen,
       remote_ip
from sessions_old;

drop table sessions_old;

--

alter table passkeys
    rename to passkeys_old;

drop index passkeys_credential_id_index;
drop index passkeys_passkey_user_id_index;

create table passkeys
(
    user_id         varchar not null
        constraint passkeys_users_id_fk
            references users
            on update cascade on delete cascade,
    name            varchar not null,
    passkey_user_id varchar not null,
    passkey         varchar not null,
    credential_id   blob    not null,
    registered      integer not null,
    last_used       integer not null,
    user_verified   bool,
    constraint passkeys_pk
        primary key (user_id, name)
);

create unique index passkeys_credential_id_index
    on passkeys (credential_id);

create index passkeys_passkey_user_id_index
    on passkeys (passkey_user_id);

insert into passkeys(user_id, name, passkey_user_id, passkey, credential_id, registered, last_used, user_verified)
select user_id,
       name,
       passkey_user_id,
       passkey,
       credential_id,
       registered,
       last_used,
       user_verified
from passkeys_old;

drop table passkeys_old;

--

alter table user_attr_values
    rename to user_attr_values_old;

create table user_attr_values
(
    user_id varchar not null
        constraint user_attr_values_users_id_fk
            references users
            on update cascade on delete cascade,
    key     varchar not null
        references user_attr_config
            on update cascade on delete cascade,
    value   blob    not null,
    constraint user_attr_values_pk
        primary key (user_id, key)
);

insert into user_attr_values(user_id, key, value)
select user_id, key, value
from user_attr_values_old;

drop table user_attr_values_old;

--

alter table users_values
    rename to users_values_old;

create table users_values
(
    id        text not null
        constraint users_values_pk
            primary key
        constraint users_values_users_id_fk
            references users
            on update cascade on delete cascade,
    birthdate text,
    phone     text,
    street    text,
    zip       integer,
    city      text,
    country   text
);

insert into users_values(id, birthdate, phone, street, zip, city, country)
select id, birthdate, phone, street, zip, city, country
from users_values_old;

drop table users_values_old;

--


alter table webids
    rename to webids_old;

create table webids
(
    user_id        varchar not null
        constraint webids_pk
            primary key
        constraint webids_users_id_fk
            references users
            on update cascade on delete cascade,
    custom_triples text,
    expose_email   bool    not null
);

insert into webids(user_id, custom_triples, expose_email)
select user_id, custom_triples, expose_email
from webids_old;

drop table webids_old;

-- finally drop the old users table

drop table users_old;
