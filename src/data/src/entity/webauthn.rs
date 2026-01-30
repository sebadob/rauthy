use crate::api_cookie::ApiCookie;
use crate::database::{Cache, DB};
use crate::entity::auth_codes::AuthCodeToSAwait;
use crate::entity::browser_id::BrowserId;
use crate::entity::login_locations::LoginLocation;
use crate::entity::password::PasswordPolicy;
use crate::entity::sessions::Session;
use crate::entity::users::{AccountType, User};
use crate::rauthy_config::RauthyConfig;
use actix_web::cookie::Cookie;
use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, HeaderValue,
};
use actix_web::http::{StatusCode, header};
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder};
use chrono::Utc;
use cryptr::EncValue;
use deadpool_postgres::GenericClient;
use hiqlite::Params;
use hiqlite_macros::params;
use rauthy_api_types::tos::ToSAwaitLoginResponse;
use rauthy_api_types::users::{
    MfaPurpose, PasskeyResponse, WebauthnAuthFinishRequest, WebauthnAuthStartResponse,
    WebauthnLoginFinishResponse, WebauthnRegFinishRequest, WebauthnRegStartRequest,
};
use rauthy_common::constants::{COOKIE_MFA, IDX_WEBAUTHN};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{base64_decode, base64_encode, deserialize, get_rand, serialize};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::{error, info, warn};
use utoipa::ToSchema;
use webauthn_rs::prelude::*;
use webauthn_rs_proto::{
    AuthenticatorSelectionCriteria, ResidentKeyRequirement, UserVerificationPolicy,
};

#[derive(Clone, Deserialize, Serialize)]
pub struct PasskeyEntity {
    pub user_id: String,
    pub name: String,
    pub passkey_user_id: String,
    pub passkey: String,
    pub credential_id: Vec<u8>,
    pub registered: i64,
    pub last_used: i64,
    pub user_verified: Option<bool>,
}

impl From<tokio_postgres::Row> for PasskeyEntity {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            user_id: row.get("user_id"),
            name: row.get("name"),
            passkey_user_id: row.get("passkey_user_id"),
            passkey: row.get("passkey"),
            credential_id: row.get("credential_id"),
            registered: row.get("registered"),
            last_used: row.get("last_used"),
            user_verified: row.get("user_verified"),
        }
    }
}

impl Debug for PasskeyEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PasskeyEntity {{ user_id: {}, name: {}, passkey_user_id: {}, passkey: <hidden>, \
        credential_id: <hidden>, registered: {}, last_used: {}, user_verified: {:?} }}",
            self.user_id,
            self.name,
            self.passkey_user_id,
            self.registered,
            self.last_used,
            self.user_verified
        )
    }
}

// CRUD
impl PasskeyEntity {
    /// If the `User` is `Some(_)`, a `User::save()` will be included in the `txn`
    pub async fn create(
        user_id: String,
        user: Option<User>,
        passkey_user_id: Uuid,
        name: String,
        pk: Passkey,
        user_verified: bool,
    ) -> Result<(), ErrorResponse> {
        // json, because bincode does not support deserialize from any, which would be the case here
        let passkey = serde_json::to_string(&pk)?;
        let now = Utc::now().timestamp();

        let entity = Self {
            user_id,
            name,
            passkey_user_id: passkey_user_id.to_string(),
            passkey,
            credential_id: pk.cred_id().to_vec(),
            registered: now,
            last_used: now,
            user_verified: Some(user_verified),
        };

        let user_email = user.as_ref().map(|u| u.email.clone());
        let client = DB::hql();

        let sql = r#"
INSERT INTO passkeys
(user_id, name, passkey_user_id, passkey, credential_id, registered, last_used, user_verified)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#;

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(2);

            if let Some(user) = user {
                debug_assert!(user.webauthn_user_id.is_some());
                user.save_txn_append(&mut txn);
            }

            txn.push((
                sql,
                params!(
                    &entity.user_id,
                    entity.name,
                    entity.passkey_user_id,
                    entity.passkey,
                    entity.credential_id,
                    now,
                    now,
                    entity.user_verified
                ),
            ));

            client.txn(txn).await?;
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            if let Some(user) = user {
                debug_assert!(user.webauthn_user_id.is_some());
                user.save_txn(&txn).await?;
            }
            DB::pg_txn_append(
                &txn,
                sql,
                &[
                    &entity.user_id,
                    &entity.name,
                    &entity.passkey_user_id,
                    &entity.passkey,
                    &entity.credential_id,
                    &now,
                    &now,
                    &entity.user_verified,
                ],
            )
            .await?;

            txn.commit().await?;
        }

