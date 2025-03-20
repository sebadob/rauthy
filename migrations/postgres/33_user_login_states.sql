CREATE TABLE user_login_states
(
    user_id    VARCHAR NOT NULL
        CONSTRAINT user_login_states_users_id_fk
            REFERENCES users
            ON DELETE RESTRICT,
    client_id  VARCHAR not null
        CONSTRAINT user_login_states_clients_id_fk
            REFERENCES clients
            ON DELETE RESTRICT,
    session_id VARCHAR
        CONSTRAINT user_login_states_sessions_id_fk
            REFERENCES sessions
            ON DELETE SET NULL,
    login_ts   BIGINT  not null,
    CONSTRAINT user_login_states_pk
        PRIMARY KEY (user_id, client_id, session_id)
);

CREATE INDEX user_login_states_session_id_index
    ON user_login_states (session_id);

CREATE INDEX user_login_states_user_id_index
    ON user_login_states (user_id);
