use crate::database::{Cache, DB};
use crate::entity::auth_providers::AuthProviderTemplate;
use crate::entity::colors::ColorEntity;
use crate::html::templates::{
    AccountHtml, AdminConfigArgon2Html, AdminConfigEncryptionHtml, AdminConfigJwksHtml,
    AdminConfigPolicyHtml, DeviceHtml, FedCMHtml, HtmlTemplate, IndexHtml,
};
use crate::language::Language;
use actix_web::http::header::ACCEPT_ENCODING;
use actix_web::{HttpRequest, HttpResponse};
use rauthy_common::compression::{compress_br_9, compress_br_dyn, compress_gzip};
use rauthy_common::constants::HEADER_HTML;
use rauthy_error::ErrorResponse;
use serde::Deserialize;
use templates::AdminDocsHtml;

pub mod templates;

#[derive(Debug, Deserialize)]
pub enum HtmlCached {
    Account,
    Docs,
    ConfigArgon2,
    ConfigEncryption,
    ConfigJwks,
    ConfigPolicy,
    Device,
    FedCM,
    Index,
}

impl HtmlCached {
    #[inline]
    fn as_str(&self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::Docs => "docs",
            Self::ConfigArgon2 => "cfg_argon2",
            Self::ConfigEncryption => "cfg_encryption",
            Self::ConfigJwks => "cfg_jwks",
            Self::ConfigPolicy => "cfg_policy",
            Self::Device => "device",
            Self::FedCM => "fed_cm",
            Self::Index => "index",
        }
    }

    #[inline]
    fn cache_key(&self, lang: &Language, encoding: &str) -> String {
        format!("{}_{}_{}", self.as_str(), lang.as_str(), encoding)
    }

    pub async fn handle(
        &self,
        req: HttpRequest,
        with_cache: bool,
    ) -> Result<HttpResponse, ErrorResponse> {
        let encoding = if let Some(enc) = req.headers().get(ACCEPT_ENCODING) {
            let accept = enc.to_str().unwrap_or("none");
            if accept.contains("br") {
                "br"
            } else if accept.contains("gzip") {
                "gzip"
            } else {
                "none"
            }
        } else {
            "none"
        };

        let lang = Language::try_from(&req).unwrap_or_default();
        if with_cache {
            if let Some(bytes) = DB::client()
                .get_bytes(Cache::Html, self.cache_key(&lang, encoding))
                .await?
            {
                return Ok(HttpResponse::Ok()
                    .insert_header(("content-encoding", encoding))
                    .insert_header(HEADER_HTML)
                    .body(bytes));
            }
        }

        // TODO remove the colors after svelte 5 migration is finished
        let colors = ColorEntity::find_rauthy().await?;

        let body = match self {
            Self::Account => {
                let providers = AuthProviderTemplate::get_all_json_template().await?;
                AccountHtml::build(&colors, &lang, &[HtmlTemplate::AuthProviders(providers)])
            }
            Self::Docs => AdminDocsHtml::build(&colors, &lang),
            Self::ConfigArgon2 => AdminConfigArgon2Html::build(&colors, &lang),
            Self::ConfigEncryption => AdminConfigEncryptionHtml::build(&colors, &lang),
            Self::ConfigJwks => AdminConfigJwksHtml::build(&colors, &lang),
            Self::ConfigPolicy => AdminConfigPolicyHtml::build(&colors, &lang),
            Self::Device => DeviceHtml::build(&colors, &lang),
            Self::FedCM => FedCMHtml::build(&colors, &lang),
            Self::Index => IndexHtml::build(&colors, &lang),
        };
        let body_bytes = match encoding {
            "br" => {
                if with_cache {
                    // it's only worth the extra compute if we actually cache this response
                    compress_br_9(body.as_bytes())?
                } else {
                    compress_br_dyn(body.as_bytes())?
                }
            }
            // TODO lower quality gzip for dynamic content?
            "gzip" => compress_gzip(body.as_bytes())?,
            _ => body.as_bytes().to_vec(),
        };

        if with_cache {
            DB::client()
                .put_bytes(
                    Cache::Html,
                    self.cache_key(&lang, encoding),
                    body_bytes.clone(),
                    None,
                )
                .await?;
        }

        Ok(HttpResponse::Ok()
            .insert_header(HEADER_HTML)
            .insert_header(("content-encoding", encoding))
            .body(body))
    }
}