        if let Some(email) = user_email {
            User::invalidate_cache(&entity.user_id, &email).await?;
        }
        client
            .delete(Cache::Webauthn, Self::cache_idx_user(&entity.user_id))
            .await?;
        client
            .delete(
                Cache::Webauthn,
                Self::cache_idx_user_with_uv(&entity.user_id),
            )
            .await?;
        client
            .delete(Cache::Webauthn, Self::cache_idx_creds(&entity.user_id))
            .await?;

        Ok(())
    }

    pub async fn count_for_user(user_id: String) -> Result<i64, ErrorResponse> {
        let sql = "SELECT COUNT (*) AS count FROM passkeys WHERE user_id = $1";
        let count: i64 = if is_hiqlite() {
            DB::hql()
                .query_raw_one(sql, params!(user_id))
                .await?
                .get("count")
        } else {
            DB::pg_query_one_row(sql, &[&user_id]).await?.get("count")
        };

        Ok(count)
    }

    pub async fn delete(user_id: String, name: String) -> Result<(), ErrorResponse> {
        // if we delete a passkey, we must check if this is the last existing one for the user
        let pk_count = Self::count_for_user(user_id.clone()).await?;

        let mut user_to_save: Option<User> = None;
        let mut user_email: Option<String> = None;

        if pk_count < 2 {
            let mut user = User::find(user_id.clone()).await?;
            user.webauthn_user_id = None;

            // in this case, we need to check against the current password policy,
            // if the password should expire again
            let policy = PasswordPolicy::find().await?;
            if let Some(valid_days) = policy.valid_days {
                if user.password.is_some() {
                    user.password_expires = Some(
                        Utc::now()
                            .add(chrono::Duration::days(valid_days as i64))
                            .timestamp(),
                    );
                } else {
                    user.password_expires = None;
                }
            }

            user_email = Some(user.email.clone());
            user_to_save = Some(user);
        }

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(2);

            if let Some(user) = user_to_save {
                user.save_txn_append(&mut txn);
            }
            Self::delete_by_id_name_append(user_id.clone(), name.clone(), &mut txn);

            DB::hql().txn(txn).await?;
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            if let Some(user) = user_to_save {
                user.save_txn(&txn).await?;
            }
            Self::delete_by_id_name(&user_id, &name, &txn).await?;

            txn.commit().await?;
        }

        Self::clear_caches_by_id_name(&user_id, user_email, &name).await?;

        Ok(())
    }

    /// MUST call `PasskeyEntity::clear_caches_by_id_name()` after txn commit!
    async fn delete_by_id_name(
        user_id: &str,
        name: &str,
        txn: &deadpool_postgres::Transaction<'_>,
    ) -> Result<(), ErrorResponse> {
        DB::pg_txn_append(
            txn,
            "DELETE FROM passkeys WHERE user_id = $1 AND name = $2",
            &[&user_id, &name],
        )
        .await?;
        Ok(())
    }

    /// MUST call `PasskeyEntity::clear_caches_by_id_name()` after txn commit!
    fn delete_by_id_name_append(user_id: String, name: String, txn: &mut Vec<(&str, Params)>) {
        txn.push((
            "DELETE FROM passkeys WHERE user_id = $1 AND name = $2",
            params!(user_id, name),
        ));
    }

    async fn clear_caches_by_id_name(
        user_id: &str,
        user_email: Option<String>,
        name: &str,
    ) -> Result<(), ErrorResponse> {
        let client = DB::hql();

        if let Some(email) = user_email {
            User::invalidate_cache(user_id, &email).await?;
        }

        client
            .delete(Cache::Webauthn, Self::cache_idx_single(user_id, name))
            .await?;
        client
            .delete(Cache::Webauthn, Self::cache_idx_user(user_id))
            .await?;
        client
            .delete(Cache::Webauthn, Self::cache_idx_user_with_uv(user_id))
            .await?;
        client
            .delete(Cache::Webauthn, Self::cache_idx_creds(user_id))
            .await?;

        Ok(())
    }

    pub async fn find(user_id: &str, name: &str) -> Result<Self, ErrorResponse> {
        let idx = Self::cache_idx_single(user_id, name);
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM passkeys WHERE user_id = $1 AND name = $2";
        let slf = if is_hiqlite() {
            client.query_as_one(sql, params!(user_id, name)).await?
        } else {
            DB::pg_query_one(sql, &[&user_id, &name]).await?
        };

        let ttl = Some(RauthyConfig::get().vars.webauthn.req_exp as i64);
        client.put(Cache::Webauthn, idx, &slf, ttl).await?;

        Ok(slf)
    }

    pub async fn find_cred_ids_for_user(user_id: &str) -> Result<Vec<CredentialID>, ErrorResponse> {
        let idx = Self::cache_idx_creds(user_id);
        let client = DB::hql();

        let opt: Option<Vec<Vec<u8>>> = client.get(Cache::Webauthn, &idx).await?;
        if let Some(bytes) = opt {
            return Ok(bytes.into_iter().map(CredentialID::from).collect());
        }

        let sql = "SELECT credential_id FROM passkeys WHERE user_id = $1";
        let creds = if is_hiqlite() {
            client
                .query_raw(sql, params!(user_id))
                .await?
                .into_iter()
                .map(|mut r| r.get::<Vec<u8>>("credential_id"))
                .collect::<Vec<_>>()
        } else {
            DB::pg_query_rows(sql, &[&user_id], 2)
                .await?
                .into_iter()
                .map(|r| r.get::<_, Vec<u8>>("credential_id"))
                .collect::<Vec<_>>()
        };

        let ttl = Some(RauthyConfig::get().vars.webauthn.req_exp as i64);
        client.put(Cache::Webauthn, idx, &creds, ttl).await?;

        Ok(creds.into_iter().map(CredentialID::from).collect())
    }

    pub async fn find_for_user(user_id: &str) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx_user(user_id);
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM passkeys WHERE user_id = $1";
        let pks = if is_hiqlite() {
            client.query_as(sql, params!(user_id)).await?
        } else {
            DB::pg_query(sql, &[&user_id], 2).await?
        };

        let ttl = Some(RauthyConfig::get().vars.webauthn.req_exp as i64);
        client.put(Cache::Webauthn, idx, &pks, ttl).await?;

        Ok(pks)
    }

    pub async fn find_for_user_with_uv(user_id: &str) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx_user_with_uv(user_id);
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM passkeys WHERE user_id = $1 AND user_verified = true";
        let pks = if is_hiqlite() {
            client.query_as(sql, params!(user_id)).await?
        } else {
            DB::pg_query(sql, &[&user_id], 2).await?
        };

        let ttl = Some(RauthyConfig::get().vars.webauthn.req_exp as i64);
        client.put(Cache::Webauthn, idx, &pks, ttl).await?;

        Ok(pks)
    }

    pub fn update_passkey_txn_append(&self, txn: &mut Vec<(&str, Params)>) {
        txn.push((
            r#"
UPDATE passkeys
SET passkey = $1, last_used = $2
WHERE user_id = $3 AND name = $4"#,
            params!(&self.passkey, self.last_used, &self.user_id, &self.name),
        ));
    }

    pub async fn update_passkey_txn(
        &self,
        txn: &deadpool_postgres::Transaction<'_>,
    ) -> Result<(), ErrorResponse> {
        DB::pg_txn_append(
            txn,
            r#"
UPDATE passkeys
SET passkey = $1, last_used = $2
WHERE user_id = $3 AND name = $4"#,
            &[&self.passkey, &self.last_used, &self.user_id, &self.name],
        )
        .await?;

        Ok(())
    }

    async fn update_caches_after_update(&self) -> Result<(), ErrorResponse> {
        let client = DB::hql();

        let ttl = Some(RauthyConfig::get().vars.webauthn.req_exp as i64);
        client
            .put(
                Cache::Webauthn,
                Self::cache_idx_single(&self.user_id, &self.name),
                self,
                ttl,
            )
            .await?;

        client
            .delete(Cache::Webauthn, Self::cache_idx_user(&self.user_id))
            .await?;
        client
            .delete(Cache::Webauthn, Self::cache_idx_user_with_uv(&self.user_id))
            .await?;

        Ok(())
    }
}

