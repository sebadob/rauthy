CREATE TABLE one_time_password
(
    id          TEXT    NOT NULL
        CONSTRAINT one_time_password_pk
            PRIMARY KEY,
    user_id     TEXT    NOT NULL
        REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    name        TEXT    NULL,
    secret      BLOB    NOT NULL,
    last_used   INTEGER NOT NULL,
    kind        TEXT    NOT NULL,
    is_active   INTEGER DEFAULT false NOT NULL
) STRICT;

CREATE INDEX one_time_password_kind_index
    ON one_time_password (kind);
