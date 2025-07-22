use crate::database::DB;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;

#[derive(Debug)]
pub struct PamGroupUserLink {
    pub uid: u32,
    pub gid: u32,
    pub wheel: bool,
}

impl From<hiqlite::Row<'_>> for PamGroupUserLink {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            uid: row.get::<i64>("uid") as u32,
            gid: row.get::<i64>("gid") as u32,
            wheel: row.get("wheel"),
        }
    }
}

impl From<tokio_postgres::Row> for PamGroupUserLink {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            uid: row.get::<_, i64>("uid") as u32,
            gid: row.get::<_, i64>("gid") as u32,
            wheel: row.get("wheel"),
        }
    }
}

impl PamGroupUserLink {
    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM pam_rel_groups_users";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 32).await?
        };

        Ok(res)
    }

    pub async fn find_for_group(gid: u32) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM pam_rel_groups_users WHERE gid = $1";

        let res: Vec<Self> = if is_hiqlite() {
            DB::hql().query_map(sql, params!(gid)).await?
        } else {
            DB::pg_query(sql, &[&gid], 4).await?
        };

        Ok(res)
    }

    pub async fn find_for_user(uid: u32) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM pam_rel_groups_users WHERE uid = $1";

        let res: Vec<Self> = if is_hiqlite() {
            DB::hql().query_map(sql, params!(uid)).await?
        } else {
            DB::pg_query(sql, &[&uid], 4).await?
        };

        Ok(res)
    }
}

impl From<PamGroupUserLink> for rauthy_api_types::pam::PamGroupUserLink {
    fn from(link: PamGroupUserLink) -> Self {
        Self {
            uid: link.uid,
            gid: link.gid,
            wheel: link.wheel,
        }
    }
}

impl From<rauthy_api_types::pam::PamGroupUserLink> for PamGroupUserLink {
    fn from(link: rauthy_api_types::pam::PamGroupUserLink) -> Self {
        Self {
            uid: link.uid,
            gid: link.gid,
            wheel: link.wheel,
        }
    }
}
