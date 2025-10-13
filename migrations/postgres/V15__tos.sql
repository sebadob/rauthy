CREATE TABLE tos
(
    ts      BIGINT  NOT NULL
        CONSTRAINT tos_pk
            PRIMARY KEY,
    author  VARCHAR NOT NULL,
    is_html BOOLEAN NOT NULL,
    content VARCHAR NOT NULL
);

CREATE UNIQUE INDEX tos_ts_uindex
    ON tos (ts DESC);
