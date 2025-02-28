CREATE TABLE themes
(
    client_id     TEXT    NOT NULL
        CONSTRAINT themes_pk
            PRIMARY KEY
        CONSTRAINT themes_clients_id_fk
            REFERENCES clients
            ON UPDATE CASCADE ON DELETE CASCADE,
    last_update   INTEGER NOT NULL,
    version       INTEGER NOT NULL,
    light         BLOB    NOT NULL,
    dark          BLOB    NOT NULL,
    border_radius TEXT    NOT NULL
) STRICT;

DROP TABLE colors;