use crate::database::DB;
use crate::entity::pam::groups::PamGroup;
use crate::entity::pam::hosts::PamHost;
use crate::entity::pam::users::PamUser;
use crate::entity::users::User;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;

pub async fn apply_temp_migrations() -> Result<(), ErrorResponse> {
    // TODO these are only temp until we know what we actually need
    //  we don't care about postgres at this point
    //  we also clean up with each restart to not get annoyed with migrations,
    //  because we need to deploy on an actual server for proper testing
    if is_hiqlite() {
        let client = DB::hql();

        // TODO remember to make the IDs as BIGINTs for postgres later on

        let sql = r#"
CREATE TABLE pam_groups
(
    id   INTEGER NOT NULL
        CONSTRAINT pam_groups_pk
            PRIMARY KEY AUTOINCREMENT,
    name TEXT    NOT NULL,
    is_user_group INTEGER NOT NULL
) STRICT;

CREATE UNIQUE INDEX pam_groups_name_uindex
    ON pam_groups (name);

CREATE TABLE pam_hosts
(
    id        TEXT    NOT NULL
        CONSTRAINT pam_machines_pk
            PRIMARY KEY,
    secret    BLOB    NOT NULL,
    gid      INTEGER NOT NULL
        CONSTRAINT pam_hosts_pam_groups_id_fk
            REFERENCES pam_groups,
    force_mfa INTEGER NOT NULL,
    ip        TEXT,
    name      TEXT,
    dns       TEXT,
    notes     TEXT,
    groups_prefix TEXT
) STRICT;

CREATE INDEX pam_hosts_gid_index
    ON pam_hosts (gid);

CREATE TABLE pam_users
(
    id       INTEGER NOT NULL
        CONSTRAINT pam_users_pk
            PRIMARY KEY AUTOINCREMENT,
    name TEXT    NOT NULL,
    gid      INTEGER NOT NULL
        CONSTRAINT pam_users_pam_groups_id_fk
            REFERENCES pam_groups,
    email TEXT    not null
        CONSTRAINT pam_users_users_email_fk
            REFERENCES users (email)
            ON UPDATE CASCADE ON DELETE CASCADE,
    shell    TEXT    NOT NULL
) STRICT;

CREATE INDEX pam_users_gid_index
    ON pam_users (gid);

CREATE UNIQUE INDEX pam_users_name_uindex
    ON pam_users (name);

CREATE UNIQUE INDEX pam_users_email_uindex
    ON pam_users (email);

CREATE TABLE pam_groups_users
(
    gid INTEGER NOT NULL
        CONSTRAINT pam_groups_users_pam_groups_id_fk
            REFERENCES pam_groups
            ON DELETE CASCADE,
    uid INTEGER NOT NULL
        CONSTRAINT pam_groups_users_pam_users_id_fk
            REFERENCES pam_users
            ON DELETE CASCADE,
    CONSTRAINT pam_groups_users_pk
        PRIMARY KEY (gid, uid)
) STRICT;

-- make sure uid + gid sequences start at 100_000
INSERT INTO sqlite_sequence (name, seq)
VALUES  ('pam_groups', 100000),
        ('pam_users', 100000);
"#;

        client
            .txn([
                ("DROP TABLE IF EXISTS pam_hosts", params!()),
                ("DROP TABLE IF EXISTS pam_groups_users", params!()),
                ("DROP TABLE IF EXISTS pam_users", params!()),
                ("DROP TABLE IF EXISTS pam_groups", params!()),
                (
                    "DELETE FROM sqlite_sequence WHERE name IN ('pam_groups', 'pam_users')",
                    params!(),
                ),
            ])
            .await?;

        client.batch(sql).await?;

        let group = PamGroup::insert("test_group".to_string(), false).await?;
        PamHost::insert("test".to_string(), group.id).await?;

        if let Ok(user) = User::find_by_email("".to_string()).await {
            PamUser::insert("sebadob".to_string(), user.email).await?;
        } else if let Ok(user) = User::find_by_email("admin@localhost".to_string()).await {
            PamUser::insert("sebadob".to_string(), user.email).await?;
        }
    }

    Ok(())
}
