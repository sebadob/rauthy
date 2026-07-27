DROP TABLE login_locations;

CREATE TABLE login_locations
(
    user_id    TEXT    NOT NULL
        CONSTRAINT login_locations_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    browser_id TEXT    NOT NULL,
    ip         TEXT    NOT NULL,
    last_seen  INTEGER NOT NULL,
    user_agent TEXT    NOT NULL,
    location   TEXT,
    CONSTRAINT login_locations_pk
        PRIMARY KEY (user_id, ip, browser_id)
) STRICT;

CREATE INDEX login_locations_last_seen_index
    ON login_locations (last_seen);
