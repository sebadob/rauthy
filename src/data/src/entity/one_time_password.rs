//! Email OTP (`OtpKind::Email`) is the only implemented kind, `Time` and `Phone` are rejected.
//!
//! The invariant: `generate_otp` is `HMAC(secret, step)`, which makes the row's `secret` the whole
//! security of the mailed code. It is generated random for every kind by
//! [`OneTimePassword::create`] and it can never be empty, because an empty secret turns the code
//! into a public function of the server time — see [`OtpSecret`]. Rows from before that are
//! rotated at startup by [`crate::temp_migrations`]. A code is
//! only ever sent out via email, the dev-only reveal endpoint is the single test-only exception.
use crate::database::{Cache, DB};
use crate::email::otp::send_email_otp;
use crate::entity::auth_codes::AuthCodeToSAwait;
use crate::entity::browser_id::BrowserId;
use crate::entity::login_locations::LoginLocation;
use crate::entity::sessions::{MfaAssurance, Session};
use crate::entity::user_consents::PendingConsent;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use actix_web::http::StatusCode;
use actix_web::http::header::{
    self, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, HeaderValue,
};
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder};
use chrono::Utc;
use hiqlite::macros::params;
use image::EncodableLayout;
use rauthy_api_types::tos::ToSAwaitLoginResponse;
use rauthy_api_types::users::{
    MfaPurpose, OtpAuthFinishRequest, OtpAuthResendRequest, OtpAuthStartRequest,
    OtpAuthStartResponse, OtpGetResponse, OtpKind, OtpLoginFinishResponse,
};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{get_rand, new_store_id};
use rauthy_derive::FromPgRow;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use ring::digest;
use ring::hmac;
use ring::rand;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use tracing::info;
use utoipa::ToSchema;

#[derive(Clone, Deserialize, FromPgRow)]
pub struct OneTimePassword {
    pub id: String,
    pub user_id: String,
    pub name: Option<String>,
    pub secret: Vec<u8>,
    pub last_used: i64,
    #[column(parse)]
    pub kind: OtpKind,
    pub is_active: bool,
}

impl Debug for OneTimePassword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OneTimePassword {{ id: {}(...), user_id: {}, name: {:?}, last_used: {}, kind: {}, is_active: {} }}",
            &self.id[..5],
            self.user_id,
            self.name,
            self.last_used,
            self.kind,
            self.is_active
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OtpSecret(pub Vec<u8>);

impl OtpSecret {
    /// The only entropy source. [`OneTimePassword::create`] uses it for every [`OtpKind`].
    ///
    /// If the len is longer than the algorithm it will be compressed by the digest, if the len is
    /// shorter than the algorithm it will be padded with 0x30.
    pub fn generate() -> Result<Self, ErrorResponse> {
        let secret: [u8; digest::SHA512_OUTPUT_LEN] =
            rand::generate(&rand::SystemRandom::new())?.expose();
        Ok(Self(secret.to_vec()))
    }

    /// A stored secret, validated. Rows of `OtpKind::Email` created before this existed have an
    /// empty `secret`; [`crate::temp_migrations`] rotates them at startup, and a code is never
    /// derived from an empty one.
    pub fn from_stored(raw: &[u8]) -> Result<Self, ErrorResponse> {
        if raw.is_empty() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "OTP secret is empty",
            ));
        }
        Ok(Self(raw.to_vec()))
    }
}

