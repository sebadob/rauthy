CREATE TABLE issued_tokens
(
    jti     TEXT    NOT NULL
        CONSTRAINT issued_tokens_pk
            PRIMARY KEY,
    user_id TEXT,
    did     TEXT,
    sid     TEXT,
    exp     INTEGER NOT NULL,
    revoked INTEGER
) STRICT;

CREATE INDEX issued_tokens_jti_index
    ON issued_tokens (jti);

CREATE INDEX issued_tokens_exp_index
    ON issued_tokens (exp);

ALTER TABLE refresh_tokens
    ADD access_token_jti TEXT;

CREATE INDEX refresh_tokens_access_token_jti_index
    ON refresh_tokens (access_token_jti);

ALTER TABLE refresh_tokens_devices
    ADD access_token_jti TEXT;

CREATE INDEX refresh_tokens_devices_access_token_jti_index
    ON refresh_tokens_devices (access_token_jti);
