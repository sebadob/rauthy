-- modify the clients table and remove obsolete `refresh_token` column
-- the new logic will just take a look at the flows_enabled value

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

-- client_logos
alter table client_logos
    rename to client_logos_old;

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
insert into client_logos(client_id, res, content_Type, data)
select client_id, res, content_Type, data
from client_logos_old;

-- devices
alter table devices
    rename to devices_old;

drop index devices_access_exp_refresh_exp_index;

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

insert into devices(id, client_id, user_id, created, access_exp, refresh_exp, peer_ip, name)
select id,
       client_id,
       user_id,
       created,
       access_exp,
       refresh_exp,
       peer_ip,
       name
from devices_old;

-- refresh_tokens_devices
alter table refresh_tokens_devices
    rename to refresh_tokens_devices_old;

drop index refresh_tokens_devices_exp_index;

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

insert into refresh_tokens_devices(id, device_id, user_id, nbf, exp, scope)
select id, device_id, user_id, nbf, exp, scope
from refresh_tokens_devices_old;

-- finally, drop all the old tables

drop table clients_dyn_old;
drop table colors_old;
drop table client_logos_old;
drop table clients_old;
drop table devices_old;
drop table refresh_tokens_devices_old;
