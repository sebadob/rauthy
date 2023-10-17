create table events
(
    id        varchar     not null,
    timestamp timestamptz not null,
    typ       varchar     not null,
    level     smallint    not null,
    ip        varchar,
    constraint events_pk
        primary key (id, timestamp)
);

create index events_timestamp_index
    on events (timestamp desc);
