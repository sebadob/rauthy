use crate::database::DB;
use crate::email;
use crate::entity::user_revoke::UserRevoke;
use crate::entity::users::User;
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use std::net::IpAddr;
use tokio::task;
use tracing::{debug, error};

#[derive(Debug)]
pub struct LoginLocation {
    pub user_id: String,
    pub ip: String,
    pub last_seen: i64,
    pub user_agent: String,
    pub location: Option<String>,
}

impl From<hiqlite::Row<'_>> for LoginLocation {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            user_id: row.get("user_id"),
            ip: row.get("ip"),
            last_seen: row.get("last_seen"),
            user_agent: row.get("user_agent"),
            location: row.get("location"),
        }
    }
}

impl From<tokio_postgres::Row> for LoginLocation {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            user_id: row.get("user_id"),
            ip: row.get("ip"),
            last_seen: row.get("last_seen"),
            user_agent: row.get("user_agent"),
            location: row.get("location"),
        }
    }
}

impl LoginLocation {
    pub async fn insert(
        user_id: String,
        ip: IpAddr,
        user_agent: String,
        location: Option<String>,
    ) -> Result<Self, ErrorResponse> {
        let now = Utc::now().timestamp();
        let ip = ip.to_string();

        let sql = r#"
INSERT INTO login_locations (user_id, ip, last_seen, user_agent, location)
VALUES ($1, $2, $3, $4, $5)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(&user_id, &ip, now, &user_agent, &location))
                .await?;
        } else {
            DB::pg_execute(sql, &[&user_id, &ip, &now, &user_agent, &location]).await?;
        }

        Ok(Self {
            user_id,
            ip,
            last_seen: now,
            user_agent,
            location,
        })
    }

    pub async fn find(user_id: String, ip: IpAddr) -> Result<Option<Self>, ErrorResponse> {
        let ip = ip.to_string();
        let sql = "SELECT * FROM login_locations WHERE user_id = $1 AND ip = $2";

        let slf = if is_hiqlite() {
            DB::hql()
                .query_map_optional(sql, params!(user_id, ip))
                .await?
        } else {
            DB::pg_query_opt(sql, &[&user_id, &ip]).await?
        };

        Ok(slf)
    }
}

impl LoginLocation {
    pub fn spawn_background_check(
        user_id: String,
        ip: IpAddr,
        user_agent: String,
        location: Option<String>,
    ) {
        task::spawn(async move {
            if let Err(err) = Self::background_check(user_id, ip, user_agent, location).await {
                error!(
                    "Error during LoginLocation::background_check(): {:?}",
                    err.message
                );
            }
        });
    }

    async fn background_check(
        user_id: String,
        ip: IpAddr,
        user_agent: String,
        location: Option<String>,
    ) -> Result<(), ErrorResponse> {
        if Self::find(user_id.clone(), ip.clone()).await?.is_some() {
            debug!("Login from IP {} for user {} is known", ip, user_id);
            return Ok(());
        }

        let slf = Self::insert(user_id, ip, user_agent, location).await?;
        let revoke = UserRevoke::find_or_upsert(slf.user_id.clone()).await?;
        let user = User::find(slf.user_id).await?;
        email::send_login_location(&user, slf.ip, slf.user_agent, slf.location, revoke.code).await;

        Ok(())
    }
}
