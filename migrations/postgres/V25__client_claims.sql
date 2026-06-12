ALTER TABLE clients
    ADD claims BYTEA;

ALTER TABLE clients
    ADD claims_at_root bool DEFAULT false NOT NULL;
