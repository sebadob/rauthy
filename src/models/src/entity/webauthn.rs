use crate::api_cookie::ApiCookie;
use crate::app_state::{AppState, DbTxn};
use crate::database::{Cache, DB};
use crate::entity::password::PasswordPolicy;
use crate::entity::users::{AccountType, User};
use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use cryptr::EncValue;
use hiqlite::{params, Param, Params};
use rauthy_api_types::users::{
    MfaPurpose, PasskeyResponse, WebauthnAuthFinishRequest, WebauthnAuthStartResponse,
    WebauthnLoginFinishResponse, WebauthnRegFinishRequest, WebauthnRegStartRequest,
};
use rauthy_common::constants::{
    CACHE_TTL_WEBAUTHN, CACHE_TTL_WEBAUTHN_DATA, COOKIE_MFA, IDX_WEBAUTHN, WEBAUTHN_FORCE_UV,
    WEBAUTHN_NO_PASSWORD_EXPIRY, WEBAUTHN_RENEW_EXP, WEBAUTHN_REQ_EXP,
};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::base64_decode;
use rauthy_common::utils::{base64_encode, get_rand};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::ops::Add;
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::{error, info, warn};
use utoipa::ToSchema;
use webauthn_rs::prelude::*;
use webauthn_rs_proto::{
    AuthenticatorSelectionCriteria, ResidentKeyRequirement, UserVerificationPolicy,
};

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
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
        let client = DB::client();

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(2);

            if let Some(user) = user {
                debug_assert!(user.webauthn_user_id.is_some());
                user.save_txn_append(&mut txn);
            }

            txn.push((
                r#"
INSERT INTO passkeys
(user_id, name, passkey_user_id, passkey, credential_id, registered, last_used, user_verified)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
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
            let mut txn = DB::txn().await?;

            if let Some(user) = user {
                debug_assert!(user.webauthn_user_id.is_some());
                user.save_txn(&mut txn).await?;
            }

            sqlx::query!(
                r#"
INSERT INTO passkeys
(user_id, name, passkey_user_id, passkey, credential_id, registered, last_used, user_verified)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
                entity.user_id,
                entity.name,
                entity.passkey_user_id,
                entity.passkey,
                entity.credential_id,
                now,
                now,
                entity.user_verified,
            )
            .execute(&mut *txn)
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
        let count: i64 = if is_hiqlite() {
            DB::client()
                .query_raw_one(
                    "SELECT COUNT (*) AS count FROM passkeys WHERE user_id = $1",
                    params!(user_id),
                )
                .await?
                .get("count")
        } else {
            sqlx::query!(
                "SELECT COUNT (*) AS count FROM passkeys WHERE user_id = $1",
                user_id
            )
            .fetch_one(DB::conn())
            .await?
            .count
            .unwrap_or_default()
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

            DB::client().txn(txn).await?;
        } else {
            let mut txn = DB::txn().await?;

            if let Some(user) = user_to_save {
                user.save_txn(&mut txn).await?;
            }
            Self::delete_by_id_name(&user_id, &name, &mut txn).await?;

            txn.commit().await?;
        }

        Self::clear_caches_by_id_name(&user_id, user_email, &name).await?;

        Ok(())
    }

    /// MUST call `PasskeyEntity::clear_caches_by_id_name()` after txn commit!
    async fn delete_by_id_name(
        user_id: &str,
        name: &str,
        txn: &mut DbTxn<'_>,
    ) -> Result<(), ErrorResponse> {
        sqlx::query!(
            "DELETE FROM passkeys WHERE user_id = $1 AND name = $2",
            user_id,
            name
        )
        .execute(&mut **txn)
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
        let client = DB::client();

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
        let client = DB::client();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let slf = if is_hiqlite() {
            client
                .query_as_one(
                    "SELECT * FROM passkeys WHERE user_id = $1 AND name = $2",
                    params!(user_id, name),
                )
                .await?
        } else {
            sqlx::query_as!(
                Self,
                "SELECT * FROM passkeys WHERE user_id = $1 AND name = $2",
                user_id,
                name,
            )
            .fetch_one(DB::conn())
            .await?
        };

        client
            .put(Cache::Webauthn, idx, &slf, *CACHE_TTL_WEBAUTHN)
            .await?;

        Ok(slf)
    }

    pub async fn find_cred_ids_for_user(user_id: &str) -> Result<Vec<CredentialID>, ErrorResponse> {
        let idx = Self::cache_idx_creds(user_id);
        let client = DB::client();

        let opt: Option<Vec<Vec<u8>>> = client.get(Cache::Webauthn, &idx).await?;
        if let Some(bytes) = opt {
            return Ok(bytes.into_iter().map(CredentialID::from).collect());
        }

        let creds = if is_hiqlite() {
            client
                .query_raw(
                    "SELECT credential_id FROM passkeys WHERE user_id = $1",
                    params!(user_id),
                )
                .await?
                .into_iter()
                .map(|mut r| r.get::<Vec<u8>>("credential_id"))
                .collect::<Vec<_>>()
        } else {
            sqlx::query!(
                "SELECT credential_id FROM passkeys WHERE user_id = $1",
                user_id
            )
            .fetch_all(DB::conn())
            .await?
            .into_iter()
            .map(|row| row.credential_id)
            .collect::<Vec<Vec<u8>>>()
        };

        client
            .put(Cache::Webauthn, idx, &creds, *CACHE_TTL_WEBAUTHN)
            .await?;

        Ok(creds.into_iter().map(CredentialID::from).collect())
    }

    pub async fn find_for_user(user_id: &str) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx_user(user_id);
        let client = DB::client();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let pks = if is_hiqlite() {
            client
                .query_as(
                    "SELECT * FROM passkeys WHERE user_id = $1",
                    params!(user_id),
                )
                .await?
        } else {
            sqlx::query_as!(Self, "SELECT * FROM passkeys WHERE user_id = $1", user_id)
                .fetch_all(DB::conn())
                .await?
        };

        client
            .put(Cache::Webauthn, idx, &pks, *CACHE_TTL_WEBAUTHN)
            .await?;

        Ok(pks)
    }

    pub async fn find_for_user_with_uv(user_id: &str) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx_user_with_uv(user_id);
        let client = DB::client();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let pks = if is_hiqlite() {
            client
                .query_as(
                    "SELECT * FROM passkeys WHERE user_id = $1 AND user_verified = true",
                    params!(user_id),
                )
                .await?
        } else {
            sqlx::query_as!(
                Self,
                "SELECT * FROM passkeys WHERE user_id = $1 AND user_verified = true",
                user_id
            )
            .fetch_all(DB::conn())
            .await?
        };

        client
            .put(Cache::Webauthn, idx, &pks, *CACHE_TTL_WEBAUTHN)
            .await?;

        Ok(pks)
    }

    //     pub async fn update_passkey(&self, txn: &mut DbTxn<'_>) -> Result<(), ErrorResponse> {
    //         let client = DB::client();
    //
    //         if is_hiqlite() {
    //             client
    //                 .execute(
    //                     r#"
    // UPDATE passkeys
    // SET passkey = $1, last_used = $2
    // WHERE user_id = $3 AND name = $4"#,
    //                     params!(&self.passkey, self.last_used, &self.user_id, &self.name),
    //                 )
    //                 .await?;
    //         } else {
    //             sqlx::query!(
    //                 r#"
    // UPDATE passkeys
    // SET passkey = $1, last_used = $2
    // WHERE user_id = $3 AND name = $4"#,
    //                 self.passkey,
    //                 self.last_used,
    //                 self.user_id,
    //                 self.name,
    //             )
    //             .execute(&mut **txn)
    //             .await?;
    //         }
    //
    //         client
    //             .put(
    //                 Cache::Webauthn,
    //                 Self::cache_idx_single(&self.user_id, &self.name),
    //                 self,
    //                 *CACHE_TTL_WEBAUTHN,
    //             )
    //             .await?;
    //
    //         client
    //             .delete(Cache::Webauthn, Self::cache_idx_user(&self.user_id))
    //             .await?;
    //         client
    //             .delete(Cache::Webauthn, Self::cache_idx_user_with_uv(&self.user_id))
    //             .await?;
    //
    //         Ok(())
    //     }

    pub fn update_passkey_txn_append(&self, txn: &mut Vec<(&str, Params)>) {
        txn.push((
            r#"
UPDATE passkeys
SET passkey = $1, last_used = $2
WHERE user_id = $3 AND name = $4"#,
            params!(&self.passkey, self.last_used, &self.user_id, &self.name),
        ));
    }

    pub async fn update_passkey_txn(&self, txn: &mut DbTxn<'_>) -> Result<(), ErrorResponse> {
        sqlx::query!(
            r#"
UPDATE passkeys
SET passkey = $1, last_used = $2
WHERE user_id = $3 AND name = $4"#,
            self.passkey,
            self.last_used,
            self.user_id,
            self.name,
        )
        .execute(&mut **txn)
        .await?;

        Ok(())
    }

    async fn update_caches_after_update(&self) -> Result<(), ErrorResponse> {
        let client = DB::client();

        client
            .put(
                Cache::Webauthn,
                Self::cache_idx_single(&self.user_id, &self.name),
                self,
                *CACHE_TTL_WEBAUTHN,
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
        format!("{}{}{}", IDX_WEBAUTHN, user_id, name)
    }

    /// Index for all passkeys a user has
    #[inline]
    fn cache_idx_user(user_id: &str) -> String {
        format!("{}{}", IDX_WEBAUTHN, user_id)
    }

    /// Index for all passkeys a user has
    #[inline]
    fn cache_idx_user_with_uv(user_id: &str) -> String {
        format!("{}_UV_{}", IDX_WEBAUTHN, user_id)
    }

    /// Index for credentials for a user
    #[inline]
    fn cache_idx_creds(user_id: &str) -> String {
        format!("{}{}_creds", IDX_WEBAUTHN, user_id)
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

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct WebauthnCookie {
    pub email: String,
    pub exp: OffsetDateTime,
}

impl WebauthnCookie {
    pub fn new(email: String) -> Self {
        let exp = OffsetDateTime::now_utc().add(::time::Duration::hours(*WEBAUTHN_RENEW_EXP));
        Self { email, exp }
    }

    pub fn build(&self) -> Result<Cookie, ErrorResponse> {
        let ser = bincode::serialize(self)?;
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
        let slf = bincode::deserialize::<Self>(&dec)?;

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
        DB::client()
            .delete(Cache::Webauthn, self.code.clone())
            .await?;
        Ok(())
    }

    pub async fn find(code: String) -> Result<Self, ErrorResponse> {
        let res: Option<Self> = DB::client().get(Cache::Webauthn, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Webauthn Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        DB::client()
            .put(
                Cache::Webauthn,
                self.code.clone(),
                &self,
                *CACHE_TTL_WEBAUTHN_DATA,
            )
            .await?;
        Ok(())
    }
}

// This is the data, that will be passed to the client as response to a successful MFA auth
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum WebauthnAdditionalData {
    Login(WebauthnLoginReq),
    // the String inside the Service(_) is always the corresponding user id
    Service(WebauthnServiceReq),
    Test,
}

impl WebauthnAdditionalData {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        match self {
            Self::Login(d) => d.delete().await,
            Self::Service(_uid) => Ok(()),
            Self::Test => Ok(()),
        }
    }

    pub fn into_response(self) -> HttpResponse {
        match self {
            Self::Login(login_req) => {
                let header_loc = (
                    header::LOCATION,
                    HeaderValue::from_str(&login_req.header_loc).unwrap(),
                );
                let body = WebauthnLoginFinishResponse {
                    loc: login_req.header_loc,
                };
                let mut res = HttpResponse::Accepted()
                    .insert_header(header_loc)
                    .json(body);
                if let Some(value) = login_req.header_origin {
                    res.headers_mut().insert(
                        header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        HeaderValue::from_str(&value).unwrap(),
                    );
                }
                res
            }

            Self::Service(svc_req) => HttpResponse::Accepted().json(svc_req),

            Self::Test => HttpResponse::Accepted().finish(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct WebauthnLoginReq {
    pub code: String,
    pub user_id: String,
    pub header_loc: String,
    pub header_origin: Option<String>,
}

// CRUD
impl WebauthnLoginReq {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::client()
            .delete(Cache::Webauthn, self.code.clone())
            .await?;
        Ok(())
    }

    pub async fn find(code: String) -> Result<Self, ErrorResponse> {
        let res: Option<Self> = DB::client().get(Cache::Webauthn, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Webauthn Login Request Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        DB::client()
            .put(
                Cache::Webauthn,
                self.code.clone(),
                self,
                *CACHE_TTL_WEBAUTHN_DATA,
            )
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
        DB::client()
            .delete(Cache::Webauthn, self.code.clone())
            .await?;
        Ok(())
    }

    pub async fn find(code: String) -> Result<Self, ErrorResponse> {
        let res = DB::client().get(Cache::Webauthn, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Webauthn Service Request Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        DB::client()
            .put(
                Cache::Webauthn,
                self.code.clone(),
                self,
                *CACHE_TTL_WEBAUTHN_DATA,
            )
            .await?;
        Ok(())
    }
}

pub async fn auth_start(
    data: &web::Data<AppState>,
    user_id: String,
    purpose: MfaPurpose,
) -> Result<WebauthnAuthStartResponse, ErrorResponse> {
    // This app_data will be returned to the client upon successful webauthn authentication
    let add_data = match purpose {
        MfaPurpose::Login(code) => {
            let d = WebauthnLoginReq::find(code).await?;
            WebauthnAdditionalData::Login(d)
        }
        MfaPurpose::PasswordNew | MfaPurpose::PasswordReset => {
            let svc_req = WebauthnServiceReq::new(user_id.clone());
            svc_req.save().await?;
            WebauthnAdditionalData::Service(svc_req)
        }
        MfaPurpose::Test => WebauthnAdditionalData::Test,
    };

    let user = User::find(user_id).await?;
    let force_uv = user.account_type() == AccountType::Passkey || *WEBAUTHN_FORCE_UV;
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

    match data.webauthn.start_passkey_authentication(pks.as_slice()) {
        Ok((mut rcr, auth_state)) => {
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
                exp: *WEBAUTHN_REQ_EXP,
            })
        }

        Err(err) => {
            error!("Webauthn challenge authentication: {:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Internal error with Webauthn Challenge Authentication",
            ))
        }
    }
}

pub async fn auth_finish(
    data: &web::Data<AppState>,
    user_id: String,
    req: WebauthnAuthFinishRequest,
) -> Result<WebauthnAdditionalData, ErrorResponse> {
    let auth_data = WebauthnData::find(req.code).await?;
    let auth_state = serde_json::from_str(&auth_data.auth_state_json)?;

    let mut user = User::find(user_id).await?;
    let force_uv = user.account_type() == AccountType::Passkey || *WEBAUTHN_FORCE_UV;

    let pks = PasskeyEntity::find_for_user(&user.id).await?;

    match data
        .webauthn
        .finish_passkey_authentication(&req.data, &auth_state)
    {
        Ok(auth_result) => {
            if force_uv && !auth_result.user_verified() {
                warn!(
                    "Webauthn Authentication Ceremony without User Verification for user {:?}",
                    user.id
                );
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User Presence only is not allowed - Verification is needed",
                ));
            }
            let uid = user.id.clone();

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
                            DB::client().txn(txn).await?;
                        } else {
                            let mut txn = DB::txn().await?;
                            pk_entity.update_passkey_txn(&mut txn).await?;
                            user.save_txn(&mut txn).await?;
                            txn.commit().await?;
                        }

                        pk_entity.update_caches_after_update().await?;

                        // there can only be exactly one key that needs the update
                        break;
                    }
                }
            }

            info!("Webauthn Authentication successful for user {}", uid);

            Ok(auth_data.data)
        }
        Err(err) => {
            error!("Webauthn Auth Finish: {:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                format!("{err}"),
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
    data: &web::Data<AppState>,
    user_id: String,
    req: WebauthnRegStartRequest,
) -> Result<CreationChallengeResponse, ErrorResponse> {
    let user = User::find(user_id).await?;
    let passkey_user_id = if let Some(id) = &user.webauthn_user_id {
        Uuid::from_str(id).expect("corrupted database: user.webauthn_user_id")
    } else {
        Uuid::new_v4()
    };
    let cred_ids = PasskeyEntity::find_cred_ids_for_user(&user.id).await?;

    match data.webauthn.start_passkey_registration(
        passkey_user_id,
        &user.email,
        &user.email,
        Some(cred_ids),
    ) {
        Ok((mut ccr, reg_state)) => {
            if *WEBAUTHN_FORCE_UV || user.account_type() == AccountType::Passkey {
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
            let idx = format!("reg_{:?}_{}", req.passkey_name, user.id);
            DB::client()
                .put(Cache::Webauthn, idx, &reg_data, *CACHE_TTL_WEBAUTHN)
                .await?;

            Ok(ccr)
        }

        Err(err) => {
            error!("Webauthn challenge register: {:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Internal error with Webauthn Challenge Registration",
            ))
        }
    }
}

pub async fn reg_finish(
    data: &web::Data<AppState>,
    id: String,
    req: WebauthnRegFinishRequest,
) -> Result<(), ErrorResponse> {
    let mut user = User::find(id).await?;

    let idx = format!("reg_{:?}_{}", req.passkey_name, user.id);
    let client = DB::client();

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
    match data
        .webauthn
        .finish_passkey_registration(&req.data, &reg_state)
    {
        Ok(pk) => {
            // force UV check
            let cred = Credential::from(pk.clone());
            if (user.account_type() != AccountType::Password || *WEBAUTHN_FORCE_UV)
                && !cred.user_verified
            {
                warn!(
                    "Webauthn Registration Ceremony without User Verification for user {:?}",
                    user.id
                );
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User Presence only is not allowed - Verification is needed",
                ));
            }

            let user_id = user.id.clone();
            let create_user = if user.webauthn_user_id.is_none() {
                user.webauthn_user_id = Some(reg_data.passkey_user_id.to_string());
                if user.password.is_none() || *WEBAUTHN_NO_PASSWORD_EXPIRY {
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
                req.passkey_name,
                pk,
                cred.user_verified,
            )
            .await?;

            info!("New PasskeyEntity saved successfully for user {}", user_id);
        }
        Err(err) => {
            error!("Webauthn Reg Finish: {:?}", err);
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("{err}"),
            ));
        }
    };

    Ok(())
}
