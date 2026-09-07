CREATE TABLE pwd_exp_mails
(
    user_id      TEXT    NOT NULL
        CONSTRAINT pwd_exp_mails_pk
            PRIMARY KEY
        CONSTRAINT pwd_exp_mails_users_id_fk
            REFERENCES users
            ON DELETE CASCADE,
    mail_sent_ts INTEGER NOT NULL
) STRICT;
