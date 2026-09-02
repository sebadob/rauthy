CREATE TABLE one_time_password
(
    id        VARCHAR               NOT NULL
        CONSTRAINT one_time_password_pk
            PRIMARY KEY,
    user_id   VARCHAR               NOT NULL
        references users
            ON DELETE CASCADE,
    name      VARCHAR,
    secret    BYTEA NUT             NOT NULL,
    last_used BIGINT
                                    NOT NULL,
    kind      VARCHAR               NOT NULL,
    is_active BOOLEAN DEFAULT false NOT NULL
);

CREATE INDEX one_time_password_user_id_index
    ON one_time_password (user_id);
