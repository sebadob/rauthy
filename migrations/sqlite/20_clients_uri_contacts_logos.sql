-- modify the clients table and add 'force_mfa'

alter table clients
    rename to clients_old;

create table clients
(
    id                        varchar not null
        constraint clients_pk
            primary key,
    name                      varchar,
    enabled                   bool    not null,
    confidential              bool    not null,
    secret                    blob,
    secret_kid                varchar,
    redirect_uris             varchar not null,
    post_logout_redirect_uris varchar,
    allowed_origins           varchar,
    flows_enabled             varchar not null,
    access_token_alg          varchar not null,
    id_token_alg              varchar not null,
    refresh_token             bool    not null,
    auth_code_lifetime        integer not null,
    access_token_lifetime     integer not null,
    scopes                    varchar not null,
    default_scopes            varchar not null,
    challenge                 varchar,
    force_mfa                 bool    not null,
    client_uri                varchar,
    contacts                  varchar
);

insert into clients(id,
                    name,
                    enabled,
                    confidential,
                    secret,
                    secret_kid,
                    redirect_uris,
                    post_logout_redirect_uris,
                    allowed_origins,
                    flows_enabled,
                    access_token_alg,
                    id_token_alg,
                    refresh_token,
                    auth_code_lifetime,
                    access_token_lifetime,
                    scopes,
                    default_scopes,
                    challenge,
                    force_mfa)
select id,
       name,
       enabled,
       confidential,
       secret,
       secret_kid,
       redirect_uris,
       post_logout_redirect_uris,
       allowed_origins,
       flows_enabled,
       access_token_alg,
       id_token_alg,
       refresh_token,
       auth_code_lifetime,
       access_token_lifetime,
       scopes,
       default_scopes,
       challenge,
       false as force_mfa
from clients_old;

-- recreate all tables with foreign keys to clients

-- clients_dyn

alter table clients_dyn
    rename to clients_dyn_old;

drop index clients_dyn_last_used_index;

create table clients_dyn
(
    id                         text    not null
        constraint clients_dyn_pk
            primary key
        constraint clients_dyn_clients_id_fk
            references clients
            on update cascade on delete cascade,
    created                    integer not null,
    last_used                  integer,
    registration_token         blob    not null,
    token_endpoint_auth_method text    not null
);

create index clients_dyn_last_used_index
    on clients_dyn (last_used);

insert into clients_dyn(id, created, last_used, registration_token, token_endpoint_auth_method)
select id, created, last_used, registration_token, token_endpoint_auth_method
from clients_dyn_old;

-- colors
alter table colors
    rename to colors_old;

create table colors
(
    client_id varchar not null
        constraint colors_pk
            primary key
        constraint colors_clients_id_fk
            references clients
            on update cascade
            on delete cascade,
    data      blob    not null
);

insert into colors(client_id, data)
select client_id, data
from colors_old;

-- logos
alter table logos
    rename to logos_old;

create table logos
(
    client_id varchar not null
        constraint logos_pk
            primary key
        constraint logos_clients_id_fk
            references clients
            on update cascade
            on delete cascade,
    data      blob    not null
);

insert into logos(client_id, data)
select client_id, data
from logos_old;

-- finally, drop all the old tables

drop table clients_dyn_old;
drop table colors_old;
drop table logos_old;
drop table clients_old;

-- re-create the logos table

drop table logos;

create table client_logos
(
    client_id    varchar not null
        constraint client_logos_client_id_fk
            references clients
            on update cascade on delete cascade,
    res          varchar not null,
    content_type varchar not null,
    data         blob    not null,
    constraint client_logos_pk
        primary key (client_id, res)
);
