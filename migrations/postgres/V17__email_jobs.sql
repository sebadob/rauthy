CREATE TABLE email_jobs
(
    id           BIGINT   NOT NULL
        CONSTRAINT email_jobs_pk
            PRIMARY KEY,
    scheduled    BIGINT,
    status       SMALLINT NOT NULL,
    updated      BIGINT   NOT NULL,
    last_user_ts VARCHAR,
    filter       VARCHAR,
    content_type VARCHAR  NOT NULL,
    subject      VARCHAR  NOT NULL,
    body         VARCHAR  NOT NULL
);

CREATE INDEX email_jobs_scheduled_index
    ON email_jobs (scheduled);

CREATE INDEX email_jobs_status_index
    ON email_jobs (status);

CREATE INDEX email_jobs_updated_index
    ON email_jobs (updated);