impl PasskeyEntity {
    #[inline]
    pub fn get_pk(&self) -> Passkey {
        // Passkeys cannot be serialized with bincode -> no support for deserialize from any
        serde_json::from_str(&self.passkey).unwrap()
    }

    /// Index for a single passkey for a user
    #[inline]
    fn cache_idx_single(user_id: &str, name: &str) -> String {
        format!("{IDX_WEBAUTHN}{user_id}{name}")
    }

    /// Index for all passkeys a user has
    #[inline]
    fn cache_idx_user(user_id: &str) -> String {
        format!("{IDX_WEBAUTHN}{user_id}")
    }

    /// Index for all passkeys a user has
    #[inline]
    fn cache_idx_user_with_uv(user_id: &str) -> String {
        format!("{IDX_WEBAUTHN}_UV_{user_id}",)
    }

    /// Index for credentials for a user
    #[inline]
    fn cache_idx_creds(user_id: &str) -> String {
        format!("{IDX_WEBAUTHN}{user_id}_creds")
    }
}

impl From<PasskeyEntity> for PasskeyResponse {
    fn from(value: PasskeyEntity) -> Self {
        Self {
            name: value.name,
            registered: value.registered,
            last_used: value.last_used,
            user_verified: value.user_verified,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebauthnCookie {
    pub email: String,
    pub exp: OffsetDateTime,
}

impl WebauthnCookie {
    pub fn new(email: String) -> Self {
        let renew = RauthyConfig::get().vars.webauthn.renew_exp as i64;
        let exp = OffsetDateTime::now_utc().add(::time::Duration::hours(renew));
        Self { email, exp }
    }

    pub fn build(&self) -> Result<Cookie<'_>, ErrorResponse> {
        let ser = serialize(self)?;
        let enc = EncValue::encrypt(&ser)?.into_bytes();
        let b64 = base64_encode(&enc);

        let max_age = self.exp.unix_timestamp() - Utc::now().timestamp();
        Ok(ApiCookie::build(COOKIE_MFA, b64, max_age))
    }

    pub fn parse_validate(cookie: &Option<String>) -> Result<Self, ErrorResponse> {
        if cookie.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Webauthn Cookie is missing",
            ));
        }
        let cookie = cookie.as_ref().unwrap();
        let bytes = base64_decode(cookie)?;
        let dec = EncValue::try_from(bytes)?.decrypt()?;
        let slf = deserialize::<Self>(&dec)?;

