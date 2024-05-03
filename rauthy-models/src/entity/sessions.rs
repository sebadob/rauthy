use crate::app_state::AppState;
use crate::entity::continuation_token::ContinuationToken;
use crate::entity::users::User;
use actix_web::cookie::{time, Cookie, SameSite};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{cookie, web, HttpRequest};
use rauthy_common::constants::{
    CACHE_NAME_12HR, CACHE_NAME_SESSIONS, COOKIE_SESSION, CSRF_HEADER, DANGER_COOKIE_INSECURE,
    IDX_SESSION,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::get_rand;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, cache_remove, AckLevel};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};
use std::ops::Add;
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::{error, warn};
use utoipa::ToSchema;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub csrf_token: String,
    pub user_id: Option<String>,
    pub roles: Option<String>,
    pub groups: Option<String>,
    pub is_mfa: bool,
    #[sqlx(flatten)]
    pub state: SessionState,
    pub exp: i64,
    pub last_seen: i64,
    pub remote_ip: Option<String>, // TODO should we maybe force a linked remote_ip all the time?
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum SessionState {
    Open,
    Init,
    Auth,
    LoggedOut,
    Unknown,
}

impl From<String> for SessionState {
    fn from(value: String) -> Self {
        Self::from_str(value.as_str()).unwrap()
    }
}

impl FromRow<'_, SqliteRow> for SessionState {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::error::Error> {
        let s = row.try_get("state").unwrap();
        Ok(Self::from_str(s).expect("Corrupted 'state' in 'sessions'"))
    }
}

impl FromRow<'_, PgRow> for SessionState {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::error::Error> {
        let s = row.try_get("state").unwrap();
        Ok(Self::from_str(s).expect("Corrupted 'state' in 'sessions'"))
    }
}

impl FromStr for SessionState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "open" => SessionState::Open,
            "init" => SessionState::Init,
            "auth" => SessionState::Auth,
            "logged_out" => SessionState::LoggedOut,
            _ => SessionState::Unknown,
        };
        Ok(res)
    }
}

impl SessionState {
    pub fn as_str(&self) -> &str {
        match self {
            SessionState::Open => "open",
            SessionState::Init => "init",
            SessionState::Auth => "auth",
            SessionState::LoggedOut => "logged_out",
            SessionState::Unknown => "unknown",
        }
    }
}

// CRUD
impl Session {
    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        sqlx::query!("DELETE FROM sessions WHERE id = $1", self.id)
            .execute(&data.db)
            .await?;

        cache_remove(
            CACHE_NAME_SESSIONS.to_string(),
            Session::cache_idx(&self.id),
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;

        Ok(())
    }

    pub async fn delete_by_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<(), ErrorResponse> {
        let sessions: Vec<Self> =
            sqlx::query_as!(Self, "SELECT * FROM sessions WHERE user_id = $1", user_id)
                .fetch_all(&data.db)
                .await?;

        // we do not use any fancy 'returning' statement here for compatibility with sqlite
        sqlx::query!("DELETE FROM sessions WHERE user_id = $1", user_id)
            .execute(&data.db)
            .await?;

        for s in sessions {
            cache_remove(
                CACHE_NAME_SESSIONS.to_string(),
                Session::cache_idx(&s.id),
                &data.caches.ha_cache_config,
                AckLevel::Quorum,
            )
            .await?;
        }

        Ok(())
    }

    // Returns a session by id
    pub async fn find(data: &web::Data<AppState>, id: String) -> Result<Self, ErrorResponse> {
        let idx = Session::cache_idx(&id);
        let session = cache_get!(
            Session,
            CACHE_NAME_SESSIONS.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if let Some(session) = session {
            return Ok(session);
        }

        let session = sqlx::query_as!(Self, "select * from sessions where id = $1", id)
            .fetch_one(&data.db)
            .await?;

        cache_insert(
            CACHE_NAME_SESSIONS.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &session,
            AckLevel::Leader,
        )
        .await?;

        Ok(session)
    }

    // not cached, since this is only used in the admin ui
    /// Returns all sessions and an empty Vec if not a single session exists
    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let sessions = sqlx::query_as!(Self, "select * from sessions")
            .fetch_all(&data.db)
            .await?;
        Ok(sessions)
    }

