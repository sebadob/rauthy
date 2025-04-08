use crate::api_cookie::ApiCookie;
use crate::database::{Cache, DB};
use crate::entity::continuation_token::ContinuationToken;
use crate::entity::users::User;
use actix_web::cookie::{SameSite, time};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{HttpRequest, cookie, web};
use chrono::Utc;
use hiqlite::{Param, params};
use rauthy_api_types::generic::SearchParamsIdx;
use rauthy_common::constants::{
    CACHE_TTL_SESSION, COOKIE_SESSION, COOKIE_SESSION_FED_CM, CSRF_HEADER, SESSION_LIFETIME_FED_CM,
};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cmp::max;
use std::fmt::{Debug, Formatter};
use std::net::IpAddr;
use std::ops::Add;
use std::str::FromStr;
use tracing::{error, warn};

#[derive(Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub csrf_token: String,
    pub user_id: Option<String>,
    pub roles: Option<String>,
    pub groups: Option<String>,
    pub is_mfa: bool,
    pub state: String,
    pub exp: i64,
    pub last_seen: i64,
    pub remote_ip: Option<String>,
}

impl Debug for Session {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}(...), csrf_token: {}(...), user_id: {:?}, roles: {:?}, groups: {:?}, \
        is_mfa: {}, state: {}, exp: {}, last_seen: {}, remote_ip: {:?}",
            &self.id[..5],
            &self.csrf_token[..5],
            self.user_id,
            self.roles,
            self.groups,
            self.is_mfa,
            self.state,
            self.exp,
            self.last_seen,
            self.remote_ip
        )
    }
}

impl From<tokio_postgres::Row> for Session {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            csrf_token: row.get("csrf_token"),
            user_id: row.get("user_id"),
            roles: row.get("roles"),
            groups: row.get("groups"),
            is_mfa: row.get("is_mfa"),
            state: row.get("state"),
            exp: row.get("exp"),
            last_seen: row.get("last_seen"),
            remote_ip: row.get("remote_ip"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Open,
    Init,
    Auth,
    LoggedOut,
    Unknown,
}

impl FromStr for SessionState {
    type Err = ErrorResponse;

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

impl From<rauthy_api_types::sessions::SessionState> for SessionState {
    fn from(value: rauthy_api_types::sessions::SessionState) -> Self {
        match value {
            rauthy_api_types::sessions::SessionState::Open => Self::Open,
            rauthy_api_types::sessions::SessionState::Init => Self::Init,
            rauthy_api_types::sessions::SessionState::Auth => Self::Auth,
            rauthy_api_types::sessions::SessionState::LoggedOut => Self::LoggedOut,
            rauthy_api_types::sessions::SessionState::Unknown => Self::Unknown,
        }
    }
}

impl From<SessionState> for rauthy_api_types::sessions::SessionState {
    fn from(value: SessionState) -> Self {
        match value {
            SessionState::Open => Self::Open,
            SessionState::Init => Self::Init,
            SessionState::Auth => Self::Auth,
            SessionState::LoggedOut => Self::LoggedOut,
            SessionState::Unknown => Self::Unknown,
        }
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
    pub async fn delete(self) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM sessions WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(&self.id)).await?;
        } else {
            DB::pg_execute(sql, &[&self.id]).await?;
        }

        DB::hql().delete(Cache::Session, self.id).await?;

        Ok(())
    }

    pub async fn delete_by_user(user_id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM sessions WHERE user_id = $1 RETURNING id";
        let sids: Vec<String> = if is_hiqlite() {
            let rows = DB::hql().execute_returning(sql, params!(user_id)).await?;

            let mut ids = Vec::with_capacity(rows.len());
            for row in rows {
                ids.push(row?.get("id"));
            }
            ids
        } else {
            let rows = DB::pg_query_rows(sql, &[&user_id], 2).await?;
            let mut ids = Vec::with_capacity(rows.len());
            for row in rows {
                ids.push(row.get::<_, String>("id"));
            }
            ids
        };

        let client = DB::hql();
        for id in sids {
            client.delete(Cache::Session, id).await?;
        }

        Ok(())
    }

