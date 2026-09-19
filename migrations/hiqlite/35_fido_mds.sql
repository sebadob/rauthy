CREATE TABLE fido_mds_certs
(
    hash BLOB NOT NULL
        CONSTRAINT fido_mds_certs_pk
            PRIMARY KEY,
    der  BLOB NOT NULL
) STRICT;

CREATE TABLE fido_mds_entries
(
    aaguid            BLOB    NOT NULL
        CONSTRAINT fido_mds_entries_pk
            PRIMARY KEY,
    description       TEXT    NOT NULL,
    key_protection    INTEGER NOT NULL,
    attachment_hint   INTEGER NOT NULL,
    attestation_types INTEGER NOT NULL,
    cert_level        INTEGER NOT NULL
) STRICT;

CREATE TABLE fido_mds_entry_certs
(
    aaguid    BLOB NOT NULL
        CONSTRAINT fido_mds_entry_certs_entry_fk
            REFERENCES fido_mds_entries
            ON UPDATE CASCADE ON DELETE CASCADE,
    cert_hash BLOB NOT NULL
        CONSTRAINT fido_mds_entry_certs_cert_fk
            REFERENCES fido_mds_certs (hash)
            ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT fido_mds_entry_certs_pk
        PRIMARY KEY (aaguid, cert_hash)
) STRICT;

ALTER TABLE passkeys
    ADD aaguid BLOB
        CONSTRAINT passkeys_fido_mds_entries_fk
            REFERENCES fido_mds_entries
            ON UPDATE CASCADE ON DELETE SET NULL;
