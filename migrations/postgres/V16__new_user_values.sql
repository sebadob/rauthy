ALTER TABLE users_values
    ADD preferred_username VARCHAR;

ALTER TABLE users_values
    ADD tz VARCHAR;

CREATE UNIQUE INDEX users_values_preferred_username_uindex
    ON users_values (preferred_username);
