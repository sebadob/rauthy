CREATE TABLE users_values_dg_tmp
(
    id        TEXT NOT NULL
        CONSTRAINT users_values_pk
            PRIMARY KEY
        CONSTRAINT users_values_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    birthdate TEXT,
    phone     TEXT,
    street    TEXT,
    zip       TEXT,
    city      TEXT,
    country   TEXT
) STRICT;

INSERT INTO users_values_dg_tmp(id, birthdate, phone, street, zip, city, country)
SELECT id, birthdate, phone, street, zip, city, country
FROM users_values;

DROP TABLE users_values;

ALTER TABLE users_values_dg_tmp
    RENAME TO users_values;
