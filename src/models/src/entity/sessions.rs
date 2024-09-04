use crate::api_cookie::ApiCookie;
use crate::app_state::AppState;
use crate::cache::{Cache, DB};
use crate::entity::continuation_token::ContinuationToken;
use crate::entity::users::User;
use actix_web::cookie::{time, Cookie, SameSite};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{cookie, web, HttpRequest};
use chrono::Utc;
use rauthy_api_types::generic::SearchParamsIdx;
use rauthy_common::constants::{
    CACHE_TTL_SESSION, COOKIE_SESSION, COOKIE_SESSION_FED_CM, CSRF_HEADER, IDX_SESSION,
    SESSION_LIFETIME_FED_CM,
};
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow};
use std::borrow::Cow;
use std::net::IpAddr;
use std::ops::Add;
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::{error, warn};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
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
    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        sqlx::query!("DELETE FROM sessions WHERE id = $1", self.id)
            .execute(&data.db)
            .await?;

        DB::client()
            .delete(Cache::Session, Session::cache_idx(&self.id))
            .await?;

        Ok(())
    }

    pub async fn delete_by_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<(), ErrorResponse> {
        // TODO refactor to EXECUTE RETURNING that works with PG + SQLite
        let sessions: Vec<Self> =
            sqlx::query_as!(Self, "SELECT * FROM sessions WHERE user_id = $1", user_id)
                .fetch_all(&data.db)
                .await?;

        sqlx::query!("DELETE FROM sessions WHERE user_id = $1", user_id)
            .execute(&data.db)
            .await?;

        let client = DB::client();
        for s in sessions {
            client
                .delete(Cache::Session, Session::cache_idx(&s.id))
                .await?;
        }

        Ok(())
    }

    // Returns a session by id
    pub async fn find(data: &web::Data<AppState>, id: String) -> Result<Self, ErrorResponse> {
        let idx = Session::cache_idx(&id);
        let client = DB::client();

        if let Some(slf) = client.get(Cache::Session, &idx).await? {
            return Ok(slf);
        }

        let session = sqlx::query_as!(
            Self,
            "SELECT * FROM sessions WHERE id = $1 ORDER BY exp DESC",
            id
        )
        .fetch_one(&data.db)
        .await?;

        client
            .put(Cache::Session, idx, &session, CACHE_TTL_SESSION)
            .await?;

        Ok(session)
    }

    // not cached, since this is only used in the admin ui
    /// Returns all sessions and an empty Vec if not a single session exists
    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let sessions = sqlx::query_as!(Self, "SELECT * FROM sessions ORDER BY exp DESC")
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
        // Allowing this unused assignment here makes the type conflicts
        // from sqlx later easier to handle.
        #[allow(unused_assignments)]
        let mut res = None;
        let mut latest_ts = 0;

        if let Some(token) = continuation_token {
            if backwards {
                offset += page_size;
                let mut rows: Vec<Self> = sqlx::query_as!(
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

                rows.reverse();
                if let Some(s) = rows.last() {
                    latest_ts = s.exp;
                }
                res = Some(rows);
            } else {
                let rows = sqlx::query_as!(
                    Self,
                    r#"SELECT *
                    FROM sessions
                    WHERE exp <= $1 AND id != $2
                    ORDER BY exp DESC
                    LIMIT $3
                    OFFSET $4"#,
                    token.ts,
                    token.id,
                    page_size,
                    offset,
                )
                .fetch_all(&data.db)
                .await?;

                if let Some(s) = rows.last() {
                    latest_ts = s.exp;
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
                   ORDER BY exp ASC
                   LIMIT $1
                   OFFSET $2"#,
                page_size,
                offset,
            )
            .fetch_all(&data.db)
            .await?;

            rows.reverse();
            if let Some(s) = rows.last() {
                latest_ts = s.exp;
            }
            res = Some(rows);
        } else {
            let rows = sqlx::query_as!(
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

        let client = DB::client();
        for id in removed {
            client
                .delete(Cache::Session, Session::cache_idx(&id))
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

        let client = DB::client();
        for id in removed {
            client
                .delete(Cache::Session, Session::cache_idx(&id))
                .await?;
        }

        Ok(())
    }

    /// Saves a Session
    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let state_str = &self.state;

        #[cfg(not(feature = "postgres"))]
        let q = sqlx::query!(
            r#"INSERT OR REPLACE INTO
            sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen, remote_ip)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
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
            r#"INSERT INTO
            sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen, remote_ip)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT(id) DO UPDATE set user_id = $3, roles = $4, groups = $5, is_mfa = $6,
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

        DB::client()
            .put(
                Cache::Session,
                Session::cache_idx(&self.id),
                self,
                CACHE_TTL_SESSION,
            )
            .await?;

        Ok(())
    }

    /// Caution: Uses regex / LIKE on the database -> very costly query
    pub async fn search(
        data: &web::Data<AppState>,
        idx: &SearchParamsIdx,
        q: &str,
        limit: i64,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let q = format!("%{}%", q);

        let res = match idx {
            SearchParamsIdx::UserId => {
                query_as!(
                    Self,
                    "SELECT * FROM sessions WHERE user_id LIKE $1 ORDER BY exp DESC LIMIT $2",
                    q,
                    limit
                )
                .fetch_all(&data.db)
                .await?
            }
            SearchParamsIdx::Id | SearchParamsIdx::SessionId => {
                query_as!(
                    Self,
                    "SELECT * FROM sessions WHERE id LIKE $1 ORDER BY exp DESC LIMIT $2",
                    q,
                    limit
                )
                .fetch_all(&data.db)
                .await?
            }
            SearchParamsIdx::Ip => {
                query_as!(
                    Self,
                    "SELECT * FROM sessions WHERE remote_ip LIKE $1 ORDER BY exp DESC LIMIT $2",
                    q,
                    limit
                )
                .fetch_all(&data.db)
                .await?
            }
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "supported search idx for users: id / session_id, user_id, ip",
                ))
            }
        };

        Ok(res)
    }
}

