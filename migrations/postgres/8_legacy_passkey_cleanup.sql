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
