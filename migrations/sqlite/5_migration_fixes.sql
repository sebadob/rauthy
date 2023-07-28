alter table groups
    rename to groups_old;

create table groups
(
    id   varchar(36) not null
        constraint groups_pk
            primary key,
    name varchar(32) not null
);

insert into groups(id, name)
select id, name
from groups_old;

drop table groups_old;

--

alter table jwks
    rename to jwks_old;

create table jwks
(
    kid        varchar not null
        constraint jwks_pk
            primary key,
    created_at integer not null,
    signature  varchar not null,
    enc_key_id varchar not null,
    jwk        blob    not null
);

insert into jwks(kid, created_at, signature, enc_key_id, jwk)
select kid, created_at, signature, enc_key_id, jwk
from jwks_old;

drop table jwks_old;

--

alter table magic_links
    rename to magic_links_old;

drop index magic_links_exp_index;
drop index magic_links_user_id_index;

create table magic_links
(
    id         varchar(64) not null
        constraint magic_links_pk
            primary key,
    user_id    varchar(36) not null
        references users
            on delete cascade
            on update cascade,
    csrf_token varchar(48) not null,
    cookie     varchar(48),
    exp        integer     not null,
    used       bool        not null
);

create index magic_links_exp_index
    on magic_links (exp);

create index magic_links_user_id_index
    on magic_links (user_id);

insert into magic_links(id, user_id, csrf_token, cookie, exp, used)
select id, user_id, csrf_token, cookie, exp, used
from magic_links_old;

drop table magic_links_old;

--

alter table recent_passwords
    rename to recent_passwords_old;

create table recent_passwords
(
    user_id   varchar(36)   not null
        references users
            on delete cascade
            on update cascade
        constraint recent_passwords_pk
            primary key,
    passwords varchar(1024) not null
);

insert into recent_passwords(user_id, passwords)
select user_id, passwords
from recent_passwords_old;

drop table recent_passwords_old;

--

alter table user_attr_values
    rename to user_attr_values_old;

create table user_attr_values
(
    user_id varchar not null,
    key     varchar not null
        references user_attr_config
            on delete cascade
            on update cascade,
    value   blob    not null,
    constraint user_attr_values_pk
        primary key (user_id, key)
);

insert into user_attr_values(user_id, key, value)
select user_id, key, value
from user_attr_values_old;

drop table user_attr_values_old;
