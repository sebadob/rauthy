-- no FK on `client_id`: ephemeral clients don't exist in `clients`
CREATE TABLE user_consents
(
    user_id      TEXT    NOT NULL
        CONSTRAINT user_consents_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    client_id    TEXT    NOT NULL,
    resource     TEXT    NOT NULL,
    scopes       TEXT    NOT NULL,
    content_hash TEXT    NOT NULL,
    created      INTEGER NOT NULL,
    updated      INTEGER NOT NULL,
    ip           TEXT    NOT NULL,
    CONSTRAINT user_consents_pk
        PRIMARY KEY (user_id, client_id, resource)
) STRICT;

CREATE INDEX user_consents_client_id_index
    ON user_consents (client_id);
