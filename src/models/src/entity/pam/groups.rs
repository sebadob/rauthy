use crate::database::DB;
use hiqlite_macros::params;
use rauthy_api_types::pam::PamGroupResponse;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PamGroup {
    pub id: u32,
    pub name: String,
    pub is_user_group: bool,
}

impl From<hiqlite::Row<'_>> for PamGroup {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            id: row.get::<i64>("id") as u32,
            name: row.get("name"),
            is_user_group: row.get("is_user_group"),
        }
    }
}

impl From<tokio_postgres::Row> for PamGroup {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get::<_, i64>("id") as u32,
            name: row.get("name"),
            is_user_group: row.get("is_user_group"),
        }
    }
}

impl PamGroup {
    pub async fn insert(name: String, is_user_group: bool) -> Result<Self, ErrorResponse> {
        let sql = r#"
INSERT INTO pam_groups (name, is_user_group)
VALUES ($1, $2)
ON CONFLICT (name) DO NOTHING
RETURNING *
"#;

        let slf: Self = if is_hiqlite() {
            DB::hql()
                .execute_returning_map_one(sql, params!(name, is_user_group))
                .await?
        } else {
            DB::pg_query_one(sql, &[&name, &is_user_group]).await?
        };

        Ok(slf)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM pam_groups";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 8).await?
        };

        Ok(res)
    }

    pub async fn find_by_id(id: u32) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM pam_groups WHERE id = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(slf)
    }

    pub async fn find_by_name(name: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM pam_groups WHERE name = $1";

        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(name)).await?
        } else {
            DB::pg_query_one(sql, &[&name]).await?
        };

        Ok(slf)
    }

    pub async fn delete(id: u32) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM pam_groups WHERE id = $1";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(id)).await?
        } else {
            DB::pg_execute(sql, &[&id]).await?
        };

        Ok(())
    }
}

impl PamGroup {
    pub async fn build_response(self) -> Result<PamGroupResponse, ErrorResponse> {
        let sql = "SELECT uid FROM pam_groups_users WHERE gid = $1";

        let members = if is_hiqlite() {
            DB::hql()
                .query_raw(sql, params!(self.id))
                .await?
                .into_iter()
                .map(|mut r| r.get("uid"))
                .collect::<Vec<String>>()
        } else {
            DB::pg_query_rows(sql, &[&self.id], 4)
                .await?
                .into_iter()
                .map(|r| r.get("uid"))
                .collect::<Vec<String>>()
        };

        Ok(PamGroupResponse {
            id: self.id,
            name: self.name,
            members,
        })
    }
}
