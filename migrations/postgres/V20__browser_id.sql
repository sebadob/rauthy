DROP TABLE login_locations;

CREATE TABLE login_locations
(
    user_id    VARCHAR NOT NULL
        CONSTRAINT login_locations_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    browser_id VARCHAR NOT NULL,
    ip         VARCHAR NOT NULL,
    last_seen  BIGINT  NOT NULL,
    user_agent VARCHAR NOT NULL,
    location   VARCHAR,
    CONSTRAINT login_locations_pk
        PRIMARY KEY (user_id, ip, browser_id)
);

CREATE INDEX login_locations_last_seen_index
    ON login_locations (last_seen);