// CRUD
impl OneTimePassword {
    pub async fn create(
        user_id: String,
        name: Option<String>,
        kind: OtpKind,
    ) -> Result<Self, ErrorResponse> {
        let otp = Self {
            id: new_store_id(),
            user_id,
            name,
            secret: OtpSecret::generate()?.0,
            last_used: 0,
            kind,
            is_active: false,
        };

        let sql = r#"
INSERT INTO one_time_password (id, user_id, name, secret, last_used, kind, is_active)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        otp.id.clone(),
                        otp.user_id.clone(),
                        otp.name.clone(),
                        otp.secret.clone(),
                        otp.last_used,
                        otp.kind.as_str(),
                        otp.is_active
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &otp.id,
                    &otp.user_id,
                    &otp.name,
                    &otp.secret,
                    &otp.last_used,
                    &otp.kind.as_str(),
                    &otp.is_active,
                ],
            )
            .await?;
        }

        Ok(otp)
    }

    pub async fn delete_all_otp_for_user(user_id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM one_time_password WHERE user_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(user_id)).await?;
        } else {
            DB::pg_execute(sql, &[&user_id]).await?;
        }

        Ok(())
    }

    pub async fn delete(id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM one_time_password WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(id)).await?;
        } else {
            DB::pg_execute(sql, &[&id]).await?;
        }

        Ok(())
    }

    pub async fn find(id: &str) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM one_time_password WHERE id = $1";
        let res = if is_hiqlite() {
            DB::hql().query_as_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(res)
    }

    pub async fn find_by_id_for_user(
        otp_id: &String,
        user_id: &String,
    ) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM one_time_password WHERE id = $1 AND user_id = $2";
        let res = if is_hiqlite() {
            DB::hql()
                .query_as_one(sql, params!(otp_id, user_id))
                .await?
        } else {
            DB::pg_query_one(sql, &[&otp_id, &user_id]).await?
        };

        Ok(res)
    }

    pub async fn find_kind_for_user(
        kind: &OtpKind,
        user_id: &String,
    ) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM one_time_password WHERE user_id = $1 AND kind = $2";
        let res = if is_hiqlite() {
            DB::hql()
                .query_as_one(sql, params!(user_id, kind.as_str()))
                .await?
        } else {
            DB::pg_query_one(sql, &[&user_id, &kind.as_str()]).await?
        };

        Ok(res)
    }

    pub async fn find_active_kind_for_user(
        kind: &OtpKind,
        user_id: &String,
    ) -> Result<Self, ErrorResponse> {
        let sql =
            "SELECT * FROM one_time_password WHERE user_id = $1 AND kind = $2 AND is_active = true";
        let res = if is_hiqlite() {
            DB::hql()
                .query_as_one(sql, params!(user_id, kind.as_str()))
                .await?
        } else {
            DB::pg_query_one(sql, &[&user_id, &kind.as_str()]).await?
        };

        Ok(res)
    }

    pub async fn find_for_user(user_id: &String) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM one_time_password WHERE user_id = $1";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!(user_id)).await?
        } else {
            DB::pg_query(sql, &[&user_id], 1).await?
        };

        Ok(res)
    }

    pub async fn find_active_for_user(user_id: &String) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM one_time_password WHERE user_id = $1 AND is_active = true";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!(user_id)).await?
        } else {
            DB::pg_query(sql, &[&user_id], 1).await?
        };

        Ok(res)
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let sql =
            "UPDATE one_time_password SET name = $1, last_used = $2, is_active = $3 WHERE id = $4";
        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(&self.name, self.last_used, self.is_active, &self.id),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[&self.name, &self.last_used, &self.is_active, &self.id],
            )
            .await?;
        }

        Ok(())
    }
}

impl OneTimePassword {
    const DIGITS_POWER: [u32; 9] = [
        1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000,
    ];

    fn generate_otp(secret: &OtpSecret, step: i64, digest_len: u16, code_len: u8) -> String {
        let msg = step.to_be_bytes();
        let algorithm = match digest_len {
            256 => hmac::HMAC_SHA256,
            384 => hmac::HMAC_SHA384,
            512 => hmac::HMAC_SHA512,
            _ => hmac::HMAC_SHA512,
        };
        let key = hmac::Key::new(algorithm, &secret.0);
        let hash = hmac::sign(&key, msg.as_bytes());
        let hash = hash.as_ref();

        // Unreachable should never panic since the tag should never be empty
        let offset = match hash.last() {
            Some(e) => (e & 0xf) as usize,
            None => unreachable!(),
        };

        let binary = [
            hash[offset] & 0x7f,
            hash[offset + 1],
            hash[offset + 2],
            hash[offset + 3],
        ];
        let binary = u32::from_be_bytes(binary);

        let otp = binary % Self::DIGITS_POWER[code_len as usize];

        format!("{:0width$}", otp, width = code_len as usize)
    }

