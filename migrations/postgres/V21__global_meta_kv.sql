CREATE TABLE kv_ns
(
    ns VARCHAR NOT NULL
        CONSTRAINT kv_ns_pk
            PRIMARY KEY
);

create table kv_values
(
    ns        VARCHAR not null
        constraint kv_values_kv_ns_ns_fk
            references kv_ns
            on update cascade on delete cascade,
    key       VARCHAR not null,
    encrypted BOOLEAN not null,
    value     BYTEA   not null,
    CONSTRAINT kv_values_pk
        PRIMARY KEY (ns, key)
);

CREATE TABLE kv_access
(
    id      VARCHAR              NOT NULL
        CONSTRAINT kv_access_pk
            PRIMARY KEY,
    ns      VARCHAR              NOT NULL
        CONSTRAINT kv_access_kv_ns_ns_fk
            REFERENCES kv_ns
            ON UPDATE CASCADE ON DELETE CASCADE,
    secret  BYTEA                NOT NULL,
    enabled BOOLEAN DEFAULT true NOT NULL,
    name    VARCHAR
);

CREATE INDEX kv_access_ns_index
    ON kv_access (ns);

INSERT INTO kv_ns
VALUES ('default')