    pub async fn find_paginated(
        data: &web::Data<AppState>,
        continuation_token: Option<ContinuationToken>,
        page_size: i64,
        mut offset: i64,
        backwards: bool,
    ) -> Result<(Vec<Self>, Option<ContinuationToken>), ErrorResponse> {
        // Allowing this unused assignment here makes to type conflicts from sqlx later easier
        // to handle.
        #[allow(unused_assignments)]
        let mut res = None;
        let mut latest_ts = 0;

        if let Some(token) = continuation_token {
            if backwards {
                // when we go backwards, we must skip the current page
                // the continuation token will always point at the last entry
                // of the current page
                // -> add to the offset
                offset += page_size;

                let mut rows: Vec<Self> = sqlx::query_as!(
                    Self,
                    r#"SELECT *
                    FROM sessions
                    WHERE exp <= $1
                    ORDER BY exp DESC
                    LIMIT $2
                    OFFSET $3"#,
                    token.ts,
                    page_size,
                    offset,
                )
                .fetch_all(&data.db)
                .await?;

                rows.reverse();
                if let Some(last) = rows.last() {
                    latest_ts = last.exp;
                }
                res = Some(rows);
            } else {
                let rows = sqlx::query_as!(
                    Self,
                    r#"SELECT *
                    FROM sessions
                    WHERE exp >= $1 AND id != $2
                    ORDER BY exp ASC
                    LIMIT $3
                    OFFSET $4"#,
                    token.ts,
                    token.id,
                    page_size,
                    offset,
                )
                .fetch_all(&data.db)
                .await?;

                if let Some(last) = rows.last() {
                    latest_ts = last.exp;
                }
                res = Some(rows);
            };
        } else if backwards {
            // backwards without any continuation token will simply
            // serve the last elements without any other conditions
            let mut rows = sqlx::query_as!(
                Self,
                r#"SELECT *
                   FROM sessions
                   ORDER BY exp DESC
                   LIMIT $1
                   OFFSET $2"#,
                page_size,
                offset,
            )
            .fetch_all(&data.db)
            .await?;

            rows.reverse();
            if let Some(last) = rows.last() {
                latest_ts = last.exp;
            }
            res = Some(rows);
        } else {
            let rows = sqlx::query_as!(
                Self,
                r#"SELECT *
                   FROM sessions
                   ORDER BY exp ASC
                   LIMIT $1
                   OFFSET $2"#,
                page_size,
                offset,
            )
            .fetch_all(&data.db)
            .await?;

            if let Some(last) = rows.last() {
                latest_ts = last.exp;
            }
            res = Some(rows);
        };

        let res = res
            .expect("sessions paginated should always be at least an empty Vec<_> at this point");
        let token = res
            .last()
            .map(|entry| ContinuationToken::new(entry.id.clone(), latest_ts));

        Ok((res, token))
    }

    /// Invalidates all sessions by setting the expiry to `now()`
    pub async fn invalidate_all(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let sessions = Session::find_all(data).await?;
        let mut removed = Vec::default();

        for mut s in sessions {
            if s.exp > now {
                s.exp = now;
                if let Err(err) = s.save(data).await {
                    error!("Error invalidating session: {}", err);
                }
                removed.push(s.id);
            }
        }

        for id in removed {
            cache_remove(
                CACHE_NAME_SESSIONS.to_string(),
                Session::cache_idx(&id),
                &data.caches.ha_cache_config,
                AckLevel::Quorum,
            )
            .await?;
        }

        Ok(())
    }

    /// Invalidates all sessions for the given user_id by setting the expiry to `now()`
    pub async fn invalidate_for_user(
        data: &web::Data<AppState>,
        uid: &str,
    ) -> Result<(), ErrorResponse> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let sessions = Session::find_all(data).await?;
        let mut removed = Vec::default();

        for mut s in sessions {
            if s.user_id.is_none() {
                continue;
            }

            if s.user_id.as_ref().unwrap().as_str() == uid {
                s.exp = now;
                s.save(data).await?;
                removed.push(s.id);
            }
        }

        for id in removed {
            cache_remove(
                CACHE_NAME_SESSIONS.to_string(),
                Session::cache_idx(&id),
                &data.caches.ha_cache_config,
                AckLevel::Quorum,
            )
            .await?;
        }

