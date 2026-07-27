CREATE TABLE tos
(
    ts        INTEGER NOT NULL
        CONSTRAINT tos_pk
            PRIMARY KEY,
    author    TEXT    NOT NULL,
    is_html   INTEGER NOT NULL,
    opt_until INTEGER,
    content   TEXT    NOT NULL
) STRICT;

CREATE UNIQUE INDEX tos_ts_uindex
    ON tos (ts DESC);

CREATE TABLE tos_user_accept
(
    user_id   TEXT    NOT NULL
        CONSTRAINT tos_user_accept_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    tos_ts    INTEGER NOT NULL
        CONSTRAINT tos_user_accept_tos_ts_fk
            REFERENCES tos
            ON UPDATE CASCADE ON DELETE CASCADE,
    accept_ts INTEGER NOT NULL,
    location  TEXT    NOT NULL,
    CONSTRAINT tos_user_accept_pk
        PRIMARY KEY (user_id, tos_ts)
) STRICT;

CREATE UNIQUE INDEX tos_user_accept_user_id_tos_Ts_uindex
    ON tos_user_accept (user_id, tos_Ts);
