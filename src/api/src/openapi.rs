use crate::{
    api_keys, auth_providers, blacklist, clients, events, fed_cm, generic, groups, oidc, roles,
    scopes, sessions, users,
};
use actix_web::web;
use rauthy_api_types::{request, response};
use rauthy_common::constants::{PROXY_MODE, RAUTHY_VERSION};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity;
use rauthy_models::ListenScheme;
use rauthy_service::token_set;
use utoipa::openapi::Server;
use utoipa::{openapi, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        api_keys::get_api_keys,
        api_keys::post_api_key,
        api_keys::put_api_key,
        api_keys::delete_api_key,
        api_keys::get_api_key_test,
        api_keys::put_api_key_secret,

        auth_providers::post_providers,
        auth_providers::post_provider,
        auth_providers::post_provider_lookup,
        auth_providers::post_provider_login,
        auth_providers::post_provider_callback,
        auth_providers::delete_provider_link,
        auth_providers::get_providers_minimal,
        auth_providers::put_provider,
        auth_providers::delete_provider,
        auth_providers::get_provider_delete_safe,
        auth_providers::get_provider_img,
        auth_providers::put_provider_img,

        blacklist::get_blacklist,
        blacklist::post_blacklist,
        blacklist::delete_blacklist,

        clients::get_clients,
        clients::get_client_by_id,
        clients::get_client_colors,
        clients::put_client_colors,
        clients::delete_client_colors,
        clients::get_client_logo,
        clients::put_client_logo,
        clients::delete_client_logo,
        clients::get_client_secret,
        clients::post_clients,
        clients::put_clients,
        clients::put_generate_client_secret,
        clients::delete_client,

        events::sse_events,
        events::post_event_test,

        fed_cm::get_fed_cm_accounts,
        fed_cm::get_fed_cm_client_meta,
        fed_cm::get_fed_cm_config,
        fed_cm::get_fed_cm_status,
        fed_cm::post_fed_cm_token,
        fed_cm::get_fed_cm_well_known,

        generic::get_auth_check,
        generic::get_auth_check_admin,
        generic::get_enc_keys,
        generic::post_migrate_enc_key,
        generic::get_login_time,
        generic::post_password_hash_times,
        generic::get_password_policy,
        generic::put_password_policy,
        generic::get_health,
        generic::post_pow,
        generic::get_ready,
        generic::ping,
        generic::get_version,
        generic::get_whoami,

        groups::get_groups,
        groups::post_group,
        groups::put_group,
        groups::delete_group,

        oidc::get_authorize,
        oidc::post_authorize,
        oidc::get_certs,
        oidc::get_cert_by_kid,
        oidc::post_device_auth,
        oidc::get_logout,
        oidc::post_logout,
        oidc::rotate_jwk,
        oidc::post_session,
        oidc::get_session_info,
        oidc::get_session_xsrf,
        oidc::post_token,
        oidc::post_token_info,
        oidc::post_validate_token,
        oidc::get_userinfo,
        oidc::get_forward_auth,
        oidc::get_well_known,

        roles::get_roles,
        roles::post_role,
        roles::put_role,
        roles::delete_role,

        scopes::get_scopes,
        scopes::post_scope,
        scopes::put_scope,
        scopes::delete_scope,

        sessions::get_sessions,
        sessions::delete_sessions,
        sessions::delete_sessions_for_user,

        users::get_users,
        users::post_users,
        users::get_cust_attr,
        users::post_cust_attr,
        users::put_cust_attr,
        users::delete_cust_attr,
        users::get_users_register,
        users::post_users_register,
        users::get_user_by_id,
        users::get_user_attr,
        users::put_user_attr,
        users::get_user_webid,
        users::get_user_webid_data,
        users::put_user_webid_data,
        users::get_user_password_reset,
        users::put_user_password_reset,
        users::post_webauthn_auth_start,
        users::post_webauthn_auth_finish,
        users::delete_webauthn,
        users::post_webauthn_reg_start,
        users::post_user_password_request_reset,
        users::get_user_by_email,
        users::put_user_by_id,
        users::put_user_self,
        users::post_user_self_convert_passkey,
        users::delete_user_by_id,
    ),
    components(
        schemas(
            entity::colors::Colors,
            entity::fed_cm::FedCMAccount,
            entity::fed_cm::FedCMAccounts,
            entity::fed_cm::FedCMIdPBranding,
            entity::fed_cm::FedCMIdPConfig,
            entity::fed_cm::FedCMIdPIcon,
            entity::fed_cm::WebIdentity,
            entity::groups::Group,
            entity::password::PasswordHashTime,
            entity::password::PasswordHashTimes,
            entity::roles::Role,
            entity::scopes::Scope,
            entity::sessions::SessionState,
            entity::webauthn::WebauthnAdditionalData,
            entity::webauthn::WebauthnLoginReq,
            entity::webauthn::WebauthnServiceReq,
            entity::well_known::WellKnown,

            rauthy_api_types::AccessGroup,
            rauthy_api_types::AccessRights,
            rauthy_api_types::AddressClaim,
            rauthy_api_types::ApiKeyAccess,
            rauthy_api_types::AuthProviderType,
            rauthy_api_types::EventLevel,
            rauthy_api_types::JktClaim,
            rauthy_api_types::JwkKeyPairAlg,
            rauthy_api_types::JwkKeyPairType,
            rauthy_api_types::Language,

            rauthy_models::JwtTokenType,

            ErrorResponse,
            ErrorResponseType,

            request::ApiKeyRequest,
            request::AuthCodeRequest,
            request::AuthRequest,
            request::IpBlacklistRequest,
            request::ColorsRequest,
            request::DeviceGrantRequest,
            request::EncKeyMigrateRequest,
            request::FedCMAssertionRequest,
            request::FedCMClientMetadataRequest,
            request::LoginRequest,
            request::LogoutRequest,
            request::MfaAwaitRequest,
            request::MfaPurpose,
            request::NewClientRequest,
            request::DynamicClientRequest,
            request::NewGroupRequest,
            request::NewUserRequest,
            request::NewUserRegistrationRequest,
            request::NewRoleRequest,
            request::PaginationParams,
            request::PasswordHashTimesRequest,
            request::PasswordPolicyRequest,
            request::PasswordResetRequest,
            request::ProviderRequest,
            request::ProviderLoginRequest,
            request::ProviderLookupRequest,
            request::ProviderCallbackRequest,
            request::RequestResetRequest,
            request::ScopeRequest,
            request::TokenRequest,
            request::TokenValidationRequest,
            request::UpdateClientRequest,
            request::UpdateUserRequest,
            request::UpdateUserSelfRequest,
            request::UserValuesRequest,
            request::UserAttrConfigRequest,
            request::UserAttrValueRequest,
            request::UserAttrValuesUpdateRequest,
            request::WebauthnRegStartRequest,
            request::WebauthnRegFinishRequest,
            request::WebauthnAuthStartRequest,
            request::WebauthnAuthFinishRequest,
            request::WebIdRequest,

            response::ApiKeyResponse,
            response::ApiKeysResponse,
            response::AppVersionResponse,
            response::BlacklistResponse,
            response::BlacklistedIp,
            response::CsrfTokenResponse,
            response::LoginTimeResponse,
            response::ClientResponse,
            response::DeviceCodeResponse,
            response::DynamicClientResponse,
            response::ClientSecretResponse,
            response::EncKeysResponse,
            response::HealthResponse,
            response::JWKSCerts,
            response::JWKSPublicKeyCerts,
            response::Argon2ParamsResponse,
            response::OAuth2ErrorResponse,
            response::OAuth2ErrorTypeResponse,
            response::PasswordPolicyResponse,
            response::ProviderResponse,
            response::ProviderLinkedUserResponse,
            response::ProviderLookupResponse,
            response::ScopeResponse,
            response::SessionResponse,
            response::SessionInfoResponse,
            response::TokenInfo,
            response::UserAttrConfigResponse,
            response::UserAttrConfigValueResponse,
            response::UserAttrValueResponse,
            response::UserAttrValuesResponse,
            response::Userinfo,
            response::UserValuesResponse,
            response::UserAccountTypeResponse,
            response::UserResponse,
            response::WebauthnAuthStartResponse,
            response::WebauthnLoginFinishResponse,
            response::WebauthnLoginResponse,
            response::WebId,
            response::WebIdResponse,

            token_set::TokenSet,
        ),
    ),
    tags(
        (name = "oidc", description = "OpenID Connect endpoints"),
        (name = "clients", description = "OIDC Clients"),
        (name = "users", description = "Users endpoints"),
        (name = "mfa", description = "MFA endpoints"),
        (name = "sessions", description = "Sessions endpoints"),
        (name = "groups", description = "Groups endpoints"),
        (name = "roles", description = "Roles endpoints"),
        (name = "scopes", description = "Scopes endpoints"),
        (name = "events", description = "Events Stream"),
        (name = "providers", description = "Upstream Auth Providers"),
        (name = "health", description = "Ping, Health, Ready Check"),
        (name = "blacklist", description = "IP Blacklist endpoints"),
        (name = "api_keys", description = "API Keys endpoints"),
        (name = "generic", description = "Generic endpoints"),
        (name = "webid", description = "WebID endpoints"),
        (name = "fed_cm", description = "Experimental FedCM endpoints"),
        (name = "deprecated", description = "Deprecated endpoints - will be removed in a future version"),
    ),
)]

pub struct ApiDoc;

impl ApiDoc {
    pub fn build(app_state: &web::Data<AppState>) -> openapi::OpenApi {
        let mut doc = Self::openapi();

        doc.info = openapi::Info::new("Rauthy Single Sign-on", &format!("v{}", RAUTHY_VERSION));

        // let desc = r#""#;
        // doc.info.description = Some(desc.to_string());

        // let mut contact = Contact::new();
        // contact.name = Some("".to_string());
        // contact.url = Some("".to_string());
        // contact.email = Some(ADMIN);
        // doc.info.contact = Some(contact);

        let scheme = if !*PROXY_MODE && app_state.listen_scheme == ListenScheme::Http {
            "http://"
        } else {
            "https://"
        };

        // let scheme = if app_state.listen_scheme == ListenScheme::Http {
        //     "http://"
        // } else {
        //     "https://"
        // };
        let pub_url = &app_state.public_url;
        let url = format!("{}{}/auth/v1", scheme, pub_url);
        doc.servers = Some(vec![Server::new(url)]);

        doc
    }
}
