ALTER TABLE clients
    ADD backchannel_logout_uri VARCHAR;

CREATE INDEX backchannel_logout_uri_index
    ON clients (backchannel_logout_uri);

CREATE TABLE failed_backchannel_logouts
(
    client_id   VARCHAR NOT NULL
        CONSTRAINT failed_backchannel_logouts_clients_id_fk
            REFERENCES clients
            ON DELETE CASCADE,
    sub         VARCHAR NOT NULL,
    sid         VARCHAR NOT NULL,
    retry_count INTEGER NOT NULL,
    CONSTRAINT failed_backchannel_logouts_pk
        PRIMARY KEY (client_id, sub, sid)
);

CREATE INDEX failed_backchannel_logouts_client_id_index
    ON failed_backchannel_logouts (client_id);

ALTER TABLE refresh_tokens
    ADD session_id VARCHAR;

CREATE INDEX refresh_tokens_session_id_index
    ON refresh_tokens (session_id);
