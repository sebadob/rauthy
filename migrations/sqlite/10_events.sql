create table events
(
    id        varchar  not null,
    timestamp bigint   not null,
    typ       smallint not null,
    level     smallint not null,
    ip        varchar,
    data      bigint,
    text      varchar,
    constraint events_pk
        primary key (id, timestamp)
);

create index events_timestamp_index
    on events (timestamp desc);
