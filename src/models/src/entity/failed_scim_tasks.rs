use crate::database::DB;
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ScimAction {
    Create,
    Update,
    Delete,
    Sync,
    Unknown,
}

impl From<&str> for ScimAction {
    fn from(value: &str) -> Self {
        match value {
            "create" => Self::Create,
            "update" => Self::Update,
            "delete" => Self::Delete,
            "sync" => Self::Sync,
            _ => Self::Unknown,
        }
    }
}

impl ScimAction {
    fn as_str(&self) -> &str {
        match self {
            Self::Create => "create",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Sync => "sync",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug)]
pub enum ScimResource {
    // user_id
    User(String),
    // created ts of the last successful user for bigger syncs
    Users(i64),
    // group_id
    Group(String),
    Groups,
    Unknown,
}

impl From<&str> for ScimResource {
    fn from(s: &str) -> Self {
        if s.starts_with("u_") {
            let (_, uid) = s.split_once('_').unwrap();
            Self::User(uid.to_string())
        } else if s.starts_with("us_") {
            let (_, ts_str) = s.split_once('_').unwrap();
            let ts = ts_str.parse::<i64>().unwrap_or_default();
            Self::Users(ts)
        } else if s.starts_with("g_") {
            let (_, gid) = s.split_once('_').unwrap();
            Self::Group(gid.to_string())
        } else if s.starts_with("gs_") {
            Self::Groups
        } else {
            Self::Unknown
        }
    }
}

impl Display for ScimResource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScimResource::User(uid) => write!(f, "u_{}", uid),
            ScimResource::Users(created_ts) => write!(f, "us_{}", created_ts),
            ScimResource::Group(gid) => write!(f, "g_{}", gid),
            ScimResource::Groups => write!(f, "gs_"),
            ScimResource::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug)]
pub struct FailedScimTask {
    pub action: ScimAction,
    pub client_id: String,
    pub resource: ScimResource,
    pub retry_count: i64,
}

impl<'r> From<hiqlite::Row<'r>> for FailedScimTask {
    fn from(mut row: hiqlite::Row<'r>) -> Self {
        Self {
            action: ScimAction::from(row.get::<String>("action").as_str()),
            resource: ScimResource::from(row.get::<String>("resource").as_str()),
            client_id: row.get("client_id"),
            retry_count: row.get::<i64>("retry_count"),
        }
    }
}

impl From<tokio_postgres::Row> for FailedScimTask {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            action: ScimAction::from(row.get::<_, String>("action").as_str()),
            resource: ScimResource::from(row.get::<_, String>("resource").as_str()),
            client_id: row.get("client_id"),
            retry_count: row.get::<_, i32>("retry_count") as i64,
        }
    }
}

impl FailedScimTask {
    pub async fn upsert(
        action: &ScimAction,
        resource: &ScimResource,
        client_id: &str,
    ) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO failed_scim_tasks (action, client_id, resource, retry_count)
VALUES ($1, $2, $3, 0)
ON CONFLICT (action, client_id, resource)
DO UPDATE SET retry_count = retry_count + 1"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(action.as_str(), client_id, resource.to_string()),
                )
                .await?;
        } else {
            DB::pg_execute(sql, &[&action.as_str(), &client_id, &resource.to_string()]).await?;
        }

        Ok(())
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM failed_scim_tasks";

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };

        Ok(res)
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        let sql =
            "DELETE FROM failed_scim_tasks WHERE action = $1 AND client_id = $2 AND resource = $3";
        let resource = self.resource.to_string();

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(self.action.as_str(), &self.client_id, resource),
                )
                .await?;
        } else {
            DB::pg_execute(sql, &[&self.action.as_str(), &self.client_id, &resource]).await?;
        }

        Ok(())
    }
}
