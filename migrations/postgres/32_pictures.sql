CREATE TABLE pictures
(
    id           VARCHAR NOT NULL
        CONSTRAINT pictures_pk
            PRIMARY KEY,
    updated      BIGINT  NOT NULL,
    content_type VARCHAR NOT NULL,
    storage      VARCHAR NOT NULL,
    data         BYTEA
);

ALTER TABLE users
    ADD picture_id VARCHAR;
