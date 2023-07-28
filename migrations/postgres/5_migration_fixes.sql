alter table groups
    alter column name set not null;

alter table magic_links
    alter column used set not null;

alter table recent_passwords
    alter column passwords set not null;

alter table user_attr_values
    alter column value set not null;
