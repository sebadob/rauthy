CREATE TABLE user_login_states
(
    timestamp  BIGINT  NOT NULL,
    user_id    VARCHAR NOT NULL
        CONSTRAINT table_name_users_id_fk
            REFERENCES users
            ON DELETE RESTRICT,
    client_id  VARCHAR NOT NULL
        CONSTRAINT table_name_clients_id_fk
            REFERENCES clients
            ON DELETE RESTRICT,
    session_id VARCHAR,
    CONSTRAINT table_name_pk
        PRIMARY KEY (timestamp, user_id)
);

CREATE INDEX user_login_states_session_id_index
    ON user_login_states (session_id);

CREATE UNIQUE INDEX user_login_states_user_id_client_id_session_id_uindex
    ON user_login_states (user_id, client_id, session_id);
