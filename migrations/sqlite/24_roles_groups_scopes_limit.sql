-- remove the limit checks on id and name fields for roles, groups, scopes
-- to allow longer scope names like i.e. 'urn:ietf:params:oauth:grant-type:device_code'

create table scopes_dg_tmp
(
    id                  varchar(36) not null
        constraint scopes_pk
            primary key,
    name                varchar     not null,
    attr_include_access varchar,
    attr_include_id     varchar
);

insert into scopes_dg_tmp(id, name, attr_include_access, attr_include_id)
select id, name, attr_include_access, attr_include_id
from scopes;

drop table scopes;

alter table scopes_dg_tmp
    rename to scopes;

create table groups_dg_tmp
(
    id   varchar not null
        constraint groups_pk
            primary key,
    name varchar not null
);

insert into groups_dg_tmp(id, name)
select id, name
from groups;

drop table groups;

alter table groups_dg_tmp
    rename to groups;

create table roles_dg_tmp
(
    id   varchar not null
        constraint roles_pk
            primary key,
    name varchar not null
);

insert into roles_dg_tmp(id, name)
select id, name
from roles;

drop table roles;

alter table roles_dg_tmp
    rename to roles;