    // Returns a session by id
    pub async fn find(id: String) -> Result<Self, ErrorResponse> {
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::Session, id.clone()).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM sessions WHERE id = $1 ORDER BY exp DESC";
        let slf: Self = if is_hiqlite() {
            client.query_as_one(sql, params!(id)).await?
        } else {
            DB::pg_query_map_one(sql, &[&id]).await?
        };

        client
            .put(Cache::Session, slf.id.clone(), &slf, CACHE_TTL_SESSION)
            .await?;

        Ok(slf)
    }

    // not cached -> only used in the admin ui and can get very big
    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM sessions ORDER BY exp DESC";
        let sessions = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query_map(sql, &[], 4).await?
        };
        Ok(sessions)
    }

    pub async fn find_paginated(
        continuation_token: Option<ContinuationToken>,
        page_size: i64,
        offset: i64,
        backwards: bool,
    ) -> Result<(Vec<Self>, Option<ContinuationToken>), ErrorResponse> {
        // Allowing this unused assignment here makes the type conflicts
        // from sqlx later easier to handle.
        #[allow(unused_assignments)]
        let mut res = None;
        let mut latest_ts = 0;
        let size_hint = max(page_size as usize, 1);

        if let Some(token) = continuation_token {
            if backwards {
                let sql = r#"
SELECT *
FROM sessions
WHERE exp >= $1 AND id != $2
ORDER BY exp ASC
LIMIT $3
OFFSET $4"#;

                let mut rows: Vec<Self> = if is_hiqlite() {
                    DB::hql()
                        .query_as(sql, params!(token.ts, token.id, page_size, offset))
                        .await?
                } else {
                    DB::pg_query_map(sql, &[&token.ts, &token.id, &page_size, &offset], size_hint)
                        .await?
                };

                rows.reverse();
                if let Some(s) = rows.last() {
                    latest_ts = s.exp;
                }
                res = Some(rows);
            } else {
                let sql = r#"
SELECT *
FROM sessions
WHERE exp <= $1 AND id != $2
ORDER BY exp DESC
LIMIT $3
OFFSET $4"#;

                let rows: Vec<Self> = if is_hiqlite() {
                    DB::hql()
                        .query_as(sql, params!(token.ts, token.id, page_size, offset))
                        .await?
                } else {
                    DB::pg_query_map(sql, &[&token.ts, &token.id, &page_size, &offset], size_hint)
                        .await?
                };

                if let Some(s) = rows.last() {
                    latest_ts = s.exp;
                }
                res = Some(rows);
            };
        } else if backwards {
            // backwards without any continuation token will simply
            // serve the last elements without any other conditions
            let sql = r#"
SELECT *
FROM sessions
ORDER BY exp ASC
LIMIT $1
OFFSET $2"#;

            let mut rows: Vec<Self> = if is_hiqlite() {
                DB::hql().query_as(sql, params!(page_size, offset)).await?
            } else {
                DB::pg_query_map(sql, &[&page_size, &offset], size_hint).await?
            };

            rows.reverse();
            if let Some(s) = rows.last() {
                latest_ts = s.exp;
            }
            res = Some(rows);
        } else {
            let sql = r#"
SELECT *
FROM sessions
ORDER BY exp DESC
LIMIT $1
OFFSET $2"#;

            let rows: Vec<Self> = if is_hiqlite() {
                DB::hql().query_as(sql, params!(page_size, offset)).await?
            } else {
                DB::pg_query_map(sql, &[&page_size, &offset], size_hint).await?
            };

            if let Some(s) = rows.last() {
                latest_ts = s.exp;
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
    pub async fn invalidate_all() -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp() - 1;
        let sql = "UPDATE sessions SET exp = $1 WHERE exp > $1";

        let rows_affected = if is_hiqlite() {
            DB::hql().execute(sql, params!(now)).await?
        } else {
            DB::pg_execute(sql, &[&now]).await?
        };
        // at least the session used to invalidate all will always exist
        debug_assert!(rows_affected > 0);

        DB::hql().clear_cache(Cache::Session).await?;

        Ok(())
    }

    /// Returns `Vec<SessionId>`
    pub async fn invalidate_for_user(uid: &str) -> Result<Vec<String>, ErrorResponse> {
        let sql = "DELETE FROM sessions WHERE user_id = $1 RETURNING id";
        let sids: Vec<String> = if is_hiqlite() {
            let rows = DB::hql().execute_returning(sql, params!(uid)).await?;

            let mut ids = Vec::with_capacity(rows.len());
            for row in rows {
                ids.push(row?.get("id"));
            }
            ids
        } else {
            let rows = DB::pg_query_rows(sql, &[&uid], 1).await?;
            let mut ids = Vec::with_capacity(rows.len());
            for row in rows {
                ids.push(row.get("id"));
            }
            ids
        };

        let client = DB::hql();
        for sid in &sids {
            client.delete(Cache::Session, sid.clone()).await?;
        }

        Ok(sids)
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let state_str = &self.state;

        let sql = r#"
INSERT INTO
sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen, remote_ip)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
ON CONFLICT(id) DO UPDATE
SET user_id = $3, roles = $4, groups = $5, is_mfa = $6, state = $7, exp = $8, last_seen = $9,
    remote_ip = $10"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        &self.id,
                        &self.csrf_token,
                        &self.user_id,
                        &self.roles,
                        &self.groups,
                        self.is_mfa,
                        state_str,
                        self.exp,
                        self.last_seen,
                        &self.remote_ip
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.id,
                    &self.csrf_token,
                    &self.user_id,
                    &self.roles,
                    &self.groups,
                    &self.is_mfa,
                    &state_str,
                    &self.exp,
                    &self.last_seen,
                    &self.remote_ip,
                ],
            )
            .await?;
        }

        DB::hql()
            .put(Cache::Session, self.id.clone(), self, CACHE_TTL_SESSION)
            .await?;

        Ok(())
    }

    /// Caution: Uses regex / LIKE on the database -> very costly query
    pub async fn search(
        idx: &SearchParamsIdx,
        q: &str,
        limit: i64,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let q = format!("%{}%", q);

        let size_hint = max(limit as usize, 1);

        let sql = match idx {
            SearchParamsIdx::UserId => {
                "SELECT * FROM sessions WHERE user_id LIKE $1 ORDER BY exp DESC LIMIT $2"
            }
            SearchParamsIdx::Id | SearchParamsIdx::SessionId => {
                "SELECT * FROM sessions WHERE id LIKE $1 ORDER BY exp DESC LIMIT $2"
            }
            SearchParamsIdx::Ip => {
                "SELECT * FROM sessions WHERE remote_ip LIKE $1 ORDER BY exp DESC LIMIT $2"
            }
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "supported search idx for users: id / session_id, user_id, ip",
                ));
            }
        };

        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!(q, limit)).await?
        } else {
            DB::pg_query_map(sql, &[&q, &limit], size_hint).await?
        };

        Ok(res)
    }
}

