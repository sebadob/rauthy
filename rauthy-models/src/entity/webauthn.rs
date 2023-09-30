use crate::app_state::{AppState, DbTxn};
use crate::entity::users::{AccountType, User};
use crate::request::{
    MfaPurpose, WebauthnAuthFinishRequest, WebauthnRegFinishRequest, WebauthnRegStartRequest,
};
use crate::response::{WebauthnAuthStartResponse, WebauthnLoginFinishResponse};
use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::{cookie, web, HttpResponse};
use rauthy_common::constants::{
    CACHE_NAME_WEBAUTHN, CACHE_NAME_WEBAUTHN_DATA, COOKIE_MFA, IDX_WEBAUTHN, WEBAUTHN_FORCE_UV,
    WEBAUTHN_NO_PASSWORD_EXPIRY, WEBAUTHN_RENEW_EXP, WEBAUTHN_REQ_EXP,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::{base64_decode, decrypt};
use rauthy_common::utils::{base64_encode, encrypt, get_rand};
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, cache_remove, AckLevel};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::{error, info, warn};
use utoipa::ToSchema;
use webauthn_rs::prelude::*;
use webauthn_rs_proto::{AuthenticatorSelectionCriteria, UserVerificationPolicy};

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
        data: &web::Data<AppState>,
        user_id: String,
        passkey_user_id: Uuid,
        name: String,
        pk: Passkey,
        user_verified: bool,
        txn: &mut DbTxn<'_>,
    ) -> Result<(), ErrorResponse> {
        // json, because bincode does not support deserialize from any, which would be the case here
        let passkey = serde_json::to_string(&pk).unwrap();
        let now = OffsetDateTime::now_utc().unix_timestamp();

        let entity = Self {
            user_id,
            name,
            passkey_user_id: passkey_user_id.to_string(),
            passkey,
            credential_id: pk.cred_id().0.clone(),
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

        cache_remove(
            CACHE_NAME_WEBAUTHN.to_string(),
            Self::cache_idx_user(&entity.user_id),
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;
        cache_remove(
            CACHE_NAME_WEBAUTHN.to_string(),
            Self::cache_idx_creds(&entity.user_id),
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
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

        cache_remove(
            CACHE_NAME_WEBAUTHN.to_string(),
            Self::cache_idx_single(user_id, name),
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;
        cache_remove(
            CACHE_NAME_WEBAUTHN.to_string(),
            Self::cache_idx_user(user_id),
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;
        cache_remove(
            CACHE_NAME_WEBAUTHN.to_string(),
            Self::cache_idx_creds(user_id),
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;

        Ok(())
    }

    pub async fn find(
        data: &web::Data<AppState>,
        user_id: &str,
        name: &str,
    ) -> Result<Self, ErrorResponse> {
        let idx = Self::cache_idx_single(user_id, name);
        let pk = cache_get!(
            PasskeyEntity,
            CACHE_NAME_WEBAUTHN.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if pk.is_some() {
            return Ok(pk.unwrap());
        }

        let pk = sqlx::query_as!(
            Self,
            "SELECT * FROM passkeys WHERE user_id = $1 AND name = $2",
            user_id,
            name,
        )
        .fetch_one(&data.db)
        .await?;

        cache_insert(
            CACHE_NAME_WEBAUTHN.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &pk,
            AckLevel::Leader,
        )
        .await?;

        Ok(pk)
    }

    pub async fn find_cred_ids_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<Vec<CredentialID>, ErrorResponse> {
        let idx = Self::cache_idx_creds(user_id);
        let pk = cache_get!(
            Vec<Vec<u8>>,
            CACHE_NAME_WEBAUTHN.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if pk.is_some() {
            return Ok(pk.unwrap().into_iter().map(CredentialID::from).collect());
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

        cache_insert(
            CACHE_NAME_WEBAUTHN.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &creds,
            AckLevel::Leader,
        )
        .await?;

        Ok(creds.into_iter().map(CredentialID::from).collect())
    }

    pub async fn find_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx_user(user_id);
        let pk = cache_get!(
            Vec<PasskeyEntity>,
            CACHE_NAME_WEBAUTHN.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if pk.is_some() {
            return Ok(pk.unwrap());
        }

        let pks = sqlx::query_as!(Self, "SELECT * FROM passkeys WHERE user_id = $1", user_id)
            .fetch_all(&data.db)
            .await?;

        cache_insert(
            CACHE_NAME_WEBAUTHN.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &pks,
            AckLevel::Leader,
        )
        .await?;

        Ok(pks)
    }

    pub async fn find_for_user_with_uv(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx_user(user_id);
        let pk = cache_get!(
            Vec<PasskeyEntity>,
            CACHE_NAME_WEBAUTHN.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if pk.is_some() {
            return Ok(pk.unwrap());
        }

        let pks = sqlx::query_as!(
            Self,
            "SELECT * FROM passkeys WHERE user_id = $1 AND user_verified = true",
            user_id
        )
        .fetch_all(&data.db)
        .await?;

        cache_insert(
            CACHE_NAME_WEBAUTHN.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &pks,
            AckLevel::Leader,
        )
        .await?;

        Ok(pks)
    }

    pub async fn update_passkey(
        &self,
        data: &web::Data<AppState>,
        txn: &mut DbTxn<'_>,
    ) -> Result<(), ErrorResponse> {
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

        cache_insert(
            CACHE_NAME_WEBAUTHN.to_string(),
            Self::cache_idx_single(&self.user_id, &self.name),
            &data.caches.ha_cache_config,
            &self,
            AckLevel::Quorum,
        )
        .await?;

        // TODO instead of invalidating, we could update in advance
        cache_remove(
            CACHE_NAME_WEBAUTHN.to_string(),
            Self::cache_idx_user(&self.user_id),
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;

        Ok(())
    }
}

impl PasskeyEntity {
    pub fn get_pk(&self) -> Passkey {
        // Passkeys cannot be serialized with bincode -> no support for deserialize from any
        serde_json::from_str(&self.passkey).unwrap()
    }

    fn cache_idx_single(user_id: &str, name: &str) -> String {
        format!("{}{}{}", IDX_WEBAUTHN, user_id, name)
    }

    fn cache_idx_user(user_id: &str) -> String {
        format!("{}{}", IDX_WEBAUTHN, user_id)
    }

    fn cache_idx_creds(user_id: &str) -> String {
        format!("{}{}_creds", IDX_WEBAUTHN, user_id)
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

    pub fn build(&self, key_id: &str, enc_key: &[u8]) -> Result<Cookie, ErrorResponse> {
        let ser = bincode::serialize(self)?;
        let enc = encrypt(&ser, enc_key)?;
        let b64 = base64_encode(&enc);
        let value = format!("{}/{}", key_id, b64);

        let cookie_exp = cookie::Expiration::from(self.exp);
        Ok(Cookie::build(COOKIE_MFA, value)
            .http_only(true)
            .secure(true)
            .same_site(cookie::SameSite::Lax)
            .expires(cookie_exp)
            .path("/auth")
            .finish())
    }

    pub fn parse_validate(
        cookie: &Option<Cookie>,
        enc_keys: &HashMap<String, Vec<u8>>,
    ) -> Result<Self, ErrorResponse> {
        if cookie.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Webauthn Cookie is missing".to_string(),
            ));
        }
        let cookie = cookie.as_ref().unwrap();

        let (key_id, b64) = match cookie.value().split_once('/') {
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Webauthn Cookie is in a bad format".to_string(),
                ));
            }
            Some(s) => s,
        };

        let key = match enc_keys.get(key_id) {
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Webauthn Cookie key ID does not exist".to_string(),
                ));
            }
            Some(id) => id,
        };

        let b64 = base64_decode(b64)?;
        let dec = decrypt(&b64, key.as_ref())?;
        let slf = bincode::deserialize::<Self>(&dec)?;

        if slf.exp < OffsetDateTime::now_utc() {
            Err(ErrorResponse::new(
                ErrorResponseType::SessionExpired,
                "Webauthn Cookie has expired".to_string(),
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
    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_remove(
            CACHE_NAME_WEBAUTHN_DATA.to_string(),
            self.code.clone(),
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;
        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, code: String) -> Result<Self, ErrorResponse> {
        let res = cache_get!(
            WebauthnData,
            CACHE_NAME_WEBAUTHN_DATA.to_string(),
            code,
            &data.caches.ha_cache_config,
            false
        )
        .await?;

        if res.is_none() {
            Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Webauthn Data not found".to_string(),
            ))
        } else {
            Ok(res.unwrap())
        }
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_insert(
            CACHE_NAME_WEBAUTHN_DATA.to_string(),
            self.code.clone(),
            &data.caches.ha_cache_config,
            &self,
            AckLevel::Quorum,
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
    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        match self {
            Self::Login(d) => d.delete(data).await,
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
    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_remove(
            CACHE_NAME_WEBAUTHN_DATA.to_string(),
            self.code.clone(),
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;
        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, code: String) -> Result<Self, ErrorResponse> {
        let res = cache_get!(
            WebauthnLoginReq,
            CACHE_NAME_WEBAUTHN_DATA.to_string(),
            code,
            &data.caches.ha_cache_config,
            false
        )
        .await?;

        if res.is_none() {
            Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Webauthn Login Request Data not found".to_string(),
            ))
        } else {
            Ok(res.unwrap())
        }
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_insert(
            CACHE_NAME_WEBAUTHN_DATA.to_string(),
            self.code.clone(),
            &data.caches.ha_cache_config,
            &self,
            AckLevel::Quorum,
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

    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_remove(
            CACHE_NAME_WEBAUTHN_DATA.to_string(),
            self.code.clone(),
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;
        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, code: String) -> Result<Self, ErrorResponse> {
        let res = cache_get!(
            Self,
            CACHE_NAME_WEBAUTHN_DATA.to_string(),
            code,
            &data.caches.ha_cache_config,
            false
        )
        .await?;

        if res.is_none() {
            Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Webauthn Service Request Data not found".to_string(),
            ))
        } else {
            Ok(res.unwrap())
        }
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_insert(
            CACHE_NAME_WEBAUTHN_DATA.to_string(),
            self.code.clone(),
            &data.caches.ha_cache_config,
            &self,
            AckLevel::Quorum,
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
            let d = WebauthnLoginReq::find(data, code).await?;
            WebauthnAdditionalData::Login(d)
        }
        MfaPurpose::PasswordReset => {
            let svc_req = WebauthnServiceReq::new(user_id.clone());
            svc_req.save(data).await?;
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
            "No Security Keys with active user verification found".to_string(),
        ));
    }

    match data.webauthn.start_passkey_authentication(pks.as_slice()) {
        Ok((mut rcr, auth_state)) => {
            if force_uv {
                rcr.public_key.user_verification = UserVerificationPolicy::Required;
            }
            add_data.delete(data).await?;

            // cannot be serialized with bincode -> no deserialize from any
            let auth_state_json = serde_json::to_string(&auth_state).unwrap();
            let auth_data = WebauthnData {
                code: get_rand(48),
                auth_state_json,
                data: add_data,
            };
            auth_data.save(data).await?;

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
                "Internal error with Webauthn Challenge Authentication".to_string(),
            ))
        }
    }
}

pub async fn auth_finish(
    data: &web::Data<AppState>,
    user_id: String,
    req: WebauthnAuthFinishRequest,
) -> Result<WebauthnAdditionalData, ErrorResponse> {
    let mut user = User::find(data, user_id).await?;
    let force_uv = user.account_type() == AccountType::Passkey || *WEBAUTHN_FORCE_UV;

    let auth_data = WebauthnData::find(data, req.code).await?;
    let auth_state = serde_json::from_str(&auth_data.auth_state_json).unwrap();

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
                    "User Presence only is not allowed - Verification is needed".to_string(),
                ));
            }

            for mut pk_entity in pks {
                let mut pk = pk_entity.get_pk();
                if let Some(updated) = pk.update_credential(&auth_result) {
                    if updated {
                        pk_entity.passkey = serde_json::to_string(&pk).unwrap();
                    }

                    let now = OffsetDateTime::now_utc().unix_timestamp();
                    pk_entity.last_used = now;
                    user.last_login = Some(now);
                    user.last_failed_login = None;
                    user.failed_login_attempts = None;

                    let mut txn = data.db.begin().await?;
                    pk_entity.update_passkey(data, &mut txn).await?;
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
                            require_resident_key: false,
                            user_verification: UserVerificationPolicy::Required,
                        })
                    };
            };

            let reg_data = WebauthnReg {
                user_id: user.id.clone(),
                passkey_user_id,
                // the reg_state cannot be serialized with bincode -> missing deserialize from Any
                reg_state: serde_json::to_string(&reg_state).unwrap(),
            };

            // persist the reg_state
            let idx = format!("reg_{:?}_{}", req.passkey_name, user.id);
            cache_insert(
                CACHE_NAME_WEBAUTHN.to_string(),
                idx,
                &data.caches.ha_cache_config,
                &reg_data,
                AckLevel::Quorum,
            )
            .await?;

            Ok(ccr)
        }

        Err(err) => {
            error!("Webauthn challenge register: {:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Internal error with Webauthn Challenge Registration".to_string(),
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
    let res = cache_get!(
        WebauthnReg,
        CACHE_NAME_WEBAUTHN.to_string(),
        idx.clone(),
        &data.caches.ha_cache_config,
        true
    )
    .await?;
    if res.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Webauthn Registration Request not found".to_string(),
        ));
    }
    cache_remove(
        CACHE_NAME_WEBAUTHN.to_string(),
        idx,
        &data.caches.ha_cache_config,
        AckLevel::Quorum,
    )
    .await?;
    let reg_data = res.unwrap();

    let reg_state = serde_json::from_str::<PasskeyRegistration>(&reg_data.reg_state).unwrap();
    match data
        .webauthn
        .finish_passkey_registration(&req.data, &reg_state)
    {
        Ok(pk) => {
            // force UV check
            let cred = Credential::from(pk.clone());
            if user.account_type() != AccountType::Password || *WEBAUTHN_FORCE_UV {
                if !cred.user_verified {
                    warn!(
                        "Webauthn Registration Ceremony without User Verification for user {:?}",
                        user.id
                    );
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Forbidden,
                        "User Presence only is not allowed - Verification is needed".to_string(),
                    ));
                }
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
                data,
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
