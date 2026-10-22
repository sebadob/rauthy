ALTER TABLE refresh_tokens
    ADD client_id VARCHAR;

CREATE INDEX refresh_tokens_user_id_client_id_index
    ON refresh_tokens (user_id, client_id);

ALTER TABLE clients
    ADD created_micros BIGINT;

ALTER TABLE refresh_tokens
    ADD client_created_micros BIGINT;

ALTER TABLE devices
    ADD client_created_micros BIGINT;

CREATE INDEX devices_client_id_index
    ON devices (client_id);
