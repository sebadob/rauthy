CREATE TABLE avatars
(
    id           TEXT    NOT NULL
        CONSTRAINT avatars_pk
            PRIMARY KEY,
    updated      INTEGER NOT NULL,
    content_type TEXT    NOT NULL,
    storage      TEXT    NOT NULL,
    data         BLOB
) STRICT;

ALTER TABLE users
    ADD avatar_id TEXT;