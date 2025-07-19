use crate::database::DB;
use crate::entity::pam::groups::{PamGroup, PamGroupType};
use hiqlite_macros::params;
use rauthy_api_types::pam::PamUserResponse;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PamUser {
    pub id: u32,
    pub name: String,
    pub gid: u32,
    pub email: String,
    pub shell: String,
}

impl From<hiqlite::Row<'_>> for PamUser {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            id: row.get::<i64>("id") as u32,
            name: row.get("name"),
            gid: row.get::<i64>("gid") as u32,
            email: row.get("email"),
            shell: row.get("shell"),
        }
    }
}

impl From<tokio_postgres::Row> for PamUser {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get::<_, i64>("id") as u32,
            name: row.get("name"),
            gid: row.get::<_, i64>("gid") as u32,
            email: row.get("email"),
            shell: row.get("shell"),
        }
    }
}

impl PamUser {
    pub async fn insert(username: String, email: String) -> Result<Self, ErrorResponse> {
        let group = PamGroup::insert(username, PamGroupType::User).await?;

        let sql_user = r#"
INSERT INTO pam_users (name, gid, email, shell)
VALUES ($1, $2, $3, '/bin/bash')
RETURNING *
"#;
        let sql_rel = "INSERT INTO pam_rel_groups_users (gid, uid) VALUES ($1, $2)";

        let slf = if is_hiqlite() {
            let slf: Self = DB::hql()
                .execute_returning_map_one(sql_user, params!(group.name, group.id, email))
                .await?;
            DB::hql()
                .execute(sql_rel, params!(group.id, slf.id))
                .await?;

            slf
        } else {
            let slf: Self = DB::pg_query_one(sql_user, &[&group.name, &group.id, &email]).await?;

            DB::pg_execute(sql_rel, &[&group.id, &slf.id]).await?;

            slf
        };

        Ok(slf)
    }

    pub async fn find_by_name(username: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM pam_users WHERE name = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(username)).await?
        } else {
            DB::pg_query_one(sql, &[&username]).await?
        };

        Ok(slf)
    }

    pub async fn find_by_id(id: u32) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM pam_users WHERE id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(slf)
    }

    pub async fn find_by_user_id(user_id: String) -> Result<Self, ErrorResponse> {
        let sql = r#"
SELECT * FROM pam_users WHERE email = (
    SELECT email FROM users WHERE id = $1
)
"#;

        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(user_id)).await?
        } else {
            DB::pg_query_one(sql, &[&user_id]).await?
        };

        Ok(slf)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM pam_users";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 8).await?
        };

        Ok(res)
    }

    pub async fn find_emails_unlinked() -> Result<Vec<String>, ErrorResponse> {
        let sql = r#"
SELECT u.email AS email FROM users u
LEFT JOIN pam_users pu
WHERE pu.id IS NULL
"#;

        let emails: Vec<String> = if is_hiqlite() {
            DB::hql()
                .query_raw(sql, params!())
                .await?
                .into_iter()
                .map(|mut r| r.get("email"))
                .collect::<Vec<_>>()
        } else {
            DB::pg_query_rows(sql, &[], 16)
                .await?
                .into_iter()
                .map(|r| r.get("email"))
                .collect::<Vec<_>>()
        };

        Ok(emails)
    }
}

impl From<PamUser> for PamUserResponse {
    fn from(u: PamUser) -> Self {
        Self {
            id: u.id,
            name: u.name,
            gid: u.gid,
            email: u.email,
            shell: u.shell,
        }
    }
}
