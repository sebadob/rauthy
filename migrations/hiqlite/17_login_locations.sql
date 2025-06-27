CREATE TABLE login_locations
(
    user_id    TEXT    NOT NULL
        CONSTRAINT login_locations_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    ip         TEXT    NOT NULL,
    last_seen  INTEGER NOT NULL,
    user_agent TEXT    NOT NULL,
    location   TEXT,
    CONSTRAINT login_locations_pk
        PRIMARY KEY (user_id, ip)
) STRICT;

CREATE INDEX login_locations_ip_index
    ON login_locations (ip);

CREATE INDEX login_locations_last_seen_index
    ON login_locations (last_seen);

CREATE TABLE user_revoke
(
    user_id TEXT NOT NULL
        CONSTRAINT user_revoke_pk
            PRIMARY KEY
        CONSTRAINT user_revoke_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    code    TEXT NOT NULL
) STRICT;
