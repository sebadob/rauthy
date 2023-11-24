create table user_attr_values_dg_tmp
(
    user_id varchar not null
        constraint user_attr_values_users_id_fk
            references users
            on update cascade on delete cascade,
    key     varchar not null
        references user_attr_config
            on update cascade on delete cascade,
    value   blob    not null,
    constraint user_attr_values_pk
        primary key (user_id, key)
);

insert into user_attr_values_dg_tmp(user_id, key, value)
select user_id, key, value
from user_attr_values;

drop table user_attr_values;

alter table user_attr_values_dg_tmp
    rename to user_attr_values;
