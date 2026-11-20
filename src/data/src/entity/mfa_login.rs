// Copyright 2026 Alexander Warg <alex@wargs.de>

use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use crate::{MfaMethod, MfaMethods};
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
    pub session_id: String,
    #[schema(value_type = Vec<MfaMethod>)]
    pub mfa_methods: MfaMethods,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct MfaToSAwaitData {
    pub auth_code: String,
    pub auth_code_lifetime: i32,
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

    pub async fn find_kind_remove(code: String, method: MfaMethod) -> Result<Self, ErrorResponse> {
        // Deliberately removes before checking the method: a request for a method that is not
        // allowed consumes the login code, and the error is the same as for an unknown code, so
        // nothing about the allowed methods leaks.
        let res: Option<Self> = DB::hql().get_remove(Cache::Webauthn, code).await?;
        match res {
            Some(res) if res.mfa_methods.contains(method) => Ok(res),
            _ => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "MFA Login Request Data not found",
            )),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        // Use the longer TTL between webauthn and otp config to ensure it doesn't expire
        // before the user completes MFA
        let vars = &RauthyConfig::get().vars;
        let ttl = self
            .mfa_methods
            .iter()
            .map(|m| match m {
                MfaMethod::Webauthn => vars.webauthn.data_exp,
                MfaMethod::Otp => vars.otp.exp,
            })
            .max()
            .unwrap_or_default();
        let ttl = Some(ttl.as_secs() as i64);

        DB::hql()
            .put(Cache::Webauthn, self.code.clone(), self, ttl)
            .await?;
        Ok(())
    }
}
