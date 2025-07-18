use crate::database::DB;
use hiqlite_macros::params;
use rauthy_api_types::pam::{PamGroupMembersResponse, PamGroupResponse};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PamGroupType {
    Immutable,
    Host,
    User,
    Generic,
    Local,
}

impl From<&str> for PamGroupType {
    fn from(value: &str) -> Self {
        match value {
            "immutable" => Self::Immutable,
            "host" => Self::Host,
            "user" => Self::User,
            "local" => Self::Local,
            _ => Self::Generic,
        }
    }
}

impl PamGroupType {
    fn as_str(&self) -> &str {
        match self {
            Self::Immutable => "immutable",
            Self::Host => "host",
            Self::User => "user",
            Self::Generic => "generic",
            Self::Local => "local",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PamGroup {
    pub id: u32,
    pub name: String,
    pub typ: PamGroupType,
}

impl From<hiqlite::Row<'_>> for PamGroup {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            id: row.get::<i64>("id") as u32,
            name: row.get("name"),
            typ: PamGroupType::from(row.get::<String>("typ").as_str()),
        }
    }
}

impl From<tokio_postgres::Row> for PamGroup {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get::<_, i64>("id") as u32,
            name: row.get("name"),
            typ: PamGroupType::from(row.get::<_, String>("typ").as_str()),
        }
    }
}

impl PamGroup {
    pub async fn insert(name: String, typ: PamGroupType) -> Result<Self, ErrorResponse> {
        let sql = r#"
INSERT INTO pam_groups (name, typ)
VALUES ($1, $2)
ON CONFLICT (name) DO NOTHING
RETURNING *
"#;

        let slf: Self = if is_hiqlite() {
            DB::hql()
                .execute_returning_map_one(sql, params!(name, typ.as_str()))
                .await?
        } else {
            DB::pg_query_one(sql, &[&name, &typ.as_str()]).await?
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
    pub async fn build_members_response(self) -> Result<PamGroupMembersResponse, ErrorResponse> {
        let sql = r#"
SELECT name FROM pam_rel_groups_users pu
JOIN pam_users u on pu.uid = u.id
WHERE pu.gid = $1
"#;

        let members = if is_hiqlite() {
            DB::hql()
                .query_raw(sql, params!(self.id))
                .await?
                .into_iter()
                .map(|mut r| r.get("name"))
                .collect::<Vec<String>>()
        } else {
            DB::pg_query_rows(sql, &[&self.id], 4)
                .await?
                .into_iter()
                .map(|r| r.get("name"))
                .collect::<Vec<String>>()
        };

        Ok(PamGroupMembersResponse {
            id: self.id,
            name: self.name,
            typ: self.typ.into(),
            members,
        })
    }
}

impl From<PamGroup> for PamGroupResponse {
    fn from(g: PamGroup) -> Self {
        Self {
            id: g.id,
            name: g.name,
            typ: g.typ.into(),
        }
    }
}

impl From<PamGroupType> for rauthy_api_types::pam::PamGroupType {
    fn from(value: PamGroupType) -> Self {
        match value {
            PamGroupType::Immutable => Self::Immutable,
            PamGroupType::Host => Self::Host,
            PamGroupType::User => Self::User,
            PamGroupType::Generic => Self::Generic,
            PamGroupType::Local => Self::Local,
        }
    }
}