        Ok(())
    }

    /// Saves a Session
    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let state_str = self.state.as_str();

        #[cfg(not(feature = "postgres"))]
        let q = sqlx::query!(
            r#"insert or replace into
            sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen, remote_ip)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
            self.id,
            self.csrf_token,
            self.user_id,
            self.roles,
            self.groups,
            self.is_mfa,
            state_str,
            self.exp,
            self.last_seen,
            self.remote_ip,
        );

        #[cfg(feature = "postgres")]
        let q = sqlx::query!(
            r#"insert into
            sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen, remote_ip)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            on conflict(id) do update set user_id = $3, roles = $4, groups = $5, is_mfa = $6,
            state = $7, exp = $8, last_seen = $9, remote_ip = $10"#,
            self.id,
            self.csrf_token,
            self.user_id,
            self.roles,
            self.groups,
            self.is_mfa,
            state_str,
            self.exp,
            self.last_seen,
            self.remote_ip,
        );

        q.execute(&data.db).await?;

        cache_insert(
            CACHE_NAME_SESSIONS.to_string(),
            Session::cache_idx(&self.id),
            &data.caches.ha_cache_config,
            &self,
            AckLevel::Quorum,
        )
        .await?;

        Ok(())
    }
}

impl Session {
    /// exp_in will be the time in seconds when the session will expire
    pub fn new(exp_in: u32, remote_ip: Option<String>) -> Self {
        let id = get_rand(32);
        let csrf_token = get_rand(32);
        let now = OffsetDateTime::now_utc();

        Self {
            id,
            csrf_token,
            user_id: None,
            roles: None,
            groups: None,
            is_mfa: false, // cannot be known at the creation stage
            state: SessionState::Init,
            exp: now
                .add(time::Duration::seconds(exp_in as i64))
                .unix_timestamp(),
            last_seen: now.unix_timestamp(),
            remote_ip,
        }
    }

    fn cache_idx(id: &String) -> String {
        format!("{}{}", IDX_SESSION, id)
    }

