CREATE TABLE ssh_auth_keys
(
    pam_uid  BIGINT  NOT NULL
        CONSTRAINT ssh_auth_keys_pam_users_id_fk
            REFERENCES pam_users
            ON UPDATE CASCADE ON DELETE CASCADE,
    ts_added BIGINT  NOT NULL,
    expires  BIGINT,
    typ      VARCHAR NOT NULL,
    data     VARCHAR NOT NULL,
    comment  VARCHAR NOT NULL,
    CONSTRAINT ssh_auth_keys_pk
        PRIMARY KEY (pam_uid, ts_added)
);

CREATE INDEX ssh_auth_keys_expires_index
    ON ssh_auth_keys (expires);

CREATE UNIQUE INDEX ssh_auth_keys_pam_uid_ts_added_uindex
    ON ssh_auth_keys (pam_uid, ts_added);

CREATE TABLE ssh_auth_keys_used
(
    used_key_hash TEXT    NOT NULL
        CONSTRAINT ssh_auth_keys_used_pk
            PRIMARY KEY,
    ts_added      INTEGER NOT NULL
) STRICT;

CREATE INDEX ssh_auth_keys_used_used_key_hash_index
    ON ssh_auth_keys_used (used_key_hash);

CREATE INDEX ssh_auth_keys_used_ts_added_index
    ON ssh_auth_keys_used (ts_added);
