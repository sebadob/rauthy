CREATE TABLE pictures
(
    id           TEXT    NOT NULL
        CONSTRAINT pictures_pk
            PRIMARY KEY,
    updated      INTEGER NOT NULL,
    content_type TEXT    NOT NULL,
    storage      TEXT    NOT NULL,
    data         BLOB
) STRICT;

ALTER TABLE users
    ADD picture_id TEXT;