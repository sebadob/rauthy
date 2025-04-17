CREATE TABLE failed_scim_tasks
(
    client_id   VARCHAR NOT NULL
        CONSTRAINT failed_scim_tasks_clients_id_fk
            REFERENCES clients
            ON DELETE CASCADE,
    action      VARCHAR NOT NULL,
    retry_count INTEGER NOT NULL,
    CONSTRAINT failed_scim_tasks_pk
        PRIMARY KEY (client_id, action)
);


CREATE TABLE clients_scim
(
    client_id         VARCHAR NOT NULL
        CONSTRAINT clients_scim_pk
            PRIMARY KEY
        CONSTRAINT clients_scim_clients_id_fk
            REFERENCES clients
            ON DELETE CASCADE ON UPDATE CASCADE,
    bearer_token      BYTEA   NOT NULL,
    base_endpoint     VARCHAR NOT NULL,
    sync_groups       BOOLEAN NOT NULL,
    group_sync_prefix VARCHAR
);

CREATE UNIQUE INDEX clients_scim_client_id_uindex
    ON clients_scim (client_id);
