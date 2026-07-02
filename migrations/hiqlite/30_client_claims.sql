ALTER TABLE clients
    ADD claims BLOB;

ALTER TABLE clients
    ADD claims_at_root INTEGER DEFAULT 0 NOT NULL;
