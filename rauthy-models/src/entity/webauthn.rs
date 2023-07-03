use crate::app_state::AppState;
use crate::entity::users::User;
use crate::request::{
    MfaPurpose, WebauthnAuthFinishRequest, WebauthnRegFinishRequest, WebauthnRegStartRequest,
};
use crate::response::{WebauthnAuthStartResponse, WebauthnLoginFinishResponse};
use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::{cookie, web, HttpResponse};
use rauthy_common::constants::{
    CACHE_NAME_WEBAUTHN, CACHE_NAME_WEBAUTHN_DATA, COOKIE_MFA, IDX_WEBAUTHN, WEBAUTHN_RENEW_EXP,
    WEBAUTHN_REQ_EXP,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::{base64_decode, decrypt, new_store_id};
use rauthy_common::utils::{base64_encode, encrypt, get_rand};
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, cache_remove, AckLevel};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use std::ops::Add;
use time::OffsetDateTime;
use tracing::{error, info};
use utoipa::ToSchema;
use webauthn_rs::prelude::*;

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct PasskeyEntity {
    pub id: String,
    pub passkey: String,
}

/// CRUD
impl PasskeyEntity {
    pub async fn create(
        pk: Passkey,
        txn: &mut sqlx::Transaction<'_, sqlx::Any>,
    ) -> Result<Self, ErrorResponse> {
        // json, because bincode does not support deserialize from any, which would be the case here
        let passkey = serde_json::to_string(&pk).unwrap();

        let entity = Self {
            id: new_store_id(),
            passkey,
        };

        sqlx::query("insert into webauthn (id, passkey) values ($1, $2)")
            .bind(&entity.id)
            .bind(&entity.passkey)
            .execute(txn)
            .await?;

        Ok(entity)
    }

    pub async fn delete(
        &self,
        data: &web::Data<AppState>,
        txn: Option<&mut sqlx::Transaction<'_, sqlx::Any>>,
    ) -> Result<(), ErrorResponse> {
        PasskeyEntity::delete_by_id(data, &self.id, txn).await
    }

    pub async fn delete_by_id(
        data: &web::Data<AppState>,
        id: &str,
        txn: Option<&mut sqlx::Transaction<'_, sqlx::Any>>,
    ) -> Result<(), ErrorResponse> {
        let q = sqlx::query("delete from webauthn where id = $1").bind(id);

        if let Some(txn) = txn {
            q.execute(txn).await?;
        } else {
            q.execute(&data.db).await?;
        }

        let idx = format!("{}{}", IDX_WEBAUTHN, id);
        cache_remove(
            CACHE_NAME_WEBAUTHN.to_string(),
            idx,
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await?;

        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, id: String) -> Result<Self, ErrorResponse> {
        let pk = cache_get!(
            PasskeyEntity,
            CACHE_NAME_WEBAUTHN.to_string(),
            id.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if pk.is_some() {
            return Ok(pk.unwrap());
        }

        let pk = sqlx::query_as::<_, Self>("select * from webauthn where id = $1")
            .bind(&id)
            .fetch_one(&data.db)
            .await?;

        let idx = format!("{}{}", IDX_WEBAUTHN, pk.id);
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

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        sqlx::query("update webauthn set passkey = $1 where id = $2")
            .bind(&self.passkey)
            .bind(&self.id)
            .execute(&data.db)
            .await?;

        let idx = format!("{}{}", IDX_WEBAUTHN, self.id);
        cache_insert(
            CACHE_NAME_WEBAUTHN.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &self,
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

/// CURD
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

/// This is the data, that will be passed to the client as response to a successful MFA auth
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

/// CRUD
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

/// CRUD
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
    id: String,
    purpose: MfaPurpose,
) -> Result<WebauthnAuthStartResponse, ErrorResponse> {
    let user = User::find(data, id).await?;

    // This app_data will be returned to the client upon successful webauthn authentication
    let add_data = match purpose {
        MfaPurpose::Login(code) => {
            let d = WebauthnLoginReq::find(data, code).await?;
            WebauthnAdditionalData::Login(d)
        }
        MfaPurpose::PasswordReset => {
            let svc_req = WebauthnServiceReq::new(user.id.clone());
            svc_req.save(data).await?;
            WebauthnAdditionalData::Service(svc_req)
        }
        MfaPurpose::Test => WebauthnAdditionalData::Test,
    };

    let pks = user
        .get_passkeys(data)
        .await?
        .iter()
        .map(|pk_entity| pk_entity.get_pk())
        .collect::<Vec<Passkey>>();

    match data.webauthn.start_passkey_authentication(pks.as_slice()) {
        Ok((rcr, auth_state)) => {
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
    id: String,
    req: WebauthnAuthFinishRequest,
) -> Result<WebauthnAdditionalData, ErrorResponse> {
    let user = User::find(data, id).await?;
    let auth_data = WebauthnData::find(data, req.code).await?;
    let auth_state = serde_json::from_str(&auth_data.auth_state_json).unwrap();

    let pks = user.get_passkeys(data).await?;

    match data
        .webauthn
        .finish_passkey_authentication(&req.data, &auth_state)
    {
        Ok(auth_result) => {
            for mut pk_entity in pks {
                let mut pk = pk_entity.get_pk();
                if let Some(updated) = pk.update_credential(&auth_result) {
                    if updated {
                        pk_entity.passkey = serde_json::to_string(&pk).unwrap();
                        pk_entity.save(data).await?;
                    }
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
    pub uid: String,
    pub uuid: Uuid,
    pub reg_state: String,
}

pub async fn reg_start(
    data: &web::Data<AppState>,
    id: String,
    req: WebauthnRegStartRequest,
) -> Result<CreationChallengeResponse, ErrorResponse> {
    let user = User::find(data, id).await?;
    user.is_slot_free(req.slot)?;
    let uuid = Uuid::new_v4();

    match data
        .webauthn
        .start_passkey_registration(uuid, &user.email, &user.email, None)
    {
        Ok((ccr, reg_state)) => {
            let reg_data = WebauthnReg {
                uid: user.id.clone(),
                uuid,
                // TODO the reg_state cannot be serialized with bincode - open an issue?
                reg_state: serde_json::to_string(&reg_state).unwrap(),
            };

            // persist the reg_state
            let idx = format!("reg_{:?}_{}", req.slot, user.id);
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
    user.is_slot_free(req.slot)?;

    let idx = format!("reg_{:?}_{}", req.slot, user.id);
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
            let mut txn = data.db.begin().await?;
            let pk_entity = PasskeyEntity::create(pk, &mut txn).await?;

            match req.slot {
                1 => user.sec_key_1 = Some(pk_entity.id),
                2 => user.sec_key_2 = Some(pk_entity.id),
                _ => unreachable!(),
            }
            user.save(data, None, Some(&mut txn)).await?;

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

// pub fn mfa_cookie_build(email: &str) -> cookie::Cookie {
//     let exp = OffsetDateTime::now_utc().add(::time::Duration::hours(*WEBAUTHN_RENEW_EXP));
//     let cookie_exp = cookie::Expiration::from(exp);
//     cookie::Cookie::build(COOKIE_MFA, email)
//         .http_only(true)
//         .secure(true)
//         .same_site(cookie::SameSite::Lax)
//         .expires(cookie_exp)
//         .path("/auth")
//         .finish()
// }
//
// /// Validates the correct user_id for the cookie value
// #[inline]
// pub fn mfa_cookie_validate_user(cookie: &cookie::Cookie, email: &str) -> bool {
//     debug!("mfa cookie value: {}", cookie.value());
//     cookie.value() == email
// }
