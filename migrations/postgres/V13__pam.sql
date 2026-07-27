CREATE TABLE pam_groups
(
    id   bigserial NOT NULL
        CONSTRAINT pam_groups_pk
            PRIMARY KEY,
    name varchar   NOT NULL,
    typ  varchar   NOT NULL
);

create unique index pam_groups_name_uindex
    on pam_groups (name);

CREATE TABLE pam_hosts
(
    id                  VARCHAR NOT NULL
        CONSTRAINT pam_hosts_pk
            PRIMARY KEY,
    hostname            varchar NOT NULL,
    gid                 bigint  NOT NULL
        constraint pam_hosts_pam_groups_id_fk
            references pam_groups,
    secret              bytea   NOT NULL,
    force_mfa           bool    NOT NULL,
    local_password_only bool    NOT NULL,
    notes               varchar
);

CREATE INDEX pam_hosts_gid_index
    ON pam_hosts (gid);

CREATE UNIQUE INDEX pam_hosts_hostname_gid_uindex
    ON pam_hosts (hostname, gid);

CREATE TABLE pam_hosts_ips
(
    host_id varchar NOT NULL
        CONSTRAINT pam_hosts_ips_pam_hosts_id_fk
            REFERENCES pam_hosts
            ON DELETE CASCADE,
    ip      varchar NOT NULL,
    CONSTRAINT pam_hosts_ips_pk
        PRIMARY KEY (host_id, ip)
);

CREATE INDEX pam_hosts_ips_ip_index
    ON pam_hosts_ips (ip);

CREATE TABLE pam_hosts_aliases
(
    host_id varchar NOT NULL
        CONSTRAINT pam_hosts_aliases_pam_hosts_id_fk
            REFERENCES pam_hosts
            ON DELETE CASCADE,
    alias   varchar NOT NULL,
    CONSTRAINT pam_hosts_aliases_pk
        PRIMARY KEY (host_id, alias)
);

CREATE INDEX pam_hosts_aliases_alias_index
    ON pam_hosts_aliases (alias);

CREATE TABLE pam_users
(
    id    bigserial
        CONSTRAINT pam_users_pk
            PRIMARY KEY,
    name  varchar NOT NULL,
    gid   bigint  NOT NULL
        CONSTRAINT pam_users_pam_groups_id_fk
            REFERENCES pam_groups,
    email varchar NOT NULL
        CONSTRAINT pam_users_users_email_fk
            REFERENCES users (email)
            ON UPDATE CASCADE ON DELETE CASCADE,
    shell varchar NOT NULL
);

CREATE UNIQUE INDEX pam_users_email_uindex
    ON pam_users (email);

CREATE INDEX pam_users_gid_index
    ON pam_users (gid);

CREATE UNIQUE INDEX pam_users_name_uindex
    ON pam_users (name);

CREATE TABLE pam_rel_groups_users
(
    gid   bigint NOT NULL
        CONSTRAINT pam_rel_groups_users_pam_groups_id_fk
            REFERENCES pam_groups
            ON DELETE CASCADE,
    uid   bigint NOT NULL
        CONSTRAINT pam_rel_groups_users_pam_users_id_fk
            REFERENCES pam_users
            ON DELETE CASCADE,
    wheel bool   NOT NULL,
    CONSTRAINT pam_rel_groups_users_pk
        PRIMARY KEY (gid, uid)
);

-- make sure uid + gid sequences start at 100_000
ALTER SEQUENCE pam_groups_id_seq RESTART WITH 100000;
ALTER SEQUENCE pam_users_id_seq RESTART WITH 100000;

-- add default groups
INSERT INTO pam_groups (name, typ)
VALUES ('wheel-rauthy', 'immutable'),
       ('default', 'host');
