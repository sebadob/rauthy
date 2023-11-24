alter table user_attr_values
    add constraint user_attr_values_users_id_fk
        foreign key (user_id) references users
            on update cascade on delete cascade;
