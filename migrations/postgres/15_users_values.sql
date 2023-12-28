create table users_values
(
    id        varchar not null
        constraint users_values_users_id_fk
            references users
            on update cascade on delete cascade,
    -- varchar instead of date because sqlite has no date type
    birthdate varchar,
    phone     varchar,
    street    varchar,
    zip       integer,
    city      varchar,
    country   varchar
);

comment on table users_values is 'holds additional, optional user values for some default OIDC claims';

alter table users_values
    add constraint users_values_pk
        primary key (id);
