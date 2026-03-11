CREATE TABLE kv_ns
(
    ns TEXT NOT NULL
        CONSTRAINT kv_ns_pk
            PRIMARY KEY
) STRICT;

create table kv_values
(
    ns        TEXT    not null
        constraint kv_values_kv_ns_ns_fk
            references kv_ns
            on update cascade on delete cascade,
    key       TEXT    not null,
    encrypted INTEGER not null,
    value     BLOB    not null,
    CONSTRAINT kv_values_pk
        PRIMARY KEY (ns, key)
) STRICT;

CREATE TABLE kv_access
(
    id      TEXT              NOT NULL
        CONSTRAINT kv_access_pk
            PRIMARY KEY,
    ns      TEXT              NOT NULL
        CONSTRAINT kv_access_kv_ns_ns_fk
            REFERENCES kv_ns
            ON UPDATE CASCADE ON DELETE CASCADE,
    enabled INTEGER DEFAULT 1 NOT NULL,
    name    TEXT
) STRICT;

CREATE INDEX kv_access_ns_index
    ON kv_access (ns);

INSERT INTO kv_ns
VALUES ('default');
