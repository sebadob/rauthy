create table colors
(
    client_id varchar not null
        constraint colors_pk
            primary key
        constraint colors_clients_id_fk
            references clients
            on update cascade on delete cascade,
    data      bytea   not null
);

create table logos
(
    client_id varchar not null
        constraint logos_pk
            primary key
        constraint logos_clients_id_fk
            references clients
            on update cascade on delete cascade,
    data      text    not null
);
