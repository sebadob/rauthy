CREATE TABLE pictures
(
    id           VARCHAR NOT NULL
        CONSTRAINT pictures_pk
            PRIMARY KEY,
    content_type VARCHAR NOT NULL,
    storage      VARCHAR NOT NULL,
    data         BYTEA
);

ALTER TABLE users
    ADD picture_id VARCHAR;
