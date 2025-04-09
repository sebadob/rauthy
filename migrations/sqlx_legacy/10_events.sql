create table events
(
    id        varchar  not null,
    timestamp bigint   not null,
    typ       smallint not null,
    level     smallint not null,
    ip        varchar,
    constraint events_pk
        primary key (id, timestamp),
    data      bigint,
    text      varchar
);

create index events_timestamp_index
    on events (timestamp desc);
