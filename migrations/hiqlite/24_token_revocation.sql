CREATE TABLE revoked_tokens
(
    jti TEXT    NOT NULL
        CONSTRAINT revoked_tokens_pk
            PRIMARY KEY,
    exp INTEGER NOT NULL
) STRICT;

CREATE INDEX revoked_tokens_jti_index
    ON revoked_tokens (jti);

CREATE INDEX revoked_tokens_exp_index
    ON revoked_tokens (exp);

ALTER TABLE refresh_tokens
    ADD access_token_jti TEXT;

CREATE INDEX refresh_tokens_access_token_jti_index
    ON refresh_tokens (access_token_jti);

ALTER TABLE refresh_tokens_devices
    ADD access_token_jti TEXT;

CREATE INDEX refresh_tokens_devices_access_token_jti_index
    ON refresh_tokens_devices (access_token_jti);
