ALTER TABLE refresh_tokens
    ADD client_id VARCHAR;

CREATE INDEX refresh_tokens_user_id_client_id_index
    ON refresh_tokens (user_id, client_id);

-- NULL for clients created before this column existed
ALTER TABLE clients
    ADD created_micros BIGINT;

-- `clients.created_micros` of the client the token / device was issued for
ALTER TABLE refresh_tokens
    ADD client_created_micros BIGINT;

ALTER TABLE devices
    ADD client_created_micros BIGINT;

-- `RefreshToken::delete_by_user_client()` looks up the devices of a client
CREATE INDEX devices_client_id_index
    ON devices (client_id);
