create table passkeys
(
    user_id varchar not null
        constraint passkeys_users_id_fk
            references users
            on update cascade on delete cascade,
    name    varchar not null,
    passkey varchar not null,
    constraint passkeys_pk
        primary key (user_id, name)
);

