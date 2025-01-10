use crate::database::{Cache, DB};
use crate::language::Language;
use account::{I18nAccount, I18nAccountMfa};
use authorize::I18nAuthorize;
use device::I18nDevice;
use error::I18nError;
use index::I18nIndex;
use logout::I18nLogout;
use password_policy::I18nPasswordPolicy;
use password_reset::I18nPasswordReset;
use rauthy_common::compression::{compress_br, compress_gzip};
use rauthy_error::ErrorResponse;
use register::I18nRegister;
use serde::Serialize;

pub mod account;
pub mod authorize;
pub mod device;
pub mod error;
pub mod index;
pub mod logout;
pub mod password_policy;
pub mod password_reset;
pub mod register;

pub trait SsrJson {
    fn build(lang: &Language) -> Self;
    fn as_json(&self) -> String;
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18n {
    pub account: I18nAccount,
    pub authorize: I18nAuthorize,
    pub device: I18nDevice,
    pub error: I18nError,
    pub index: I18nIndex,
    pub logout: I18nLogout,
    pub mfa: I18nAccountMfa,
    pub password_policy: I18nPasswordPolicy,
    pub password_reset: I18nPasswordReset,
    pub register: I18nRegister,
}

impl I18n {
    pub fn build(lang: &Language) -> Self {
        Self {
            account: I18nAccount::build(lang),
            authorize: I18nAuthorize::build(lang),
            device: I18nDevice::build(lang),
            error: I18nError::build(lang),
            index: I18nIndex::build(lang),
            logout: I18nLogout::build(lang),
            mfa: I18nAccountMfa::build(lang),
            password_policy: I18nPasswordPolicy::build(lang),
            password_reset: I18nPasswordReset::build(lang),
            register: I18nRegister::build(lang),
        }
    }

    pub fn build_json(lang: &Language) -> Result<String, ErrorResponse> {
        Ok(serde_json::to_string(&Self::build(lang))?)
    }

    pub async fn build_br(lang: &Language) -> Result<Vec<u8>, ErrorResponse> {
        if let Some(bytes) = DB::client()
            .get_bytes(Cache::Html, Self::cache_key_br(lang))
            .await?
        {
            return Ok(bytes);
        }

        let plain = Self::build_json(lang)?;
        let compressed = compress_br(plain.as_bytes())?;
        DB::client()
            .put_bytes(
                Cache::Html,
                Self::cache_key_br(lang),
                compressed.clone(),
                None,
            )
            .await?;

        Ok(compressed)
    }

    pub async fn build_gzip(lang: &Language) -> Result<Vec<u8>, ErrorResponse> {
        if let Some(bytes) = DB::client()
            .get_bytes(Cache::Html, Self::cache_key_gzip(lang))
            .await?
        {
            return Ok(bytes);
        }

        let plain = Self::build_json(lang)?;
        let compressed = compress_gzip(plain.as_bytes())?;
        DB::client()
            .put_bytes(
                Cache::Html,
                Self::cache_key_gzip(lang),
                compressed.clone(),
                None,
            )
            .await?;

        Ok(compressed)
    }

    #[inline]
    fn cache_key_br(lang: &Language) -> String {
        format!("i18n_{}_br", lang.as_str())
    }

    #[inline]
    fn cache_key_gzip(lang: &Language) -> String {
        format!("i18n_{}_br", lang.as_str())
    }
}
