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
    peer_ip     varchar not null
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
    nbf       bigint  not null,
    exp       bigint  not null,
    scope     varchar
);

create index refresh_tokens_devices_exp_index
    on refresh_tokens_devices (exp);
