use crate::api_cookie::ApiCookie;
use crate::app_state::{AppState, DbTxn};
use crate::cache::{Cache, DB};
use crate::entity::users::{AccountType, User};
use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use cryptr::EncValue;
use rauthy_api_types::users::{
    MfaPurpose, PasskeyResponse, WebauthnAuthFinishRequest, WebauthnAuthStartResponse,
    WebauthnLoginFinishResponse, WebauthnRegFinishRequest, WebauthnRegStartRequest,
};
use rauthy_common::constants::{
    CACHE_TTL_WEBAUTHN, CACHE_TTL_WEBAUTHN_DATA, COOKIE_MFA, IDX_WEBAUTHN, WEBAUTHN_FORCE_UV,
    WEBAUTHN_NO_PASSWORD_EXPIRY, WEBAUTHN_RENEW_EXP, WEBAUTHN_REQ_EXP,
};
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
    pub async fn create(
        user_id: String,
        passkey_user_id: Uuid,
        name: String,
        pk: Passkey,
        user_verified: bool,
        txn: &mut DbTxn<'_>,
    ) -> Result<(), ErrorResponse> {
        // json, because bincode does not support deserialize from any, which would be the case here
        let passkey = serde_json::to_string(&pk)?;
        let now = OffsetDateTime::now_utc().unix_timestamp();

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

        sqlx::query!(
            r#"INSERT INTO passkeys
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
        .execute(&mut **txn)
        .await?;

        let client = DB::client();
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

    pub async fn delete(
        &self,
        data: &web::Data<AppState>,
        txn: Option<&mut DbTxn<'_>>,
    ) -> Result<(), ErrorResponse> {
        Self::delete_by_id_name(data, &self.user_id, &self.name, txn).await
    }

    pub async fn delete_by_id_name(
        data: &web::Data<AppState>,
        user_id: &str,
        name: &str,
        txn: Option<&mut DbTxn<'_>>,
    ) -> Result<(), ErrorResponse> {
        let q = sqlx::query!(
            "DELETE FROM passkeys WHERE user_id = $1 AND name = $2",
            user_id,
            name
        );

        if let Some(txn) = txn {
            q.execute(&mut **txn).await?;
        } else {
            q.execute(&data.db).await?;
        }

        let client = DB::client();
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

    pub async fn find(
        data: &web::Data<AppState>,
        user_id: &str,
        name: &str,
    ) -> Result<Self, ErrorResponse> {
        let idx = Self::cache_idx_single(user_id, name);
        let client = DB::client();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let pk = sqlx::query_as!(
            Self,
            "SELECT * FROM passkeys WHERE user_id = $1 AND name = $2",
            user_id,
            name,
        )
        .fetch_one(&data.db)
        .await?;

        client
            .put(Cache::Webauthn, idx, &pk, *CACHE_TTL_WEBAUTHN)
            .await?;

        Ok(pk)
    }

    pub async fn find_cred_ids_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<Vec<CredentialID>, ErrorResponse> {
        let idx = Self::cache_idx_creds(user_id);
        let client = DB::client();

        let opt: Option<Vec<Vec<u8>>> = client.get(Cache::Webauthn, &idx).await?;
        if let Some(bytes) = opt {
            return Ok(bytes.into_iter().map(CredentialID::from).collect());
        }

        let creds = sqlx::query!(
            "SELECT credential_id FROM passkeys WHERE user_id = $1",
            user_id
        )
        .fetch_all(&data.db)
        .await?
        .into_iter()
        .map(|row| row.credential_id)
        .collect::<Vec<Vec<u8>>>();

        client
            .put(Cache::Webauthn, idx, &creds, *CACHE_TTL_WEBAUTHN)
            .await?;

        Ok(creds.into_iter().map(CredentialID::from).collect())
    }

    pub async fn find_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx_user(user_id);
        let client = DB::client();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let pks = sqlx::query_as!(Self, "SELECT * FROM passkeys WHERE user_id = $1", user_id)
            .fetch_all(&data.db)
            .await?;

        client
            .put(Cache::Webauthn, idx, &pks, *CACHE_TTL_WEBAUTHN)
            .await?;

        Ok(pks)
    }

    pub async fn find_for_user_with_uv(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx_user_with_uv(user_id);
        let client = DB::client();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let pks = sqlx::query_as!(
            Self,
            "SELECT * FROM passkeys WHERE user_id = $1 AND user_verified = true",
            user_id
        )
        .fetch_all(&data.db)
        .await?;

        client
            .put(Cache::Webauthn, idx, &pks, *CACHE_TTL_WEBAUTHN)
            .await?;

        Ok(pks)
    }

    pub async fn update_passkey(&self, txn: &mut DbTxn<'_>) -> Result<(), ErrorResponse> {
        sqlx::query!(
            r#"UPDATE passkeys
            SET passkey = $1, last_used = $2
            WHERE user_id = $3 AND name = $4"#,
            self.passkey,
            self.last_used,
            self.user_id,
            self.name,
        )
        .execute(&mut **txn)
        .await?;

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

    let user = User::find(data, user_id).await?;
    let force_uv = user.account_type() == AccountType::Passkey || *WEBAUTHN_FORCE_UV;
    let pks = if force_uv {
        // in this case, filter out all presence only keys
        PasskeyEntity::find_for_user_with_uv(data, &user.id)
            .await?
            .iter()
            .map(|pk_entity| pk_entity.get_pk())
            .collect::<Vec<Passkey>>()
    } else {
        PasskeyEntity::find_for_user(data, &user.id)
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

    let mut user = User::find(data, user_id).await?;
    let force_uv = user.account_type() == AccountType::Passkey || *WEBAUTHN_FORCE_UV;

    let pks = PasskeyEntity::find_for_user(data, &user.id).await?;

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

                    let mut txn = data.db.begin().await?;
                    pk_entity.update_passkey(&mut txn).await?;
                    user.save(data, None, Some(&mut txn)).await?;
                    txn.commit().await?;
                }
            }

            info!("Webauthn Authentication successful for user {}", user.id);

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
    let user = User::find(data, user_id).await?;
    let passkey_user_id = if let Some(id) = &user.webauthn_user_id {
        Uuid::from_str(id).expect("corrupted database: user.webauthn_user_id")
    } else {
        Uuid::new_v4()
    };
    let cred_ids = PasskeyEntity::find_cred_ids_for_user(data, &user.id).await?;

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
    let mut user = User::find(data, id).await?;

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

            let mut txn = data.db.begin().await?;

            if user.webauthn_user_id.is_none() {
                user.webauthn_user_id = Some(reg_data.passkey_user_id.to_string());
                if user.password.is_none() || *WEBAUTHN_NO_PASSWORD_EXPIRY {
                    user.password_expires = None;
                }
                user.save(data, None, Some(&mut txn)).await?;
            }

            PasskeyEntity::create(
                user.id.clone(),
                reg_data.passkey_user_id,
                req.passkey_name,
                pk,
                cred.user_verified,
                &mut txn,
            )
            .await?;
            txn.commit().await?;

            info!("New PasskeyEntity saved successfully for user {}", user.id);
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
