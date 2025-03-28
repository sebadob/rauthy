ALTER TABLE clients
    ADD backchannel_logout_uri TEXT;

CREATE INDEX backchannel_logout_uri_index
    ON clients (backchannel_logout_uri);