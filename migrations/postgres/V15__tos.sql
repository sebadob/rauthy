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

CREATE TABLE tos_user_accept
(
    user_id  VARCHAR NOT NULL,
    tos_ts   BIGINT  NOT NULL,
    ts       BIGINT  NOT NULL,
    location VARCHAR NOT NULL,
    CONSTRAINT tos_user_accept_pk
        PRIMARY KEY (user_id, tos_ts)
);

CREATE UNIQUE INDEX tos_user_accept_user_id_tos_Ts_uindex
    ON tos_user_accept (user_id, tos_Ts);
