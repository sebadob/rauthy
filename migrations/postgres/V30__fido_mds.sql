CREATE TABLE fido_mds_metadata
(
    current_blob_no BIGINT NOT NULL
        CONSTRAINT fido_mds_metadata_pk
            PRIMARY KEY,
    next_update     BIGINT NOT NULL
);

CREATE TABLE fido_mds_certs
(
    hash     BYTEA NOT NULL
        CONSTRAINT fido_mds_certs_pk
            PRIMARY KEY,
    cert_der BYTEA NOT NULL
);

CREATE TABLE fido_mds_entries
(
    aaguid            BYTEA    NOT NULL
        CONSTRAINT fido_mds_entries_pk
            PRIMARY KEY,
    description       VARCHAR  NOT NULL,
    key_protection    BIGINT   NOT NULL,
    attachment_hint   BIGINT   NOT NULL,
    attestation_types BIGINT   NOT NULL,
    cert_level        SMALLINT NOT NULL
);

CREATE TABLE fido_mds_entry_certs
(
    aaguid    BYTEA NOT NULL
        CONSTRAINT fido_mds_entry_certs_entry_fk
            REFERENCES fido_mds_entries
            ON UPDATE CASCADE ON DELETE CASCADE,
    cert_hash BYTEA NOT NULL
        CONSTRAINT fido_mds_entry_certs_cert_fk
            REFERENCES fido_mds_certs (hash)
            ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT fido_mds_entry_certs_pk
        PRIMARY KEY (aaguid, cert_hash)
);

ALTER TABLE passkeys
    ADD aaguid BYTEA
        CONSTRAINT passkeys_fido_mds_entries_fk
            REFERENCES fido_mds_entries
            ON UPDATE CASCADE ON DELETE SET NULL;