    /// exp_in will be the time in seconds when the session will expire
    pub fn try_new(
        user: &User,
        exp_in: u32,
        remote_ip: Option<String>,
    ) -> Result<Self, ErrorResponse> {
        let id = get_rand(32);
        let csrf_token = get_rand(32);
        let user_id = Some(user.id.clone());
        let roles = Some(user.roles.clone());
        let groups = user.groups.clone();

        let now = OffsetDateTime::now_utc();

        // make sure to check the max session lifetime for expiring users
        let exp = if let Some(ts) = user.user_expires {
            if now.unix_timestamp() > ts {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User has expired".to_string(),
                ));
            } else {
                let target = now
                    .add(time::Duration::seconds(exp_in as i64))
                    .unix_timestamp();
                if ts < target {
                    ts
                } else {
                    target
                }
            }
        } else {
            now.add(time::Duration::seconds(exp_in as i64))
                .unix_timestamp()
        };

        Ok(Self {
            id,
            csrf_token,
            user_id,
            roles,
            groups,
            is_mfa: false, // cannot be known at the creation stage
            state: SessionState::Init,
            exp,
            last_seen: now.unix_timestamp(),
            remote_ip,
        })
    }

    pub fn client_cookie(&self) -> cookie::Cookie {
        let exp = cookie::Expiration::from(
            OffsetDateTime::from_unix_timestamp(self.exp)
                .expect("Error with offset datetime calculation for client cookie"),
        );

        let secure = !*DANGER_COOKIE_INSECURE;
        if !secure {
            warn!("Building INSECURE session cookie - you MUST NEVER use this in production");
        }

        cookie::Cookie::build(COOKIE_SESSION, self.id.clone())
            .http_only(true)
            .secure(secure)
            .same_site(SameSite::Lax)
            .expires(exp)
            .path("/auth")
            .finish()
    }

    pub fn extract_from_req(
        session_req: web::ReqData<Option<Session>>,
    ) -> Result<Session, ErrorResponse> {
        if session_req.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("Session cookie is missing"),
            ));
        }
        Ok(session_req.into_inner().unwrap())
    }

    /// Extracts the session from an `Option<Session>` coming from an HttpRequest, validates the
    /// CSRF token from the Header and then returns the Session.
    pub fn extract_validate_csrf(
        session_req: web::ReqData<Option<Session>>,
        req: &HttpRequest,
    ) -> Result<Session, ErrorResponse> {
        let s = Session::extract_from_req(session_req)?;
        s.validate_csrf(req)?;
        Ok(s)
    }

    pub fn get_csrf_header(token: &str) -> (HeaderName, HeaderValue) {
        (
            HeaderName::from_str(CSRF_HEADER).unwrap(),
            HeaderValue::from_str(token).unwrap(),
        )
    }

    pub async fn invalidate(
        &mut self,
        data: &web::Data<AppState>,
    ) -> Result<Cookie, ErrorResponse> {
        let idx = Session::cache_idx(&self.id);

        self.exp = OffsetDateTime::now_utc().unix_timestamp();
        self.state = SessionState::LoggedOut;

        sqlx::query("update sessions set exp = $1, state = $2 where id = $3")
            .bind(self.exp)
            .bind(self.state.as_str())
            .bind(&self.id)
            .execute(&data.db)
            .await?;

        cache_remove(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;

        Ok(cookie::Cookie::build(COOKIE_SESSION, &self.id)
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .max_age(cookie::time::Duration::ZERO)
            .path("/auth")
            .finish())
    }

    /// Checks if the current session is valid: has not expired and has not timed out (last_seen)
    pub fn is_valid(&self, session_timeout: u32, remote_ip: Option<String>) -> bool {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        if self.exp < now {
            return false;
        }
        if self.last_seen < now - session_timeout as i64 {
            return false;
        }
        if self.state == SessionState::Unknown {
            return false;
        }
        if let Some(ip) = remote_ip {
            if (self.state == SessionState::Open || self.state == SessionState::Auth)
                && self.remote_ip.as_ref() != Some(&ip)
            {
                let session_ip = self.remote_ip.as_deref().unwrap_or("UNKNOWN");
                warn!(
                    "Invalid access for session {} / {} with different IP: {}",
                    self.id, session_ip, ip,
                );
                return false;
            }
        }
        true
    }

    pub fn groups_as_vec(&self) -> Result<Vec<&str>, ErrorResponse> {
        if self.groups.is_none() {
            return Ok(Vec::default());
        };
        let mut res = Vec::new();
        self.groups
            .as_ref()
            .unwrap()
            .split(',')
            .map(|g| g.trim())
            .for_each(|g| res.push(g));
        Ok(res)
    }

    pub fn roles_as_vec(&self) -> Result<Vec<String>, ErrorResponse> {
        if self.roles.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                format!("User '{}' is not assigned to any roles", &self.id),
            ));
        }
        Ok(self
            .roles
            .as_ref()
            .unwrap()
            .split(',')
            .map(|r| r.to_string())
            .collect())
    }

    #[inline]
    pub async fn set_mfa(
        &mut self,
        data: &web::Data<AppState>,
        value: bool,
    ) -> Result<(), ErrorResponse> {
        self.is_mfa = value;
        self.save(data).await
    }

    #[inline(always)]
    pub fn validate_csrf(&self, req: &HttpRequest) -> Result<(), ErrorResponse> {
        let csrf = get_header_value(req, CSRF_HEADER);
        if csrf.is_err() {
            return Err(ErrorResponse::new(
                ErrorResponseType::CSRFTokenError,
                String::from("CSRF Token not present in HTTP Header"),
            ));
        }
        if self.csrf_token.eq(csrf.unwrap()) {
            return Ok(());
        }
        Err(ErrorResponse::new(
            ErrorResponseType::CSRFTokenError,
            String::from("CSRF Token is not correct"),
        ))
    }

    pub fn validate_user_expiry(&mut self, user: &User) -> Result<(), ErrorResponse> {
        if let Some(ts) = user.user_expires {
            if OffsetDateTime::now_utc().unix_timestamp() > ts {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User has expired".to_string(),
                ));
            } else if ts < self.exp {
                self.exp = ts;
            }
        }

        Ok(())
    }
}

pub fn get_header_value<'a>(
    req: &'a HttpRequest,
    val: &'a str,
) -> Result<&'a HeaderValue, ErrorResponse> {
    let res = req.headers().get(val).ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Missing header value '{}'", val),
        )
    })?;
    Ok(res)
}
