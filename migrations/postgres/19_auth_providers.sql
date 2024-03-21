-- create ne auth_providers table for upstream auth providers

create table auth_providers
(
    id                      varchar not null
        constraint auth_providers_pk
            primary key,
    name                    varchar not null,
    issuer                  varchar not null,
    authorization_endpoint  varchar not null,
    token_endpoint          varchar not null,
    userinfo_endpoint       varchar not null,
    client_id               varchar not null,
    secret                  bytea,
    scope                   varchar not null,
    token_auth_method_basic bool    not null,
    use_pkce                bool    not null,
    root_pem                varchar,
    logo                    bytea,
    logo_type               varchar
);

comment on column auth_providers.secret is 'may be null and would still be safe, if pkce is being used';

-- modify users table with fk to auth_providers

alter table users
    add auth_provider_id varchar;

alter table users
    add constraint users_auth_providers_id_fk
        foreign key (auth_provider_id) references auth_providers (id)
            on update cascade on delete set null;
