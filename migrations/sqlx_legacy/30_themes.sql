CREATE TABLE themes
(
    client_id     VARCHAR NOT NULL
        CONSTRAINT themes_pk
            PRIMARY KEY
        CONSTRAINT themes_clients_id_fk
            REFERENCES clients
            ON UPDATE CASCADE ON DELETE CASCADE,
    last_update   BIGINT  NOT NULL,
    version       INT     NOT NULL,
    light         BYTEA   NOT NULL,
    dark          BYTEA   NOT NULL,
    border_radius VARCHAR NOT NULL
);

DROP TABLE colors;