        if slf.exp < OffsetDateTime::now_utc() {
            Err(ErrorResponse::new(
                ErrorResponseType::SessionExpired,
                "Webauthn Cookie has expired",
            ))
        } else {
            Ok(slf)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebauthnData {
    pub code: String,
    // auth_state cannot be serialized directly with bincode -> no support for deserialize from any
    pub auth_state_json: String,
    pub data: WebauthnAdditionalData,
}

// CURD
impl WebauthnData {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::Webauthn, self.code.clone()).await?;
        Ok(())
    }

    pub async fn find(code: String) -> Result<Self, ErrorResponse> {
        let res: Option<Self> = DB::hql().get(Cache::Webauthn, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Webauthn Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = Some(RauthyConfig::get().vars.webauthn.data_exp as i64);
        DB::hql()
            .put(Cache::Webauthn, self.code.clone(), &self, ttl)
            .await?;
        Ok(())
    }
}

// This is the data, that will be passed to the client as response to a successful MFA auth
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum WebauthnAdditionalData {
    Login(WebauthnLoginReq),
    Service(WebauthnServiceReq),
    Test,
    LoginToSAwait(WebauthnLoginToSAwaitCode),
}

impl WebauthnAdditionalData {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        match self {
            Self::Login(d) => d.delete().await,
            // The service req data is not deleted here, but actually further down the road
            // after the service req has been made.
            Self::Service(_) => Ok(()),
            Self::Test => Ok(()),
            Self::LoginToSAwait(_) => Ok(()),
        }
    }

