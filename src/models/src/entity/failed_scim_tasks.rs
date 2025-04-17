use crate::database::DB;
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ScimAction {
    // user_id
    UserCreateUpdate(String),
    UserDelete(String),
    // created ts of the last successful user for bigger syncs
    UsersSync(i64),
    // group_id
    GroupCreateUpdate(String),
    GroupDelete(String),
    GroupsSync,
    Unknown,
}

impl From<&str> for ScimAction {
    fn from(s: &str) -> Self {
        if s.starts_with("uc_") {
            let (_, uid) = s.split_once('_').unwrap();
            Self::UserCreateUpdate(uid.to_string())
        } else if s.starts_with("ud_") {
            let (_, uid) = s.split_once('_').unwrap();
            Self::UserDelete(uid.to_string())
        } else if s.starts_with("us_") {
            let (_, ts_str) = s.split_once('_').unwrap();
            let ts = ts_str.parse::<i64>().unwrap_or_default();
            Self::UsersSync(ts)
        } else if s.starts_with("gc_") {
            let (_, gid) = s.split_once('_').unwrap();
            Self::GroupCreateUpdate(gid.to_string())
        } else if s.starts_with("gd_") {
            let (_, gid) = s.split_once('_').unwrap();
            Self::GroupDelete(gid.to_string())
        } else if s.starts_with("gs_") {
            Self::GroupsSync
        } else {
            Self::Unknown
        }
    }
}

impl Display for ScimAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScimAction::UserCreateUpdate(uid) => write!(f, "uc_{}", uid),
            ScimAction::UserDelete(uid) => write!(f, "ud_{}", uid),
            ScimAction::UsersSync(created_ts) => write!(f, "us_{}", created_ts),
            ScimAction::GroupCreateUpdate(gid) => write!(f, "gc_{}", gid),
            ScimAction::GroupDelete(gid) => write!(f, "gd_{}", gid),
            ScimAction::GroupsSync => write!(f, "gs_"),
            ScimAction::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug)]
pub struct FailedScimTask {
    // pub action: ScimAction,
    pub client_id: String,
    pub action: ScimAction,
    pub retry_count: i64,
}

impl<'r> From<hiqlite::Row<'r>> for FailedScimTask {
    fn from(mut row: hiqlite::Row<'r>) -> Self {
        Self {
            action: ScimAction::from(row.get::<String>("action").as_str()),
            client_id: row.get("client_id"),
            retry_count: row.get::<i64>("retry_count"),
        }
    }
}

impl From<tokio_postgres::Row> for FailedScimTask {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            action: ScimAction::from(row.get::<_, String>("action").as_str()),
            client_id: row.get("client_id"),
            retry_count: row.get::<_, i32>("retry_count") as i64,
        }
    }
}

impl FailedScimTask {
    pub async fn upsert(action: &ScimAction, client_id: &str) -> Result<(), ErrorResponse> {
        let sql = r#"
INSERT INTO failed_scim_tasks (client_id, action, retry_count)
VALUES ($1, $2, 0)
ON CONFLICT (client_id, action)
DO UPDATE SET retry_count = retry_count + 1"#;

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(client_id, action.to_string()))
                .await?;
        } else {
            DB::pg_execute(sql, &[&client_id, &action.to_string()]).await?;
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
            "DELETE FROM failed_scim_tasks WHERE client_id = $1 AND action = $2 AND resource = $3";
        let action = self.action.to_string();

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(&self.client_id, action))
                .await?;
        } else {
            DB::pg_execute(sql, &[&self.client_id, &action]).await?;
        }

        Ok(())
    }
}
