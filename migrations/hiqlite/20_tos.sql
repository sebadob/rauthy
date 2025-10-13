CREATE TABLE tos
(
    ts      INTEGER NOT NULL
        CONSTRAINT tos_pk
            PRIMARY KEY,
    author  TEXT    NOT NULL,
    is_html INTEGER NOT NULL,
    content TEXT    NOT NULL
) STRICT;

CREATE UNIQUE INDEX tos_ts_uindex
    ON tos (ts DESC);
