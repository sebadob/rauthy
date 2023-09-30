alter table users
    drop column mfa_app;

alter table users
    drop column sec_key_1;

alter table users
    drop column sec_key_2;

alter table users
    add user_expires bigint;

alter table magic_links
    add usage varchar not null default 'password_reset';

alter table magic_links
    alter column usage drop default ;

alter table sessions
    add remote_ip varchar;

drop table mfa_apps;
drop table webauthn;

-- fix some indexes -> unique

drop index passkeys_credential_id_index;

create unique index passkeys_credential_id_uindex
    on passkeys (credential_id);

create unique index passkeys_passkey_user_id_uindex
    on passkeys (passkey_user_id);

-- add 'uv' to passkeys
alter table passkeys
    add uv bool;
