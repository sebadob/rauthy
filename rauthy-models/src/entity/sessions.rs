use crate::app_state::AppState;
use crate::entity::users::User;
use actix_web::cookie::{time, SameSite};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{cookie, web, HttpRequest};
use rauthy_common::constants::{
    CACHE_NAME_12HR, CACHE_NAME_SESSIONS, COOKIE_SESSION, CSRF_HEADER, DB_TYPE, IDX_SESSION,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::get_rand;
use rauthy_common::DbType;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, cache_remove, AckLevel};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};
use std::ops::Add;
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::error;
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum SessionState {
    Open,
    Init,
    Auth,
    LoggedOut,
    Unknown,
}

// impl FromRow<'_, AnyRow> for SessionState {
//     fn from_row(row: &'_ AnyRow) -> Result<Self, sqlx::error::Error> {
//         let s = row.try_get("state").unwrap();
//         Ok(Self::from_str(s).expect("Corrupted 'state' in 'sessions'"))
//     }
// }

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

/// CRUD
impl Session {
    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        sqlx::query("delete from sessions where id = $1")
            .bind(&self.id)
            .execute(&data.db)
            .await?;

        let idx = format!("{}{}", IDX_SESSION, &self.id);
        cache_remove(
            CACHE_NAME_SESSIONS.to_string(),
            idx,
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;

        Ok(())
    }

    // TODO add 'delete_by_user'

    /// Returns a session by id
    pub async fn find(data: &web::Data<AppState>, id: String) -> Result<Self, ErrorResponse> {
        // TODO set remote lookup to true here to be able to switch to in-memory sessions store only?
        let session = cache_get!(
            Session,
            CACHE_NAME_SESSIONS.to_string(),
            id.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if session.is_some() {
            return Ok(session.unwrap());
        }

        let session = sqlx::query_as::<_, Self>("select * from sessions where id = $1")
            .bind(&id)
            .fetch_one(&data.db)
            .await?;

        let idx = format!("{}{}", IDX_SESSION, session.id);
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

    // TODO should we even cache this, since this is only used rarely in the frontend?
    /// Returns all sessions and an empty Vec if not a single session exists
    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let sessions = sqlx::query_as::<_, Self>("select * from sessions")
            .fetch_all(&data.db)
            .await?;
        Ok(sessions)
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
            let idx = format!("{}{}", IDX_SESSION, &id);
            cache_remove(
                CACHE_NAME_SESSIONS.to_string(),
                idx,
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
            let idx = format!("{}{}", IDX_SESSION, &id);
            cache_remove(
                CACHE_NAME_SESSIONS.to_string(),
                idx,
                &data.caches.ha_cache_config,
                AckLevel::Quorum,
            )
            .await?;
        }

        Ok(())
    }

    /// Saves a Session
    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        match *DB_TYPE {
            DbType::Sqlite => sqlx::query(
                r#"insert or replace into
                sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
            ),
            DbType::Postgres => sqlx::query(
                r#"insert into
                sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                on conflict(id) do update set user_id = $3, roles = $4, groups = $5, is_mfa = $6,
                state = $7, exp = $8, last_seen = $9"#,
            ),
        }
        .bind(&self.id)
        .bind(&self.csrf_token)
        .bind(&self.user_id)
        .bind(&self.roles)
        .bind(&self.groups)
        .bind(self.is_mfa)
        .bind(self.state.as_str())
        .bind(self.exp)
        .bind(self.last_seen)
        .execute(&data.db)
        .await?;

        let idx = format!("{}{}", IDX_SESSION, &self.id);
        cache_insert(
            CACHE_NAME_SESSIONS.to_string(),
            idx,
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
    pub fn new(user: Option<&User>, exp_in: u32) -> Self {
        let id = get_rand(32);
        let csrf_token = get_rand(32);
        let mut user_id: Option<String> = None;
        let mut roles: Option<String> = None;
        let mut groups: Option<String> = None;

        if let Some(u) = user {
            user_id = Some(u.id.clone());
            roles = Some(u.roles.clone());
            groups = u.groups.clone();
        }

        let now = OffsetDateTime::now_utc();

        Self {
            id,
            csrf_token,
            user_id,
            roles,
            groups,
            is_mfa: false, // cannot be known at the creation stage
            state: SessionState::Init,
            exp: now
                .add(time::Duration::seconds(exp_in as i64))
                .unix_timestamp(),
            last_seen: now.unix_timestamp(),
        }
    }

    pub fn client_cookie(&self) -> cookie::Cookie {
        let exp = cookie::Expiration::from(
            OffsetDateTime::from_unix_timestamp(self.exp)
                .expect("Error with offset datetime calculation for client cookie"),
        );
        cookie::Cookie::build(COOKIE_SESSION, self.id.clone())
            .http_only(true)
            .secure(true)
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

    pub async fn invalidate(&mut self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let idx = format!("session_{}", &self.id);

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

        Ok(())
    }

    /// Checks if the current session is valid: has not expired and has not timed out (last_seen)
    pub fn is_valid(&self, session_timeout: u32) -> bool {
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
        let mut res = Vec::new();
        self.roles
            .as_ref()
            .unwrap()
            .split(',')
            .map(|x| format!("ROLE_{}", x.trim()))
            .for_each(|x| res.push(x));
        Ok(res)
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
