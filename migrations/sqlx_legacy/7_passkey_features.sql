alter table users
    add webauthn_user_id varchar;

alter table users
    add constraint users_pk2
        unique (webauthn_user_id);

-- This index was a duplicate -> there is one anyway from the 2nd PK on email
drop index users_email_uindex;

create table passkeys
(
    user_id         varchar not null
        constraint passkeys_users_id_fk
            references users
            on update cascade on delete cascade,
    name            varchar not null,
    passkey_user_id varchar    not null
        constraint passkeys_users_webauthn_user_id_fk
            references users (webauthn_user_id)
            on update cascade on delete cascade,
    passkey         varchar not null,
    credential_id   bytea   not null,
    registered      bigint  not null,
    last_used       bigint  not null,
    constraint passkeys_pk
        primary key (user_id, name)
);

create index passkeys_credential_id_index
    on passkeys (credential_id);
