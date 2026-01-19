use crate::database::DB;
use crate::email::login_location;
use crate::entity::browser_id::BrowserId;
use crate::entity::user_revoke::UserRevoke;
use crate::entity::users::User;
use crate::events::event::Event;
use crate::ipgeo::get_location;
use actix_web::HttpRequest;
use actix_web::http::header::USER_AGENT;
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::real_ip_from_req;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::net::IpAddr;
use tokio::task;
use tracing::{debug, error, info};

#[derive(Debug)]
pub struct LoginLocation {
    pub user_id: String,
    pub ip: String,
    pub last_seen: i64,
    pub user_agent: String,
    pub location: Option<String>,
    pub browser_id: Option<String>,
}

impl From<hiqlite::Row<'_>> for LoginLocation {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        Self {
            user_id: row.get("user_id"),
            ip: row.get("ip"),
            last_seen: row.get("last_seen"),
            user_agent: row.get("user_agent"),
            location: row.get("location"),
            browser_id: row.get("browser_id"),
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
            browser_id: row.get("browser_id"),
        }
    }
}

impl LoginLocation {
    pub async fn insert(
        user_id: String,
        ip: IpAddr,
        user_agent: String,
        location: Option<String>,
        browser_id: BrowserId,
    ) -> Result<Self, ErrorResponse> {
        let now = Utc::now().timestamp();
        let ip = ip.to_string();
        let browser_id = browser_id.inner();

        let sql = r#"
INSERT INTO login_locations (user_id, ip, last_seen, user_agent, location, browser_id)
VALUES ($1, $2, $3, $4, $5, $6)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(&user_id, &ip, now, &user_agent, &location, &browser_id),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[&user_id, &ip, &now, &user_agent, &location, &browser_id],
            )
            .await?;
        }

        Ok(Self {
            user_id,
            ip,
            last_seen: now,
            user_agent,
            location,
            browser_id,
        })
    }

    pub async fn delete_for_user(user_id: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM login_locations WHERE user_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(user_id)).await?;
        } else {
            DB::pg_execute(sql, &[&user_id]).await?;
        }
        Ok(())
    }

    pub async fn find_by_browser_id(
        user_id: String,
        browser_id: String,
    ) -> Result<Option<Self>, ErrorResponse> {
        let sql = "SELECT * FROM login_locations WHERE user_id = $1 AND browser_id = $2";

        let slf = if is_hiqlite() {
            DB::hql()
                .query_map_optional(sql, params!(user_id, browser_id))
                .await?
        } else {
            DB::pg_query_opt(sql, &[&user_id, &browser_id]).await?
        };

        Ok(slf)
    }

    pub async fn find_by_ip(user_id: String, ip: IpAddr) -> Result<Option<Self>, ErrorResponse> {
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

    pub async fn update_by_browser_id(
        self,
        ip_new: IpAddr,
        location: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let ip = ip_new.to_string();

        let sql = r#"
UPDATE login_locations
SET last_seen = $1, ip = $2, location = $3
WHERE user_id = $4 AND browser_id = $5"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(now, ip, location, self.user_id, self.browser_id),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[&now, &ip, &location, &self.user_id, &self.browser_id],
            )
            .await?;
        }

        Ok(())
    }

    pub async fn update_last_seen(self) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let sql = r#"
UPDATE login_locations
SET last_seen = $1
WHERE user_id = $2 AND ip = $3"#;

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(now, self.user_id, self.ip))
                .await?;
        } else {
            DB::pg_execute(sql, &[&now, &self.user_id, &self.ip]).await?;
        }

        Ok(())
    }
}

impl LoginLocation {
    pub fn spawn_background_check(
        user: User,
        req: &HttpRequest,
        browser_id: BrowserId,
    ) -> Result<(), ErrorResponse> {
        let ip = real_ip_from_req(req)?;

        let user_agent = match req.headers().get(USER_AGENT) {
            None => {
                // to make our life a bit easier, we allow empty UAs during local dev
                // and only reject in PROD to block some bots out of the box
                #[cfg(not(debug_assertions))]
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Empty User-Agent not allowed",
                ));
                #[cfg(debug_assertions)]
                "No User-Agent - not allowed in production!".to_string()
            }
            Some(v) => v.to_str().unwrap_or_default().to_string(),
        };
        if user_agent.is_empty() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid User-Agent",
            ));
        }

        let location = get_location(req, ip)?;

        task::spawn(async move {
            if let Err(err) =
                Self::background_check(user, ip, user_agent, location, browser_id).await
            {
                error!(
                    ?err.message,
                    "Error during LoginLocation::background_check()",
                );
            }
        });

        Ok(())
    }

    async fn background_check(
        user: User,
        ip: IpAddr,
        user_agent: String,
        location: Option<String>,
        browser_id: BrowserId,
    ) -> Result<(), ErrorResponse> {
        if let Some(bid) = browser_id.as_str() {
            if let Some(slf) = Self::find_by_browser_id(user.id.clone(), bid.to_string()).await? {
                debug!(
                    "Login from Browser {browser_id} / {ip} for user {} is known",
                    user.id
                );
                slf.update_by_browser_id(ip, location).await?;
                return Ok(());
            }
        } else if let Some(slf) = Self::find_by_ip(user.id.clone(), ip).await? {
            debug!("Login from IP {ip} for user {} is known", user.id);
            slf.update_last_seen().await?;
            return Ok(());
        }

        info!(
            "Login from new IP {ip} ({location:?} / {user_agent}) for user {}",
            user.email
        );

        let slf = Self::insert(user.id.clone(), ip, user_agent, location, browser_id).await?;
        if let Err(err) = Event::new_login_location(&user, &slf).send().await {
            error!(?err, "Error generating event for NewLoginLocation");
        }

        let revoke = UserRevoke::find_or_upsert(user.id.clone()).await?;
        login_location::send_login_location(
            &user,
            slf.ip,
            slf.user_agent,
            slf.location,
            revoke.code,
        )
        .await;

        Ok(())
    }
}
