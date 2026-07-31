create table one_time_password
(
    id          varchar    not null
        constraint one_time_password_pk
            primary key,
    user_id     varchar    not null
        references users
            on update cascade on delete cascade,
    name        varchar    null,
    secret      bytea    not null,
    last_used   bigint not null,
    kind        varchar    not null,
    is_active   boolean default false not null
);
