CREATE TABLE email_jobs
(
    id           INTEGER NOT NULL
        CONSTRAINT email_jobs_pk
            PRIMARY KEY,
    scheduled    INTEGER,
    status       INTEGER NOT NULL,
    updated      INTEGER NOT NULL,
    last_user_ts TEXT,
    filter       TEXT,
    content_type TEXT    NOT NULL,
    subject      TEXT    NOT NULL,
    body         TEXT    NOT NULL
) STRICT;

CREATE INDEX email_jobs_scheduled_index
    ON email_jobs (scheduled);

CREATE INDEX email_jobs_status_index
    ON email_jobs (status);

CREATE INDEX email_jobs_updated_index
    ON email_jobs (updated);
