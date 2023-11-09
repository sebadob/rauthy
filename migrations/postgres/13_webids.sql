create table webids
(
    user_id varchar not null
        constraint webids_pk
            primary key
        constraint webids_users_id_fk
            references users
            on update cascade on delete cascade,
    is_open bool    not null,
    data    bytea
);
