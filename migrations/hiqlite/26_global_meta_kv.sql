CREATE TABLE kv_ns
(
    ns     TEXT NOT NULL
        CONSTRAINT kv_ns_pk
            PRIMARY KEY,
    public INTEGER
) STRICT;

CREATE TABLE kv_values
(
    ns        TEXT NOT NULL
        CONSTRAINT kv_values_kv_ns_ns_fk
            REFERENCES kv_ns
            ON UPDATE CASCADE ON DELETE CASCADE,
    key       TEXT NOT NULL,
    encrypted INTEGER,
    value     BLOB NOT NULL,
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
    secret  BLOB              NOT NULL,
    enabled INTEGER DEFAULT 1 NOT NULL,
    name    TEXT
) STRICT;

CREATE INDEX kv_access_ns_index
    ON kv_access (ns);

INSERT INTO kv_ns
VALUES ('default', false);
