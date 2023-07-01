create table colors
(
    client_id varchar(36) not null
        constraint colors_pk
        primary key
        constraint colors_clients_id_fk
        references clients
        on update cascade
        on delete cascade,
    data      blob not null
);

create table logos
(
    client_id varchar(36) not null
        constraint logos_pk
        primary key
        constraint logos_clients_id_fk
        references clients
        on update cascade
        on delete cascade,
    data     blob not null
);
