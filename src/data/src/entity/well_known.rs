use crate::database::{Cache, DB};
use crate::entity::scopes::Scope;
use crate::language::Language;
use crate::rauthy_config::RauthyConfig;
use rauthy_common::constants::{CACHE_TTL_APP, GRANT_TYPE_DEVICE_CODE};
use rauthy_error::ErrorResponse;
use serde::Serialize;
use strum::IntoEnumIterator;
use utoipa::ToSchema;

/// The struct for the `.well-known` endpoint for automatic OIDC discovery.
///
/// An update on this struct needs an update in `src/bin/tests/handler_generic.rs` as well.
#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct WellKnown {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub backchannel_logout_supported: bool,
    pub backchannel_logout_session_supported: bool,
    pub device_authorization_endpoint: String,
    pub token_endpoint: String,
    pub introspection_endpoint: String,
    pub userinfo_endpoint: String,
    pub end_session_endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_endpoint: Option<String>,
    pub jwks_uri: String,
    pub grant_types_supported: [&'static str; 5],
    pub response_types_supported: [&'static str; 1],
    pub subject_types_supported: [&'static str; 1],
    pub id_token_signing_alg_values_supported: [&'static str; 4],
    pub token_endpoint_auth_methods_supported: [&'static str; 2],
    pub token_endpoint_auth_signing_alg_values_supported: [&'static str; 4],
    pub claims_supported: [&'static str; 12],
    pub claim_types_supported: [&'static str; 3],
    pub scopes_supported: Vec<String>,
    pub code_challenge_methods_supported: [&'static str; 2],
    pub dpop_signing_alg_values_supported: [&'static str; 4],
    pub service_documentation: &'static str,
    pub ui_locales_supported: Vec<&'static str>,
    pub claims_parameter_supported: bool,
}

static IDX: &str = ".well-known";

impl WellKnown {
    pub async fn json() -> Result<String, ErrorResponse> {
        let client = DB::hql();
        if let Some(slf) = client.get(Cache::App, IDX).await? {
            return Ok(slf);
        }

        let scopes = Scope::find_all()
            .await?
            .into_iter()
            .map(|s| s.name)
            .collect::<Vec<String>>();
        let slf = Self::new(scopes);
        let json = serde_json::to_string(&slf)?;

        client.put(Cache::App, IDX, &json, CACHE_TTL_APP).await?;

        Ok(json)
    }

    /// Rebuilds the WellKnown, serializes it as json and updates it inside the cache.
    /// Should be called after any update on the Scopes.
    pub async fn rebuild() -> Result<(), ErrorResponse> {
        let scopes = Scope::find_all()
            .await?
            .into_iter()
            .map(|s| s.name)
            .collect::<Vec<String>>();
        let slf = Self::new(scopes);
        let json = serde_json::to_string(&slf)?;

        DB::hql().put(Cache::App, IDX, &json, CACHE_TTL_APP).await?;

        Ok(())
    }
}

impl WellKnown {
    pub fn new(scopes_supported: Vec<String>) -> Self {
        let issuer = &RauthyConfig::get().issuer;

        let authorization_endpoint = format!("{issuer}/oidc/authorize");
        let device_authorization_endpoint = format!("{issuer}/oidc/device");
        let token_endpoint = format!("{issuer}/oidc/token");
        let introspection_endpoint = format!("{issuer}/oidc/introspect");
        let userinfo_endpoint = format!("{issuer}/oidc/userinfo");
        let registration_endpoint = RauthyConfig::get()
            .vars
            .dynamic_clients
            .enable
            .then_some(format!("{issuer}/clients_dyn"));
        let end_session_endpoint = format!("{issuer}/oidc/logout");
        let jwks_uri = format!("{issuer}/oidc/certs");

        WellKnown {
            issuer: String::from(issuer),
            authorization_endpoint,
            backchannel_logout_supported: true,
            backchannel_logout_session_supported: true,
            device_authorization_endpoint,
            token_endpoint,
            introspection_endpoint,
            userinfo_endpoint,
            end_session_endpoint,
            registration_endpoint,
            jwks_uri,
            grant_types_supported: [
                "authorization_code",
                "client_credentials",
                "password",
                "refresh_token",
                GRANT_TYPE_DEVICE_CODE,
            ],
            response_types_supported: ["code"],
            subject_types_supported: ["public"],
            id_token_signing_alg_values_supported: ["RS256", "RS384", "RS512", "EdDSA"],
            token_endpoint_auth_methods_supported: ["client_secret_post", "client_secret_basic"],
            token_endpoint_auth_signing_alg_values_supported: ["RS256", "RS384", "RS512", "EdDSA"],
            claims_supported: [
                "iss",
                "azp",
                "amr",
                "sub",
                "preferred_username",
                "email",
                "email_verified",
                "given_name",
                "family_name",
                "roles",
                "groups",
                "custom",
            ],
            claim_types_supported: ["normal", "aggregated", "distributed"],
            scopes_supported,
            code_challenge_methods_supported: ["plain", "S256"],
            dpop_signing_alg_values_supported: ["RS256", "RS384", "RS512", "EdDSA"],
            service_documentation: "https://sebadob.github.io/rauthy/",
            ui_locales_supported: Language::iter().map(|l| l.as_str()).collect(),
            claims_parameter_supported: true,
        }
    }
}
