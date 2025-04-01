CREATE TABLE user_login_states
(
    user_id    TEXT    NOT NULL
        CONSTRAINT user_login_states_users_id_fk
            REFERENCES users
            ON DELETE RESTRICT,
    client_id  TEXT    not null
        CONSTRAINT user_login_states_clients_id_fk
            REFERENCES clients
            ON DELETE RESTRICT,
    session_id TEXT
        CONSTRAINT user_login_states_sessions_id_fk
            REFERENCES sessions
            ON DELETE CASCADE,
    login_ts   INTEGER NOT NULL,
    CONSTRAINT user_login_states_pk
        PRIMARY KEY (user_id, client_id, session_id)
) STRICT;

CREATE INDEX user_login_states_session_id_index
    ON user_login_states (session_id);

CREATE INDEX user_login_states_user_id_index
    ON user_login_states (user_id);
