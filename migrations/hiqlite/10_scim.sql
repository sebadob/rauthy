CREATE TABLE failed_scim_tasks
(
    action      TEXT    NOT NULL,
    client_id   TEXT    NOT NULL
        CONSTRAINT failed_scim_tasks_clients_id_fk
            REFERENCES clients
            ON DELETE CASCADE,
    resource    TEXT    NOT NULL,
    retry_count INTEGER NOT NULL,
    CONSTRAINT failed_scim_tasks_pk
        PRIMARY KEY (action, client_id, resource)
) STRICT;

CREATE INDEX failed_scim_tasks_client_id_index
    ON failed_scim_tasks (client_id);
