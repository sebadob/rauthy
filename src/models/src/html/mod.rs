use crate::database::{Cache, DB};
use crate::entity::auth_providers::AuthProviderTemplate;
use crate::entity::colors::ColorEntity;
use crate::html::templates::{
    AccountHtml, AdminApiKeysHtml, AdminAttributesHtml, AdminBlacklistHtml, AdminClientsHtml,
    AdminConfigArgon2Html, AdminConfigEncryptionHtml, AdminConfigJwksHtml, AdminConfigPolicyHtml,
    AdminEventsHtml, AdminGroupsHtml, AdminHtml, AdminRolesHtml, AdminScopesHtml,
    AdminSessionsHtml, AdminUsersHtml, DeviceHtml, FedCMHtml, HtmlTemplate, IndexHtml, LogoutHtml,
    ProviderCallbackHtml, ProvidersHtml, UserRegisterHtml,
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
    AdminSessions,
    AdminUsers,
    AuthProviderCallback,
    ConfigArgon2,
    ConfigEncryption,
    ConfigJwks,
    ConfigPolicy,
    Device,
    Docs,
    FedCM,
    Index,
    Logout(String),
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
            Self::AdminSessions => "admin_sessions",
            Self::AdminUsers => "admin_users",
            Self::AuthProviderCallback => "auth_provider_cb",
            Self::ConfigArgon2 => "cfg_argon2",
            Self::ConfigEncryption => "cfg_encryption",
            Self::ConfigJwks => "cfg_jwks",
            Self::ConfigPolicy => "cfg_policy",
            Self::Device => "device",
            Self::Docs => "docs",
            Self::FedCM => "fed_cm",
            Self::Index => "index",
            Self::Logout(_) => "logout",
            Self::UserRegistration => "user_reg",
        }
    }

    #[inline]
    fn cache_key(&self, lang: &Language, encoding: &str) -> String {
        format!("{}_{}_{}", self.as_str(), lang.as_str(), encoding)
    }

    /// Handles the request and builds a full `HttpResponse` with compression and caching.
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
        let cache_key = if with_cache {
            if let Some(bytes) = DB::client()
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

        // TODO remove the colors after svelte 5 migration is finished
        let colors = ColorEntity::find_rauthy().await?;

        let body = match self {
            Self::Account => {
                let providers = AuthProviderTemplate::get_all_json_template().await?;
                AccountHtml::build(
                    &colors,
                    &lang,
                    theme_ts,
                    &[HtmlTemplate::AuthProviders(providers)],
                )
            }
            Self::Admin => AdminHtml::build(&colors, &lang, theme_ts),
            Self::AdminApiKeys => AdminApiKeysHtml::build(&colors, &lang, theme_ts),
            Self::AdminAttributes => AdminAttributesHtml::build(&colors, &lang, theme_ts),
            Self::AdminBlacklist => AdminBlacklistHtml::build(&colors, &lang, theme_ts),
            Self::AdminClients => AdminClientsHtml::build(&colors, &lang, theme_ts),
            Self::AdminEvents => AdminEventsHtml::build(&colors, &lang, theme_ts),
            Self::AdminGroups => AdminGroupsHtml::build(&colors, &lang, theme_ts),
            Self::AdminProviders => ProvidersHtml::build(&colors, &lang, theme_ts),
            Self::AdminRoles => AdminRolesHtml::build(&colors, &lang, theme_ts),
            Self::AdminScopes => AdminScopesHtml::build(&colors, &lang, theme_ts),
            Self::AdminSessions => AdminSessionsHtml::build(&colors, &lang, theme_ts),
            Self::AdminUsers => AdminUsersHtml::build(&colors, &lang, theme_ts),
            Self::AuthProviderCallback => ProviderCallbackHtml::build(&colors, &lang, theme_ts),
            Self::ConfigArgon2 => AdminConfigArgon2Html::build(&colors, &lang, theme_ts),
            Self::ConfigEncryption => AdminConfigEncryptionHtml::build(&colors, &lang, theme_ts),
            Self::ConfigJwks => AdminConfigJwksHtml::build(&colors, &lang, theme_ts),
            Self::ConfigPolicy => AdminConfigPolicyHtml::build(&colors, &lang, theme_ts),
            Self::Device => DeviceHtml::build(&colors, &lang, theme_ts),
            Self::Docs => AdminDocsHtml::build(&colors, &lang, theme_ts),
            Self::FedCM => FedCMHtml::build(&colors, &lang, theme_ts),
            Self::Index => IndexHtml::build(&colors, &lang, theme_ts),
            Self::Logout(csrf_token) => LogoutHtml::build(csrf_token, &colors, &lang, theme_ts),
            Self::UserRegistration => UserRegisterHtml::build(&colors, &lang, theme_ts),
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
                .put_bytes(Cache::Html, cache_key, body_bytes.clone(), None)
                .await?;
        }

        Ok(HttpResponse::Ok()
            .insert_header(HEADER_HTML)
            .insert_header(("content-encoding", encoding))
            .body(body))
    }
}
