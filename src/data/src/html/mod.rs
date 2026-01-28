use crate::database::{Cache, DB};
use crate::entity::auth_providers::AuthProviderTemplate;
use crate::entity::logos::{Logo, LogoType};
use crate::html::templates::{
    AccountHtml, AdminApiKeysHtml, AdminAttributesHtml, AdminBlacklistHtml, AdminClientsHtml,
    AdminConfigArgon2Html, AdminConfigBackupsHtml, AdminConfigEncryptionHtml, AdminConfigJwksHtml,
    AdminConfigPolicyHtml, AdminEventsHtml, AdminGroupsHtml, AdminHtml, AdminPAMHtml,
    AdminRolesHtml, AdminScopesHtml, AdminSessionsHtml, AdminUsersHtml, DeviceHtml, FedCMHtml,
    HtmlTemplate, IndexHtml, LogoutHtml, ProviderCallbackHtml, ProvidersHtml,
    UserPasswordResetHtml, UserRegisterHtml,
};
use crate::language::Language;
use actix_web::http::header::ACCEPT_ENCODING;
use actix_web::{HttpRequest, HttpResponse};
use rauthy_common::compression::{compress_br_9, compress_br_dyn, compress_gzip};
use rauthy_common::constants::HEADER_HTML;
use rauthy_error::ErrorResponse;
use serde::Deserialize;
use templates::AdminDocsHtml;
use tracing::debug;

pub mod templates;

#[derive(Debug, Deserialize)]
pub enum HtmlCached {
    Account,
    Admin,
    AdminApiKeys,
    AdminAttributes,
    AdminBlacklist,
    AdminClients,
    AdminEvents,
    AdminGroups,
    AdminProviders,
    AdminRoles,
    AdminScopes,
    AdminPAM,
    AdminSessions,
    AdminUsers,
    AuthProviderCallback,
    ConfigArgon2,
    ConfigBackups,
    ConfigEncryption,
    ConfigJwks,
    ConfigPolicy,
    Device,
    Docs,
    FedCM,
    Index,
    Logout(String),
    PasswordReset,
    UserRegistration,
}

