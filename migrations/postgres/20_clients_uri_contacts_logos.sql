alter table clients
    add client_uri varchar;

alter table clients
    add contacts varchar;

drop table logos;

create table client_logos
(
    client_id    varchar not null
        constraint client_logos_client_id_fk
            references clients
            on update cascade on delete cascade,
    res          varchar not null,
    content_type varchar not null,
    data         bytea   not null,
    constraint client_logos_pk
        primary key (client_id, res)
);
