ALTER TABLE refresh_tokens
    ADD client_id VARCHAR;

CREATE INDEX refresh_tokens_user_id_client_id_index
    ON refresh_tokens (user_id, client_id);

ALTER TABLE refresh_tokens
    ADD client_generation VARCHAR;

ALTER TABLE clients
    ADD generation VARCHAR NOT NULL DEFAULT '';

ALTER TABLE devices
    ADD client_generation VARCHAR NOT NULL DEFAULT '';