    pub async fn activate(&mut self) -> Result<(), ErrorResponse> {
        self.is_active = true;
        self.save().await
    }

    pub fn validate(&self, code: &str) -> Result<(), ErrorResponse> {
        match self.kind {
            OtpKind::Email => {
                if Utc::now().timestamp() - self.last_used
                    >= RauthyConfig::get().vars.otp.exp.as_secs() as i64
                {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "otp code expired",
                    ));
                }
                let valid_code = Self::generate_otp(
                    &OtpSecret::from_stored(&self.secret)?,
                    self.last_used,
                    RauthyConfig::get().vars.otp.digest_len_default,
                    RauthyConfig::get().vars.otp.length,
                );
                if !constant_time_eq::constant_time_eq(code.as_bytes(), valid_code.as_bytes()) {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "code is incorrect",
                    ));
                }
            }
            OtpKind::Time | OtpKind::Phone => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "unimplemented",
                ));
            }
        }

        Ok(())
    }

    /// Dev-only test helper: returns the very code that `validate` accepts at this moment, which is
    /// normally only ever sent out via email. Compiled out of release builds entirely.
    #[cfg(debug_assertions)]
    pub fn dev_current_code(&self) -> Result<String, ErrorResponse> {
        Ok(Self::generate_otp(
            &OtpSecret::from_stored(&self.secret)?,
            self.last_used,
            RauthyConfig::get().vars.otp.digest_len_default,
            RauthyConfig::get().vars.otp.length,
        ))
    }

    pub async fn request_otp(&mut self) -> Result<(), ErrorResponse> {
        let user = User::find(self.user_id.clone()).await?;

        let current_time = Utc::now().timestamp();
        let code_len = RauthyConfig::get().vars.otp.length;
        let code = Self::generate_otp(
            &OtpSecret::from_stored(&self.secret)?,
            current_time,
            RauthyConfig::get().vars.otp.digest_len_default,
            code_len,
        );
        self.last_used = current_time;
        self.save().await?;
        match self.kind {
            OtpKind::Email => {
                let code_len = code_len as usize;
                let code = format!("{} {}", &code[0..code_len / 2], &code[code_len / 2..]);
                send_email_otp(&code, &user).await;
            }
            OtpKind::Time | OtpKind::Phone => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "unimplemented",
                ));
            }
        }
        Ok(())
    }

    pub async fn resend_otp(&self) -> Result<(), ErrorResponse> {
        let user = User::find(self.user_id.clone()).await?;

        let code_len = RauthyConfig::get().vars.otp.length;
        let code = Self::generate_otp(
            &OtpSecret::from_stored(&self.secret)?,
            self.last_used,
            RauthyConfig::get().vars.otp.digest_len_default,
            code_len,
        );
        match self.kind {
            OtpKind::Email => {
                let code_len = code_len as usize;
                let code = format!("{} {}", &code[0..code_len / 2], &code[code_len / 2..]);
                send_email_otp(&code, &user).await;
            }
            OtpKind::Time | OtpKind::Phone => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "unimplemented",
                ));
            }
        }
        Ok(())
    }
}

