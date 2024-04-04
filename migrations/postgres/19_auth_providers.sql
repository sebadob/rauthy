-- create ne auth_providers table for upstream auth providers

create table auth_providers
(
    id                      varchar not null
        constraint auth_providers_pk
            primary key,
    enabled                 bool    not null,
    name                    varchar not null,
    typ                     varchar not null,
    issuer                  varchar not null,
    authorization_endpoint  varchar not null,
    token_endpoint          varchar not null,
    userinfo_endpoint       varchar not null,
    client_id               varchar not null,
    secret                  bytea,
    scope                   varchar not null,
    admin_claim_path        varchar,
    admin_claim_value       varchar,
    mfa_claim_path          varchar,
    mfa_claim_value         varchar,
    allow_insecure_requests bool    not null,
    use_pkce                bool    not null,
    root_pem                varchar
);

comment
on column auth_providers.secret is 'may be null and would still be safe, if pkce is being used';

-- modify users table with fk to auth_providers

alter table users
    add auth_provider_id varchar;

alter table users
    add federation_uid varchar;

alter table users
    add constraint users_federation_key
        unique (auth_provider_id, federation_uid);

alter table users
    add constraint users_auth_providers_id_fk
        foreign key (auth_provider_id) references auth_providers
            on update cascade on delete set null;

-- auth_provider_logos table

create table auth_provider_logos
(
    auth_provider_id varchar not null
        constraint auth_provider_logos_auth_providers_id_fk
            references auth_providers
            on update cascade on delete cascade,
    res              varchar not null,
    content_type     varchar not null,
    data             bytea   not null,
    constraint auth_provider_logos_pk
        primary key (auth_provider_id, res)
);