impl HtmlCached {
    #[inline]
    fn as_str(&self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::Admin => "admin",
            Self::AdminApiKeys => "admin_api_keys",
            Self::AdminAttributes => "admin_attributes",
            Self::AdminBlacklist => "admin_blacklist",
            Self::AdminClients => "admin_clients",
            Self::AdminEvents => "admin_events",
            Self::AdminGroups => "admin_groups",
            Self::AdminProviders => "admin_providers",
            Self::AdminRoles => "admin_roles",
            Self::AdminScopes => "admin_scopes",
            Self::AdminPAM => "admin_pam",
            Self::AdminSessions => "admin_sessions",
            Self::AdminUsers => "admin_users",
            Self::AuthProviderCallback => "auth_provider_cb",
            Self::ConfigArgon2 => "cfg_argon2",
            Self::ConfigBackups => "cfg_backup",
            Self::ConfigEncryption => "cfg_encryption",
            Self::ConfigJwks => "cfg_jwks",
            Self::ConfigPolicy => "cfg_policy",
            Self::Device => "device",
            Self::Docs => "docs",
            Self::FedCM => "fed_cm",
            Self::Index => "index",
            Self::Logout(_) => "logout",
            Self::PasswordReset => "password_reset",
            Self::UserRegistration => "user_reg",
        }
    }

    #[inline]
    fn cache_key(&self, lang: &Language, encoding: &str) -> String {
        format!("{}_{}_{encoding}", self.as_str(), lang.as_str())
    }

    /// Handles the request and builds a full `HttpResponse` with compression and caching.
    #[tracing::instrument(level = "debug", skip(req, theme_ts, with_cache))]
    pub async fn handle(
        self,
        req: HttpRequest,
        theme_ts: i64,
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
        debug!(language = ?lang);
        let cache_key = if with_cache {
            if let Some(bytes) = DB::hql()
                .get_bytes(Cache::Html, self.cache_key(&lang, encoding))
                .await?
            {
                return Ok(HttpResponse::Ok()
                    .insert_header(("content-encoding", encoding))
                    .insert_header(HEADER_HTML)
                    .body(bytes));
            }
            self.cache_key(&lang, encoding)
        } else {
            String::default()
        };

        let body = match self {
            Self::Account => {
                let providers = AuthProviderTemplate::get_all_json_template().await?;
                AccountHtml::build(
                    &lang,
                    theme_ts,
                    &[
                        HtmlTemplate::AuthProviders(providers),
                        HtmlTemplate::UserValues,
                    ],
                )
            }
            Self::Admin => AdminHtml::build(&lang, theme_ts),
            Self::AdminApiKeys => AdminApiKeysHtml::build(&lang, theme_ts),
            Self::AdminAttributes => AdminAttributesHtml::build(&lang, theme_ts),
            Self::AdminBlacklist => AdminBlacklistHtml::build(&lang, theme_ts),
            Self::AdminClients => AdminClientsHtml::build(&lang, theme_ts),
            Self::AdminEvents => AdminEventsHtml::build(&lang, theme_ts),
            Self::AdminGroups => AdminGroupsHtml::build(&lang, theme_ts),
            Self::AdminProviders => ProvidersHtml::build(&lang, theme_ts),
            Self::AdminRoles => AdminRolesHtml::build(&lang, theme_ts),
            Self::AdminScopes => AdminScopesHtml::build(&lang, theme_ts),
            Self::AdminPAM => AdminPAMHtml::build(&lang, theme_ts),
            Self::AdminSessions => AdminSessionsHtml::build(&lang, theme_ts),
            Self::AdminUsers => AdminUsersHtml::build(&lang, theme_ts),
            Self::AuthProviderCallback => ProviderCallbackHtml::build(&lang, theme_ts),
            Self::ConfigArgon2 => AdminConfigArgon2Html::build(&lang, theme_ts),
            Self::ConfigBackups => AdminConfigBackupsHtml::build(&lang, theme_ts),
            Self::ConfigEncryption => AdminConfigEncryptionHtml::build(&lang, theme_ts),
            Self::ConfigJwks => AdminConfigJwksHtml::build(&lang, theme_ts),
            Self::ConfigPolicy => AdminConfigPolicyHtml::build(&lang, theme_ts),
            Self::Device => DeviceHtml::build(&lang, theme_ts),
            Self::Docs => AdminDocsHtml::build(&lang, theme_ts),
            Self::FedCM => FedCMHtml::build(&lang, theme_ts),
            Self::Index => IndexHtml::build(&lang, theme_ts),
            Self::Logout(csrf_token) => LogoutHtml::build(csrf_token, &lang, theme_ts),
            Self::PasswordReset => {
                let logo_updated = Logo::find_updated("rauthy", &LogoType::Client).await?;
                UserPasswordResetHtml::build(&lang, theme_ts, logo_updated)
            }
            Self::UserRegistration => {
                let providers = AuthProviderTemplate::get_all_json_template().await?;
                UserRegisterHtml::build(&lang, theme_ts, HtmlTemplate::AuthProviders(providers))
            }
        };
        let body_bytes = match encoding {
            "br" => {
                if with_cache {
                    // it's only worth the extra compute if we actually cache this response
                    debug!("compress_br_9");
                    compress_br_9(body.as_bytes())?
                } else {
                    debug!("compress_br_dyn");
                    compress_br_dyn(body.as_bytes())?
                }
            }
            // TODO lower quality gzip for dynamic content?
            "gzip" => compress_gzip(body.as_bytes())?,
            _ => body.as_bytes().to_vec(),
        };

        if with_cache {
            DB::hql()
                .put_bytes(Cache::Html, cache_key, body_bytes.clone(), None)
                .await?;
        }

        Ok(HttpResponse::Ok()
            .insert_header(("content-encoding", encoding))
            .insert_header(HEADER_HTML)
            .body(body_bytes))
    }
}
