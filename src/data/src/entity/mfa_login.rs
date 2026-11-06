// Copyright 2026 Alexander Warg <alex@wargs.de>

use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Unified MFA login request state
/// Works for both WebAuthn and OTP flows
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct MfaLoginReq {
    pub code: String,
    pub user_id: String,
    pub header_loc: String,
    pub header_origin: Option<String>,
    pub tos_await_data: Option<MfaToSAwaitData>,
    pub needs_user_update: bool,
    pub has_webauthn: bool,
    pub has_otp: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct MfaToSAwaitData {
    pub auth_code: String,
    pub auth_code_lifetime: i32,
}

pub enum MfaKind {
    Webauthn,
    Otp,
}

// CRUD
impl MfaLoginReq {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::Webauthn, self.code.clone()).await?;
        Ok(())
    }

    pub async fn find_remove(code: String) -> Result<Self, ErrorResponse> {
        let res: Option<Self> = DB::hql().get_remove(Cache::Webauthn, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "MFA Login Request Data not found",
            )),
            Some(res) => Ok(res),
        }
    }

    pub async fn find_kind_remove(code: String, mfa_kind: MfaKind) -> Result<Self, ErrorResponse> {
        let res: Option<Self> = DB::hql().get_remove(Cache::Webauthn, code).await?;
        match res {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "MFA Login Request Data not found",
            )),
            Some(res) => {
                match mfa_kind {
                    MfaKind::Webauthn => { if res.has_webauthn { return Ok(res); } },
                    MfaKind::Otp => { if res.has_otp { return Ok(res); } },
                };
                Err(ErrorResponse::new(
                        ErrorResponseType::NotFound,
                        "MFA Login Request Data not found",
                ))
            },
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        // Use the longer TTL between webauthn and otp config to ensure it doesn't expire
        // before the user completes MFA
        let config = RauthyConfig::get();
        let ttl_webauthn = if self.has_webauthn { config.vars.webauthn.data_exp.as_secs() as i64 } else { 0 };
        let ttl_otp = if self.has_otp { config.vars.otp.exp.as_secs() as i64 } else { 0 };
        let ttl = Some(ttl_webauthn.max(ttl_otp));

        DB::hql()
            .put(Cache::Webauthn, self.code.clone(), self, ttl)
            .await?;
        Ok(())
    }
}