    pub fn into_response(self) -> HttpResponse {
        match self {
            Self::Login(login_req) => {
                let header_loc = (
                    header::LOCATION,
                    HeaderValue::from_str(&login_req.header_loc).unwrap(),
                );

                let mut builder = if login_req.needs_user_update {
                    HttpResponse::ResetContent()
                } else {
                    let mut builder = HttpResponse::Accepted();
                    builder.insert_header(header_loc);
                    builder
                };

                if let Some(value) = login_req.header_origin {
                    builder.insert_header((
                        header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        HeaderValue::from_str(&value).unwrap(),
                    ));
                }

                if login_req.needs_user_update {
                    builder.finish()
                } else {
                    builder.json(WebauthnLoginFinishResponse {
                        loc: login_req.header_loc,
                    })
                }
            }

            Self::Service(svc_req) => HttpResponse::Accepted().json(svc_req),

            Self::Test => HttpResponse::Accepted().finish(),

            Self::LoginToSAwait(tos_req) => {
                let mut resp = HttpResponseBuilder::new(StatusCode::from_u16(206).unwrap()).json(
                    &ToSAwaitLoginResponse {
                        tos_await_code: tos_req.await_code,
                        user_id: None,
                        force_accept: None,
                    },
                );
                if let Some(origin) = tos_req.header_origin {
                    resp.headers_mut()
                        .insert(header::ORIGIN, HeaderValue::from_str(&origin).unwrap());
                    resp.headers_mut().insert(
                        ACCESS_CONTROL_ALLOW_METHODS,
                        HeaderValue::from_static("POST"),
                    );
                    resp.headers_mut().insert(
                        ACCESS_CONTROL_ALLOW_CREDENTIALS,
                        HeaderValue::from_static("true"),
                    );
                }
                resp
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct WebauthnLoginReq {
    pub code: String,
    pub user_id: String,
    pub header_loc: String,
    pub header_origin: Option<String>,
    pub tos_await_data: Option<WebauthnToSAwaitData>,
    pub needs_user_update: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct WebauthnToSAwaitData {
    pub auth_code: String,
    pub auth_code_lifetime: i32,
}

// CRUD
impl WebauthnLoginReq {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::Webauthn, self.code.clone()).await?;
        Ok(())
    }

    pub async fn find(code: String) -> Result<Self, ErrorResponse> {
        let res: Option<Self> = DB::hql().get(Cache::Webauthn, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Webauthn Login Request Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = Some(RauthyConfig::get().vars.webauthn.data_exp as i64);
        DB::hql()
            .put(Cache::Webauthn, self.code.clone(), self, ttl)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct WebauthnServiceReq {
    pub code: String,
    pub user_id: String,
}

// CRUD
impl WebauthnServiceReq {
    pub fn new(user_id: String) -> Self {
        Self {
            code: get_rand(48),
            user_id,
        }
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::Webauthn, self.code.clone()).await?;
        Ok(())
    }

    pub async fn find(code: String) -> Result<Self, ErrorResponse> {
        let res = DB::hql().get(Cache::Webauthn, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Webauthn Service Request Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = Some(RauthyConfig::get().vars.webauthn.data_exp as i64);
        DB::hql()
            .put(Cache::Webauthn, self.code.clone(), self, ttl)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct WebauthnLoginToSAwaitCode {
    await_code: String,
    user_id: String,
    header_origin: Option<String>,
}

pub async fn auth_start(
    user_id: String,
    purpose: MfaPurpose,
) -> Result<WebauthnAuthStartResponse, ErrorResponse> {
    // This app_data will be returned to the client upon successful webauthn authentication
    let add_data = match purpose {
        MfaPurpose::Login(code) => {
            let d = WebauthnLoginReq::find(code).await?;
            WebauthnAdditionalData::Login(d)
        }
        MfaPurpose::MfaModToken
        | MfaPurpose::PamLogin
        | MfaPurpose::PasswordNew
        | MfaPurpose::PasswordReset => {
            let svc_req = WebauthnServiceReq::new(user_id.clone());
            svc_req.save().await?;
            WebauthnAdditionalData::Service(svc_req)
        }
        MfaPurpose::Test => WebauthnAdditionalData::Test,
    };

    let user = User::find(user_id).await?;
    let force_uv =
        user.account_type() == AccountType::Passkey || RauthyConfig::get().vars.webauthn.force_uv;
    let pks = if force_uv {
        // in this case, filter out all presence only keys
        PasskeyEntity::find_for_user_with_uv(&user.id)
            .await?
            .iter()
            .map(|pk_entity| pk_entity.get_pk())
            .collect::<Vec<Passkey>>()
    } else {
        PasskeyEntity::find_for_user(&user.id)
            .await?
            .iter()
            .map(|pk_entity| pk_entity.get_pk())
            .collect::<Vec<Passkey>>()
    };

    if pks.is_empty() {
        // may be the case if the user has presence only keys and the config has changed
        return Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "No Security Keys with active user verification found",
        ));
    }

    match RauthyConfig::get()
        .webauthn
        .start_passkey_authentication(pks.as_slice())
    {
        Ok((mut rcr, auth_state)) => {
            let req_exp = RauthyConfig::get().vars.webauthn.req_exp;
            // timeout expected in ms
            rcr.public_key.timeout = Some(req_exp as u32 * 1000);

            if force_uv {
                rcr.public_key.user_verification = UserVerificationPolicy::Required;
            }
            add_data.delete().await?;

            // cannot be serialized with bincode -> no deserialize from any
            let auth_state_json = serde_json::to_string(&auth_state)?;
            let auth_data = WebauthnData {
                code: get_rand(48),
                auth_state_json,
                data: add_data,
            };
            auth_data.save().await?;

            Ok(WebauthnAuthStartResponse {
                code: auth_data.code,
                rcr,
                user_id: user.id,
                exp: req_exp as u64,
            })
        }

        Err(err) => {
            error!(?err, "Webauthn challenge authentication");
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Internal error with Webauthn Challenge Authentication",
            ))
        }
    }
}

pub async fn auth_finish(
    user_id: String,
    req: &HttpRequest,
    browser_id: BrowserId,
    session: Option<Session>,
    payload: WebauthnAuthFinishRequest,
) -> Result<WebauthnAdditionalData, ErrorResponse> {
    let auth_data = WebauthnData::find(payload.code).await?;
    auth_data.delete().await?;
    let auth_state = serde_json::from_str(&auth_data.auth_state_json)?;

    let mut user = User::find(user_id).await?;
    let force_uv =
        user.account_type() == AccountType::Passkey || RauthyConfig::get().vars.webauthn.force_uv;

    let pks = PasskeyEntity::find_for_user(&user.id).await?;

    match RauthyConfig::get()
        .webauthn
        .finish_passkey_authentication(&payload.data, &auth_state)
    {
        Ok(auth_result) => {
            if force_uv && !auth_result.user_verified() {
                warn!(
                    user.id,
                    "Webauthn Authentication Ceremony without User Verification",
                );
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User Presence only is not allowed - Verification is needed",
                ));
            }
            let uid = user.id.clone();

            LoginLocation::spawn_background_check(user.clone(), req, browser_id)?;

            if matches!(auth_data.data, WebauthnAdditionalData::Login(_))
                && let Some(mut session) = session
            {
                session.set_authenticated(&user).await?;
            }

            if auth_result.needs_update() {
                for mut pk_entity in pks {
                    let mut pk = pk_entity.get_pk();
                    if let Some(updated) = pk.update_credential(&auth_result) {
                        if updated {
                            pk_entity.passkey = serde_json::to_string(&pk)?;
                        }

                        let now = OffsetDateTime::now_utc().unix_timestamp();
                        pk_entity.last_used = now;
                        user.last_login = Some(now);
                        user.last_failed_login = None;
                        user.failed_login_attempts = None;

                        if is_hiqlite() {
                            let mut txn = Vec::with_capacity(2);
                            pk_entity.update_passkey_txn_append(&mut txn);
                            user.save_txn_append(&mut txn);
                            DB::hql().txn(txn).await?;
                        } else {
                            let mut cl = DB::pg().await?;
                            let txn = cl.transaction().await?;
                            pk_entity.update_passkey_txn(&txn).await?;
                            user.save_txn(&txn).await?;
                            txn.commit().await?;
                        }

                        pk_entity.update_caches_after_update().await?;

                        // there can only be exactly one key that needs the update
                        break;
                    }
                }
            }

            info!(user.id = uid, "Webauthn Authentication successful");

            if let WebauthnAdditionalData::Login(data) = auth_data.data {
                data.delete().await?;

                if let Some(tos_data) = data.tos_await_data {
                    let code_await = AuthCodeToSAwait {
                        auth_code: tos_data.auth_code,
                        await_code: AuthCodeToSAwait::generate_code(),
                        auth_code_lifetime: tos_data.auth_code_lifetime,
                        header_loc: data.header_loc,
                        header_origin: data.header_origin.clone(),
                        needs_user_update: data.needs_user_update,
                    };
                    code_await.save().await?;

                    Ok(WebauthnAdditionalData::LoginToSAwait(
                        WebauthnLoginToSAwaitCode {
                            await_code: code_await.await_code,
                            user_id: uid,
                            header_origin: data.header_origin,
                        },
                    ))
                } else {
                    Ok(WebauthnAdditionalData::Login(data))
                }
            } else {
                Ok(auth_data.data)
            }
        }
        Err(err) => {
            error!(?err, "Webauthn Auth Finish");
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                err.to_string(),
            ))
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebauthnReg {
    pub user_id: String,
    pub passkey_user_id: Uuid,
    pub reg_state: String,
}

pub async fn reg_start(
    user_id: String,
    payload: WebauthnRegStartRequest,
) -> Result<CreationChallengeResponse, ErrorResponse> {
    let user = User::find(user_id).await?;
    let passkey_user_id = if let Some(id) = &user.webauthn_user_id {
        Uuid::from_str(id).expect("corrupted database: user.webauthn_user_id")
    } else {
        Uuid::new_v4()
    };
    let cred_ids = PasskeyEntity::find_cred_ids_for_user(&user.id).await?;

    match RauthyConfig::get().webauthn.start_passkey_registration(
        passkey_user_id,
        &user.email,
        &user.email,
        Some(cred_ids),
    ) {
        Ok((mut ccr, reg_state)) => {
            // timeout expected in ms
            let cfg = &RauthyConfig::get().vars.webauthn;
            ccr.public_key.timeout = Some(cfg.req_exp as u32 * 1000);

            if cfg.force_uv || user.account_type() == AccountType::Passkey {
                // in this case we need to force UV no matter what is set in the config
                ccr.public_key.authenticator_selection =
                    if let Some(mut auth_sel) = ccr.public_key.authenticator_selection {
                        auth_sel.user_verification = UserVerificationPolicy::Required;
                        Some(auth_sel)
                    } else {
                        Some(AuthenticatorSelectionCriteria {
                            authenticator_attachment: None,
                            resident_key: Some(ResidentKeyRequirement::Discouraged),
                            require_resident_key: false,
                            user_verification: UserVerificationPolicy::Required,
                        })
                    };
            };

            let reg_data = WebauthnReg {
                user_id: user.id.clone(),
                passkey_user_id,
                // the reg_state cannot be serialized with bincode -> missing deserialize from Any
                reg_state: serde_json::to_string(&reg_state)?,
            };

            // persist the reg_state
            let idx = format!("reg_{:?}_{}", payload.passkey_name, user.id);
            DB::hql()
                .put(Cache::Webauthn, idx, &reg_data, Some(cfg.req_exp as i64))
                .await?;

            Ok(ccr)
        }

        Err(err) => {
            error!(?err, "Webauthn challenge register");
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Internal error with Webauthn Challenge Registration",
            ))
        }
    }
}

pub async fn reg_finish(
    id: String,
    payload: WebauthnRegFinishRequest,
) -> Result<(), ErrorResponse> {
    let mut user = User::find(id).await?;

    let idx = format!("reg_{:?}_{}", payload.passkey_name, user.id);
    let client = DB::hql();

    let res: Option<WebauthnReg> = client.get(Cache::Webauthn, &idx).await?;
    if res.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Webauthn Registration Request not found",
        ));
    }

    client.delete(Cache::Webauthn, idx).await?;
    let reg_data = res.unwrap();

    let reg_state = serde_json::from_str::<PasskeyRegistration>(&reg_data.reg_state)?;
    match RauthyConfig::get()
        .webauthn
        .finish_passkey_registration(&payload.data, &reg_state)
    {
        Ok(pk) => {
            // force UV check
            let cfg = &RauthyConfig::get().vars.webauthn;
            let cred = Credential::from(pk.clone());
            if (user.account_type() != AccountType::Password || cfg.force_uv) && !cred.user_verified
            {
                warn!(
                    user.id,
                    "Webauthn Registration Ceremony without User Verification",
                );
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User Presence only is not allowed - Verification is needed",
                ));
            }

            let user_id = user.id.clone();
            let create_user = if user.webauthn_user_id.is_none() {
                user.webauthn_user_id = Some(reg_data.passkey_user_id.to_string());
                if user.password.is_none() || cfg.no_password_exp {
                    user.password_expires = None;
                }
                Some(user)
            } else {
                None
            };

            PasskeyEntity::create(
                user_id.clone(),
                create_user,
                reg_data.passkey_user_id,
                payload.passkey_name,
                pk,
                cred.user_verified,
            )
            .await?;

            info!(user_id, "New PasskeyEntity saved successfully");
        }
        Err(err) => {
            error!(?err, "Webauthn Reg Finish");
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("{err}"),
            ));
        }
    };

    Ok(())
}
