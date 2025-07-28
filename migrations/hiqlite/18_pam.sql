CREATE TABLE pam_groups
(
    id   INTEGER NOT NULL
        CONSTRAINT pam_groups_pk
            PRIMARY KEY AUTOINCREMENT,
    name TEXT    NOT NULL,
    typ  TEXT    NOT NULL
) STRICT;

CREATE UNIQUE INDEX pam_groups_name_uindex
    ON pam_groups (name);

CREATE TABLE pam_hosts
(
    id        TEXT    NOT NULL
        CONSTRAINT pam_hosts_pk
            PRIMARY KEY,
    hostname  TEXT    NOT NULL,
    gid       INTEGER NOT NULL
        CONSTRAINT pam_hosts_pam_groups_id_fk
            REFERENCES pam_groups,
    secret    BLOB    NOT NULL,
    force_mfa INTEGER NOT NULL,
    notes     TEXT
) STRICT;

CREATE INDEX pam_hosts_gid_index
    ON pam_hosts (gid);

CREATE UNIQUE INDEX pam_hosts_hostname_gid_uindex
    ON pam_hosts (hostname, gid);

CREATE TABLE pam_hosts_ips
(
    host_id TEXT NOT NULL
        CONSTRAINT pam_hosts_ips_pam_hosts_id_fk
            REFERENCES pam_hosts
            ON DELETE CASCADE,
    ip      TEXT NOT NULL,
    CONSTRAINT pam_hosts_ips_pk
        PRIMARY KEY (host_id, ip)
) STRICT;

CREATE INDEX pam_hosts_ips_ip_index
    ON pam_hosts_ips (ip);

CREATE TABLE pam_hosts_aliases
(
    host_id TEXT NOT NULL
        CONSTRAINT pam_hosts_aliases_pam_hosts_id_fk
            REFERENCES pam_hosts
            ON DELETE CASCADE,
    alias   TEXT NOT NULL,
    CONSTRAINT pam_hosts_aliases_pk
        PRIMARY KEY (host_id, alias)
) STRICT;

CREATE INDEX pam_hosts_aliases_alias_index
    ON pam_hosts_aliases (alias);

CREATE TABLE pam_users
(
    id    INTEGER NOT NULL
        CONSTRAINT pam_users_pk
            PRIMARY KEY AUTOINCREMENT,
    name  TEXT    NOT NULL,
    gid   INTEGER NOT NULL
        CONSTRAINT pam_users_pam_groups_id_fk
            REFERENCES pam_groups,
    email TEXT    not null
        CONSTRAINT pam_users_users_email_fk
            REFERENCES users (email)
            ON UPDATE CASCADE ON DELETE CASCADE,
    shell TEXT    NOT NULL
) STRICT;

CREATE INDEX pam_users_gid_index
    ON pam_users (gid);

CREATE UNIQUE INDEX pam_users_name_uindex
    ON pam_users (name);

CREATE UNIQUE INDEX pam_users_email_uindex
    ON pam_users (email);

CREATE TABLE pam_rel_groups_users
(
    gid   INTEGER NOT NULL
        CONSTRAINT pam_rel_groups_users_pam_groups_id_fk
            REFERENCES pam_groups
            ON DELETE CASCADE,
    uid   INTEGER NOT NULL
        CONSTRAINT pam_rel_groups_users_pam_users_id_fk
            REFERENCES pam_users
            ON DELETE CASCADE,
    wheel INTEGER NOT NULL,
    CONSTRAINT pam_rel_groups_users_pk
        PRIMARY KEY (gid, uid)
) STRICT;

-- make sure uid + gid sequences start at 100_000
INSERT INTO sqlite_sequence (name, seq)
VALUES ('pam_groups', 99999),
       ('pam_users', 99999);

-- add default groups
INSERT INTO pam_groups (name, typ)
VALUES ('wheel-rauthy', 'immutable'),
       ('default', 'host');
