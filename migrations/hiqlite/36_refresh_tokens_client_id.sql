ALTER TABLE refresh_tokens
    ADD client_id TEXT;

CREATE INDEX refresh_tokens_user_id_client_id_index
    ON refresh_tokens (user_id, client_id);

ALTER TABLE refresh_tokens
    ADD client_generation TEXT;

ALTER TABLE clients
    ADD generation TEXT NOT NULL DEFAULT '';

ALTER TABLE devices
    ADD client_generation TEXT NOT NULL DEFAULT '';
