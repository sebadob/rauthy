use crate::database::DB;
use crate::entity::pam::groups::PamGroup;
use hiqlite::StmtIndex;
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
    pub async fn insert(username: String, email: String) -> Result<(), ErrorResponse> {
        let group = PamGroup::insert(username, true).await?;

        let sql_user = r#"
INSERT INTO pam_users (name, gid, email, shell)
VALUES ($1, $2, $3, '/bin/bash')
RETURNING id
"#;
        let sql_rel = "INSERT INTO pam_groups_users (gid, uid) VALUES ($1, $2)";

        if is_hiqlite() {
            DB::hql()
                .txn([
                    (sql_user, params!(group.name, group.id, email)),
                    (sql_rel, params!(group.id, StmtIndex(0).column("id"))),
                ])
                .await?;
        } else {
            // TODO we need a custom query for postgres for the relation here
            todo!()
            // let mut cl = DB::pg().await?;
            // let txn = cl.transaction().await?;
            //
            // DB::pg_txn_append(&txn, sql_user, &[&group.name, &group.id, &email]).await?;
            // DB::pg_txn_append(&txn, sql_rel, &[&group.id, &group.id, &email]).await?;
            //
            // txn.commit().await?;
        }

        Ok(())
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

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM pam_users";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 8).await?
        };

        Ok(res)
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