impl Session {
    /// exp_in will be the time in seconds when the session will expire
    pub fn new(exp_in: u32, remote_ip: Option<IpAddr>) -> Self {
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
            state: SessionState::Init.as_str().to_string(),
            exp: now
                .add(time::Duration::seconds(exp_in as i64))
                .unix_timestamp(),
            last_seen: now.unix_timestamp(),
            remote_ip: remote_ip.map(|ip| ip.to_string()),
        }
    }

    #[inline(always)]
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
                    "User has expired",
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
            state: SessionState::Init.as_str().to_string(),
            exp,
            last_seen: now.unix_timestamp(),
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

    pub async fn invalidate(
        &mut self,
        data: &web::Data<AppState>,
    ) -> Result<Cookie, ErrorResponse> {
        let idx = Session::cache_idx(&self.id);

        self.exp = OffsetDateTime::now_utc().unix_timestamp();
        self.state = SessionState::LoggedOut.as_str().to_string();

        sqlx::query("update sessions set exp = $1, state = $2 where id = $3")
            .bind(self.exp)
            .bind(self.state.as_str())
            .bind(&self.id)
            .execute(&data.db)
            .await?;

        DB::client().delete(Cache::Session, idx).await?;

        Ok(ApiCookie::build(COOKIE_SESSION, &self.id, 0))
    }

    #[inline(always)]
    pub fn state(&self) -> Result<SessionState, ErrorResponse> {
        SessionState::from_str(self.state.as_str())
            .map_err(|_| ErrorResponse::new(ErrorResponseType::Internal, "invalid SessionState"))
    }

    /// Checks if the current session is valid: has not expired and has not timed out (last_seen)
    pub fn is_valid(&self, session_timeout: u32, remote_ip: Option<IpAddr>) -> bool {
        let now = OffsetDateTime::now_utc().unix_timestamp();
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
                "CSRF Token not present in HTTP Header",
            ));
        }
        if self.csrf_token.eq(csrf.unwrap()) {
            return Ok(());
        }
        Err(ErrorResponse::new(
            ErrorResponseType::CSRFTokenError,
            "CSRF Token is not correct",
        ))
    }

    pub fn validate_user_expiry(&mut self, user: &User) -> Result<(), ErrorResponse> {
        if let Some(ts) = user.user_expires {
            if OffsetDateTime::now_utc().unix_timestamp() > ts {
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
