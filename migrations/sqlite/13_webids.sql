create table webids
(
    user_id        varchar not null
        constraint webids_pk
            primary key
        constraint webids_users_id_fk
            references users
            on update cascade on delete cascade,
    custom_triples text,
    expose_email   bool    not null
);
