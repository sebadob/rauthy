create table users_values
(
    id        text not null
        constraint users_values_pk
            primary key
        constraint users_values_users_id_fk
            references users
            on update cascade on delete cascade,
    birthdate text,
    phone     text,
    street    text,
    zip       integer,
    city      text,
    country   text
);
