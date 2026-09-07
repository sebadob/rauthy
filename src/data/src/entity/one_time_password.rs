use crate::{
    database::{
        Cache::{self},
        DB,
    },
    email::otp::send_email_otp,
    entity::{
        auth_codes::AuthCodeToSAwait, browser_id::BrowserId, login_locations::LoginLocation,
        sessions::Session, users::User,
    },
    rauthy_config::RauthyConfig,
};
use actix_web::{
    HttpRequest, HttpResponse, HttpResponseBuilder,
    http::{
        StatusCode,
        header::{
            self, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, HeaderValue,
        },
    },
};
use chrono::{TimeDelta, Utc};
use hiqlite::macros::params;
use image::EncodableLayout;
use rauthy_api_types::{
    tos::ToSAwaitLoginResponse,
    users::{
        MfaPurpose, OtpAuthFinishRequest, OtpAuthResendRequest, OtpAuthStartRequest,
        OtpAuthStartResponse, OtpGetResponse, OtpKind, OtpLoginFinishResponse,
    },
};
use rauthy_common::{
    is_hiqlite,
    utils::{get_rand, new_store_id},
};
use rauthy_derive::FromPgRow;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use ring::{
    digest,
    hmac::{self},
    rand::{self},
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter},
    ops::Add,
};
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

// CRUD
impl OneTimePassword {
    pub async fn create(
        user_id: String,
        name: Option<String>,
        kind: OtpKind,
    ) -> Result<Self, ErrorResponse> {
        let secret = match kind {
            OtpKind::Time => {
                // if the len is longer than the algorithm it will be compressed by the digest
                // if the len is shorter than the algorithm it will be padded with 0x30
                let secret: [u8; digest::SHA512_OUTPUT_LEN] =
                    rand::generate(&rand::SystemRandom::new())?.expose();
                secret.to_vec()
            }
            _ => Vec::default(),
        };
        let otp = Self {
            id: new_store_id(),
            user_id,
            name,
            secret,
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
        };

        Ok(())
    }

    pub async fn delete(id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM one_time_password WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(id)).await?;
        } else {
            DB::pg_execute(sql, &[&id]).await?;
        };

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

    fn generate_otp(secret: &[u8], step: i64, digest_len: u16, code_len: u8) -> String {
        let msg = step.to_be_bytes();
        let algorithm = match digest_len {
            256 => hmac::HMAC_SHA256,
            384 => hmac::HMAC_SHA384,
            512 => hmac::HMAC_SHA512,
            _ => hmac::HMAC_SHA512,
        };
        let key = hmac::Key::new(algorithm, secret);
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

    pub async fn validate(&self, code: &str) -> Result<(), ErrorResponse> {
        match self.kind {
            OtpKind::Email => {
                let Some(timeout) = TimeDelta::try_seconds(self.last_used - Utc::now().timestamp())
                else {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "couldn't parse otp's timeout",
                    ));
                };
                if timeout >= RauthyConfig::get().vars.otp.exp_mins {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "otp code expired",
                    ));
                }
                let valid_code = Self::generate_otp(
                    &self.secret,
                    self.last_used,
                    RauthyConfig::get().vars.otp.digest_len_default,
                    RauthyConfig::get().vars.otp.length,
                );
                if code != valid_code {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "code is incorrect",
                    ));
                }
            }
            OtpKind::Time | OtpKind::Phone => {
                unimplemented!()
            }
        };

        Ok(())
    }

    pub async fn request_otp(&mut self) -> Result<(), ErrorResponse> {
        let user = User::find(self.user_id.clone()).await?;

        let current_time = Utc::now().timestamp();
        let code_len = RauthyConfig::get().vars.otp.length;
        let code = Self::generate_otp(
            &self.secret,
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
                unimplemented!()
            }
        };
        Ok(())
    }

    pub async fn resend_otp(&self) -> Result<(), ErrorResponse> {
        let user = User::find(self.user_id.clone()).await?;

        let code_len = RauthyConfig::get().vars.otp.length;
        let code = Self::generate_otp(
            &self.secret,
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
                unimplemented!()
            }
        };
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
        let ttl = Some(RauthyConfig::get().vars.otp.exp_mins.num_seconds());
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
        let ttl = Some(RauthyConfig::get().vars.otp.exp_mins.num_seconds());
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
        let ttl = Some(RauthyConfig::get().vars.otp.exp_mins.num_seconds());
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
        exp: Utc::now()
            .add(RauthyConfig::get().vars.otp.exp_mins)
            .timestamp(),
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
    match otp.validate(&payload.otp_code).await {
        Ok(_) => {
            let mut user = User::find(user_id.clone()).await?;

            LoginLocation::spawn_background_check(user.clone(), req, browser_id)?;

            if is_login && let Some(mut session) = session {
                session.set_authenticated(&user).await?;
                user.last_login = Some(Utc::now().timestamp());
                user.last_failed_login = None;
                user.failed_login_attempts = None;
                user.save(None).await?;
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
}