impl Session {
    /// exp_in will be the time in seconds when the session will expire
    pub fn new(exp_in: u32, remote_ip: Option<IpAddr>) -> Self {
        let id = get_rand(32);
        let csrf_token = get_rand(32);
        let now = Utc::now();

        Self {
            id,
            csrf_token,
            user_id: None,
            roles: None,
            groups: None,
            is_mfa: false, // cannot be known at the creation stage
            state: SessionState::Init.as_str().to_string(),
            exp: now
                .add(chrono::Duration::seconds(exp_in as i64))
                .timestamp(),
            last_seen: now.timestamp(),
            remote_ip: remote_ip.map(|ip| ip.to_string()),
        }
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

        let now = Utc::now();

        // make sure to check the max session lifetime for expiring users
        let exp = if let Some(ts) = user.user_expires {
            if now.timestamp() > ts {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User has expired",
                ));
            } else {
                let target = now
                    .add(chrono::Duration::seconds(exp_in as i64))
                    .timestamp();
                if ts < target { ts } else { target }
            }
        } else {
            now.add(chrono::Duration::seconds(exp_in as i64))
                .timestamp()
        };

        Ok(Self {
            id,
            csrf_token,
            user_id,
            roles,
            groups,
            is_mfa: false, // cannot be known at the creation stage
            state: SessionState::Init.as_str().to_string(),
            exp,
            last_seen: now.timestamp(),
            remote_ip,
        })
    }

    pub fn client_cookie(&self) -> cookie::Cookie {
        let max_age = self.exp - Utc::now().timestamp();
        ApiCookie::build(COOKIE_SESSION, Cow::from(&self.id), max_age)
    }

    pub fn client_cookie_fed_cm(&self) -> cookie::Cookie {
        ApiCookie::build_with_same_site(
            COOKIE_SESSION_FED_CM,
            Cow::from(&self.id),
            *SESSION_LIFETIME_FED_CM,
            SameSite::None,
        )
    }

    pub fn extract_from_req(
        session_req: web::ReqData<Option<Session>>,
    ) -> Result<Session, ErrorResponse> {
        if session_req.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Session cookie is missing",
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

    pub async fn invalidate(self) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp() - 1;
        let state = SessionState::LoggedOut.as_str().to_string();

        let sql = "UPDATE sessions SET exp = $1, state = $2 WHERE id = $3";
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(now, state, &self.id))
                .await?;
        } else {
            DB::pg_execute(sql, &[&now, &state, &self.id]).await?;
        }

        DB::hql().delete(Cache::Session, self.id).await?;

        Ok(())
    }

    #[inline(always)]
    pub fn state(&self) -> Result<SessionState, ErrorResponse> {
        SessionState::from_str(self.state.as_str())
            .map_err(|_| ErrorResponse::new(ErrorResponseType::Internal, "invalid SessionState"))
    }

    /// Checks if the current session is valid: has not expired and has not timed out (last_seen)
    pub fn is_valid(&self, session_timeout: u32, remote_ip: Option<IpAddr>) -> bool {
        let now = Utc::now().timestamp();
        if self.exp < now {
            return false;
        }
        if self.last_seen < now - session_timeout as i64 {
            return false;
        }

        let state = match self.state() {
            Ok(s) => s,
            Err(err) => {
                error!("{}", err.message);
                return false;
            }
        };
        if state == SessionState::Unknown {
            return false;
        }
        if let Some(ip) = remote_ip {
            if (state == SessionState::Open || state == SessionState::Auth)
                && self.remote_ip != Some(ip.to_string())
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
    pub async fn set_mfa(&mut self, value: bool) -> Result<(), ErrorResponse> {
        self.is_mfa = value;
        self.save().await
    }

    #[inline(always)]
    pub fn validate_csrf(&self, req: &HttpRequest) -> Result<(), ErrorResponse> {
        let csrf = get_header_value(req, CSRF_HEADER);
        if csrf.is_err() {
            return Err(ErrorResponse::new(
                ErrorResponseType::CSRFTokenError,
                "CSRF Token not present in HTTP Header",
            ));
        }
        if self.csrf_token.eq(csrf?) {
            return Ok(());
        }
        Err(ErrorResponse::new(
            ErrorResponseType::CSRFTokenError,
            "CSRF Token is not correct",
        ))
    }

    pub fn validate_user_expiry(&mut self, user: &User) -> Result<(), ErrorResponse> {
        if let Some(ts) = user.user_expires {
            if Utc::now().timestamp() > ts {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User has expired",
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
