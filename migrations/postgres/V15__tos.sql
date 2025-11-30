CREATE TABLE tos
(
    ts        BIGINT  NOT NULL
        CONSTRAINT tos_pk
            PRIMARY KEY,
    author    VARCHAR NOT NULL,
    is_html   BOOLEAN NOT NULL,
    opt_until BIGINT,
    content   VARCHAR NOT NULL
);

CREATE UNIQUE INDEX tos_ts_uindex
    ON tos (ts DESC);

CREATE TABLE tos_user_accept
(
    user_id   VARCHAR NOT NULL
        CONSTRAINT tos_user_accept_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    tos_ts    BIGINT  NOT NULL
        CONSTRAINT tos_user_accept_tos_ts_fk
            REFERENCES tos
            ON UPDATE CASCADE ON DELETE CASCADE,
    accept_ts BIGINT  NOT NULL,
    location  VARCHAR NOT NULL,
    CONSTRAINT tos_user_accept_pk
        PRIMARY KEY (user_id, tos_ts)
);

CREATE UNIQUE INDEX tos_user_accept_user_id_tos_Ts_uindex
    ON tos_user_accept (user_id, tos_Ts);
