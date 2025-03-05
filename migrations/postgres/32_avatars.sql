CREATE TABLE avatars
(
    id           VARCHAR NOT NULL
        CONSTRAINT avatars_pk
            PRIMARY KEY,
    updated      BIGINT  NOT NULL,
    content_type VARCHAR NOT NULL,
    storage      VARCHAR NOT NULL,
    data         BYTEA
);

ALTER TABLE users
    ADD avatar_id VARCHAR;
