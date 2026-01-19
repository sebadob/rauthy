CREATE TABLE issued_tokens
(
    jti     VARCHAR NOT NULL
        CONSTRAINT issued_tokens_pk
            PRIMARY KEY,
    user_id VARCHAR,
    did     VARCHAR,
    sid     VARCHAR,
    exp     BIGINT  NOT NULL,
    revoked BOOL
);

CREATE INDEX issued_tokens_user_id_index
    ON issued_tokens (user_id);

CREATE INDEX issued_tokens_sid_index
    ON issued_tokens (sid);

CREATE INDEX issued_tokens_exp_index
    ON issued_tokens (exp);

ALTER TABLE refresh_tokens
    ADD access_token_jti VARCHAR;

CREATE INDEX refresh_tokens_access_token_jti_index
    ON refresh_tokens (access_token_jti);

ALTER TABLE refresh_tokens_devices
    ADD access_token_jti VARCHAR;

CREATE INDEX refresh_tokens_devices_access_token_jti_index
    ON refresh_tokens_devices (access_token_jti);
