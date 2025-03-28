ALTER TABLE clients
    ADD backchannel_logout_uri VARCHAR;

CREATE INDEX backchannel_logout_uri_index
    ON clients (backchannel_logout_uri);