impl From<OneTimePassword> for OtpGetResponse {
    fn from(value: OneTimePassword) -> Self {
        Self {
            id: value.id,
            name: value.name,
            last_used: value.last_used,
            kind: value.kind,
            is_active: value.is_active,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtpData {
    pub code: String,
    pub otp_id: String,
    pub data: OtpAdditionalData,
}

// CRUD
impl OtpData {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .delete(Cache::OneTimePassword, self.code.clone())
            .await?;
        Ok(())
    }

    pub async fn find(code: String) -> Result<Self, ErrorResponse> {
        let res = DB::hql().get(Cache::OneTimePassword, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Otp Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = Some(RauthyConfig::get().vars.otp.exp.as_secs() as i64);
        DB::hql()
            .put(Cache::OneTimePassword, self.code.clone(), &self, ttl)
            .await?;
        Ok(())
    }
}

// This is the data, that will be passed to the client as response to a successful MFA auth
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum OtpAdditionalData {
    Login(OtpLoginReq),
    Service(OtpServiceReq),
    Test(String), // saves the User ID
    LoginToSAwait(OtpLoginToSAwaitCode),
}

impl OtpAdditionalData {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        match self {
            Self::Login(d) => d.delete().await,
            // The service req data is not deleted here, but actually further down the road
            // after the service req has been made.
            Self::Service(_) => Ok(()),
            Self::Test(_) => Ok(()),
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
                    builder.json(OtpLoginFinishResponse {
                        loc: login_req.header_loc,
                    })
                }
            }
            Self::Service(svc_req) => HttpResponse::Accepted().json(svc_req),
            Self::Test(_) => HttpResponse::Accepted().finish(),
            Self::LoginToSAwait(tos_req) => {
                let mut resp = HttpResponseBuilder::new(StatusCode::from_u16(206).unwrap()).json(
                    &ToSAwaitLoginResponse {
                        tos_await_code: tos_req.await_code,
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
pub struct OtpLoginReq {
    pub code: String,
    pub user_id: String,
    pub header_loc: String,
    pub header_origin: Option<String>,
    pub tos_await_data: Option<OtpToSAwaitData>,
    pub needs_user_update: bool,
    pub session_id: String,
    /// The consent sent with the login request (`POST /authorize`, `/authorize/refresh` or a
    /// provider login), written as the user's grant once the ceremony verified the user.
    pub pending_consent: Option<PendingConsent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct OtpToSAwaitData {
    pub auth_code: String,
    pub auth_code_lifetime: i32,
}

// CRUD
impl OtpLoginReq {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .delete(Cache::OneTimePassword, self.code.clone())
            .await?;
        Ok(())
    }

    pub async fn find(code: &String) -> Result<Self, ErrorResponse> {
        let res: Option<Self> = DB::hql().get(Cache::OneTimePassword, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "OneTimePassword Login Request Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = Some(RauthyConfig::get().vars.otp.exp.as_secs() as i64);
        DB::hql()
            .put(Cache::OneTimePassword, self.code.clone(), self, ttl)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct OtpServiceReq {
    pub code: String,
    pub user_id: String,
}

// CRUD
impl OtpServiceReq {
    pub fn new(user_id: String) -> Self {
        Self {
            code: get_rand(48),
            user_id,
        }
    }

    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .delete(Cache::OneTimePassword, self.code.clone())
            .await?;
        Ok(())
    }

    pub async fn find(code: String) -> Result<Self, ErrorResponse> {
        let res = DB::hql().get(Cache::OneTimePassword, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "OTP Service Request Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = Some(RauthyConfig::get().vars.otp.exp.as_secs() as i64);
        DB::hql()
            .put(Cache::OneTimePassword, self.code.clone(), self, ttl)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct OtpLoginToSAwaitCode {
    await_code: String,
    user_id: String,
    header_origin: Option<String>,
}

pub async fn auth_start(
    user_id: Option<String>,
    payload: &OtpAuthStartRequest,
) -> Result<OtpAuthStartResponse, ErrorResponse> {
    let (add_data, user_id) = match &payload.purpose {
        MfaPurpose::Login(code) => {
            debug_assert!(user_id.is_none());
            let d = OtpLoginReq::find(code).await?;
            let user_id = d.user_id.clone();
            (OtpAdditionalData::Login(d), user_id)
        }
        MfaPurpose::Discover => {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "User discovery does not exist for OTP",
            ));
        }
        MfaPurpose::MfaModToken
        | MfaPurpose::PamLogin
        | MfaPurpose::PasswordNew
        | MfaPurpose::PasswordReset => {
            let user_id = user_id.expect("user_id should always exist for non-login otp starts");
            let svc_req = OtpServiceReq::new(user_id.clone());
            svc_req.save().await?;
            (OtpAdditionalData::Service(svc_req), user_id)
        }
        MfaPurpose::Test => {
            let user_id = user_id.expect("user_id should always exist for non-login otp starts");
            (OtpAdditionalData::Test(user_id.clone()), user_id)
        }
    };

    let mut otp = OneTimePassword::find_by_id_for_user(&payload.otp_id, &user_id).await?;
    otp.request_otp().await?;

    add_data.delete().await?;
    let auth_data = OtpData {
        code: get_rand(48),
        otp_id: payload.otp_id.clone(),
        data: add_data,
    };
    auth_data.save().await?;

    Ok(OtpAuthStartResponse {
        code: auth_data.code,
        exp: Utc::now().add(RauthyConfig::get().vars.otp.exp).timestamp(),
    })
}

pub async fn auth_resend(payload: OtpAuthResendRequest) -> Result<(), ErrorResponse> {
    let auth_data = OtpData::find(payload.code).await?;

    let otp = OneTimePassword::find(&auth_data.otp_id).await?;
    otp.resend_otp().await
}

pub async fn auth_finish(
    req: &HttpRequest,
    browser_id: BrowserId,
    session: Option<Session>,
    payload: OtpAuthFinishRequest,
) -> Result<OtpAdditionalData, ErrorResponse> {
    let auth_data = OtpData::find(payload.code).await?;
    auth_data.delete().await?;

    let (user_id, is_login) = match &auth_data.data {
        OtpAdditionalData::Login(d) => (&d.user_id, true),
        OtpAdditionalData::Service(d) => (&d.user_id, false),
        OtpAdditionalData::Test(user_id) => (user_id, false),
        OtpAdditionalData::LoginToSAwait(d) => (&d.user_id, false),
    };

    let otp = OneTimePassword::find(&auth_data.otp_id).await?;
    match otp.validate(&payload.otp_code) {
        Ok(()) => {
            let mut user = User::find(user_id.clone()).await?;

            LoginLocation::spawn_background_check(user.clone(), req, browser_id)?;

            if is_login && let Some(mut session) = session {
                let OtpAdditionalData::Login(data) = &auth_data.data else {
                    unreachable!("is_login is only ever true for OtpAdditionalData::Login")
                };
                if data.session_id != session.id || &data.user_id != user_id {
                    data.delete().await?;
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Forbidden,
                        "Session / User ID mismatch for login ceremony",
                    ));
                }

                session
                    .set_authenticated(&user, MfaAssurance::Ceremony)
                    .await?;
                user.save_login_success().await?;
                if let Some(pending) = &data.pending_consent {
                    pending.upsert_for_unchanged_client(&user, req).await?;
                }
            }

            info!(
                user.id = user_id,
                "OneTimePassword Authentication succesful"
            );

            if let OtpAdditionalData::Login(data) = auth_data.data {
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

                    Ok(OtpAdditionalData::LoginToSAwait(OtpLoginToSAwaitCode {
                        await_code: code_await.await_code,
                        user_id: user.id,
                        header_origin: data.header_origin,
                    }))
                } else {
                    Ok(OtpAdditionalData::Login(data))
                }
            } else {
                Ok(auth_data.data)
            }
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::one_time_password::OneTimePassword;

    #[test]
    fn test_hotp_rfc_6238() {
        let seed32 = vec![
            0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34,
            0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
            0x39, 0x30, 0x31, 0x32,
        ];
        let seed64 = vec![
            0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34,
            0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
            0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32,
            0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
            0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34,
        ];
        // The RFC vectors are secrets, so they go through the validating constructor.
        let seed32 = OtpSecret::from_stored(&seed32).unwrap();
        let seed64 = OtpSecret::from_stored(&seed64).unwrap();

        let steps = 30;

        let mut t = 59 / steps;
        assert_eq!(
            OneTimePassword::generate_otp(&seed32, t, 256, 8),
            "46119246"
        );
        assert_eq!(
            OneTimePassword::generate_otp(&seed64, t, 512, 8),
            "90693936"
        );

        t = 1111111109 / steps;
        assert_eq!(
            OneTimePassword::generate_otp(&seed32, t, 256, 8),
            "68084774"
        );
        assert_eq!(
            OneTimePassword::generate_otp(&seed64, t, 512, 8),
            "25091201"
        );

        t = 1111111111 / steps;
        assert_eq!(
            OneTimePassword::generate_otp(&seed32, t, 256, 8),
            "67062674"
        );
        assert_eq!(
            OneTimePassword::generate_otp(&seed64, t, 512, 8),
            "99943326"
        );

        t = 1234567890 / steps;
        assert_eq!(
            OneTimePassword::generate_otp(&seed32, t, 256, 8),
            "91819424"
        );
        assert_eq!(
            OneTimePassword::generate_otp(&seed64, t, 512, 8),
            "93441116"
        );

        t = 2000000000 / steps;
        assert_eq!(
            OneTimePassword::generate_otp(&seed32, t, 256, 8),
            "90698825"
        );
        assert_eq!(
            OneTimePassword::generate_otp(&seed64, t, 512, 8),
            "38618901"
        );

        t = 20000000000 / steps;
        assert_eq!(
            OneTimePassword::generate_otp(&seed32, t, 256, 8),
            "77737706"
        );
        assert_eq!(
            OneTimePassword::generate_otp(&seed64, t, 512, 8),
            "47863826"
        );
    }

    /// An OTP secret is a credential: it must be unpredictable at creation and it must never be
    /// empty, because `generate_otp` is `HMAC(secret, step)` and an empty key makes the derived
    /// code a public function of the server time.
    #[test]
    fn test_otp_secret_rejects_empty() {
        assert!(OtpSecret::from_stored(&[]).is_err());
        assert!(OtpSecret::from_stored(&[0; digest::SHA512_OUTPUT_LEN]).is_ok());
    }

    /// [`OneTimePassword::create`] uses this as its only entropy source, for every [`OtpKind`].
    #[test]
    fn test_otp_secret_is_random() {
        let a = OtpSecret::generate().unwrap();
        let b = OtpSecret::generate().unwrap();
        assert_eq!(a.0.len(), digest::SHA512_OUTPUT_LEN);
        assert_ne!(a, b);
    }

    /// The derivation exactly as it was *before* the empty secret fix: `HMAC(secret, step)` over
    /// the raw stored bytes. Kept here only to pin that the code an attacker computes from an
    /// empty secret is not the code a real row accepts.
    fn legacy_code(secret: &[u8], step: i64, code_len: u8) -> String {
        let key = hmac::Key::new(hmac::HMAC_SHA512, secret);
        let hash = hmac::sign(&key, &step.to_be_bytes());
        let hash = hash.as_ref();
        let offset = (hash[hash.len() - 1] & 0xf) as usize;
        let binary = u32::from_be_bytes([
            hash[offset] & 0x7f,
            hash[offset + 1],
            hash[offset + 2],
            hash[offset + 3],
        ]);
        format!(
            "{:0width$}",
            binary % OneTimePassword::DIGITS_POWER[code_len as usize],
            width = code_len as usize
        )
    }

    /// With an empty secret, `legacy_code(&[], step)` is computable by anyone for the whole
    /// `otp.exp` window, so it must not be derivable from a real secret.
    #[test]
    fn test_empty_secret_code_is_not_a_real_code() {
        let step = 1_700_000_000;
        let attacker_code = legacy_code(&[], step, 6);
        let secret = OtpSecret::from_stored(&[0x42; digest::SHA512_OUTPUT_LEN]).unwrap();
        assert_ne!(
            attacker_code,
            OneTimePassword::generate_otp(&secret, step, 512, 6)
        );

        // The pre-fix `OtpKind::Email` rows all had the very same "secret", which is what made
        // the code predictable at all.
        let a = legacy_code(&[], step, 6);
        let b = legacy_code(&[], step, 6);
        assert_eq!(a, b);
    }
}
