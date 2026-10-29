-- no FK on `client_id`: ephemeral clients don't exist in `clients`
CREATE TABLE user_consents
(
    user_id      VARCHAR NOT NULL
        CONSTRAINT user_consents_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    client_id    VARCHAR NOT NULL,
    resource     VARCHAR NOT NULL,
    scopes       VARCHAR NOT NULL,
    content_hash VARCHAR NOT NULL,
    created      BIGINT  NOT NULL,
    updated      BIGINT  NOT NULL,
    ip           VARCHAR NOT NULL,
    CONSTRAINT user_consents_pk
        PRIMARY KEY (user_id, client_id, resource)
);

CREATE INDEX user_consents_client_id_index
    ON user_consents (client_id);
