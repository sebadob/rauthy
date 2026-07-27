ALTER TABLE users_values
    ADD preferred_username TEXT;

ALTER TABLE users_values
    ADD tz TEXT;

CREATE UNIQUE INDEX users_values_preferred_username_uindex
    ON users_values (preferred_username);
