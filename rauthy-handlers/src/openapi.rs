use crate::{
    api_keys, blacklist, clients, events, generic, groups, oidc, roles, scopes, sessions, users,
};
use actix_web::web;
use rauthy_common::constants::{PROXY_MODE, RAUTHY_VERSION};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::events::event;
use rauthy_models::language;
use rauthy_models::ListenScheme;
use rauthy_models::{entity, request, response};
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

        generic::get_auth_check,
        generic::get_auth_check_admin,
        generic::get_enc_keys,
        generic::post_migrate_enc_key,
        generic::get_login_time,
        generic::post_password_hash_times,
        generic::get_password_policy,
        generic::put_password_policy,
        generic::get_health,
        generic::get_pow,
        generic::get_ready,
        generic::ping,
        generic::whoami,
        generic::get_version,

        groups::get_groups,
        groups::post_group,
        groups::put_group,
        groups::delete_group,

        oidc::get_authorize,
        oidc::post_authorize,
        oidc::get_certs,
        oidc::get_cert_by_kid,
        oidc::get_logout,
        oidc::post_logout,
        oidc::rotate_jwk,
        oidc::get_session_info,
        oidc::get_session_xsrf,
        oidc::post_token,
        oidc::post_token_info,
        oidc::post_validate_token,
        oidc::get_userinfo,
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
            entity::api_keys::AccessGroup,
            entity::api_keys::AccessRights,
            entity::api_keys::ApiKeyAccess,
            entity::auth_providers::AuthProviderType,
            entity::clients::Client,
            entity::colors::Colors,
            entity::groups::Group,
            entity::jwk::JwkKeyPairAlg,
            entity::jwk::JwkKeyPairType,
            entity::password::PasswordHashTime,
            entity::password::PasswordHashTimes,
            entity::roles::Role,
            entity::scopes::Scope,
            entity::sessions::SessionState,
            entity::user_attr::UserAttrConfigEntity,
            entity::user_attr::UserAttrValueEntity,
            entity::webauthn::WebauthnAdditionalData,
            entity::webauthn::WebauthnLoginReq,
            entity::webauthn::WebauthnServiceReq,
            entity::well_known::WellKnown,
            entity::webids::WebId,

            event::EventLevel,
            ErrorResponse,
            ErrorResponseType,
            language::Language,

            request::ApiKeyRequest,
            request::AuthCodeRequest,
            request::AuthRequest,
            request::IpBlacklistRequest,
            request::ColorsRequest,
            request::EncKeyMigrateRequest,
            request::LoginRequest,
            request::LogoutRequest,
            request::MfaAwaitRequest,
            request::MfaPurpose,
            request::NewClientRequest,
            request::DynamicClientRequest,
            request::NewGroupRequest,
            request::PasswordHashTimesRequest,
            request::PasswordPolicyRequest,
            request::PasswordResetRequest,
            request::ProviderRequest,
            request::ProviderLoginRequest,
            request::ProviderLookupRequest,
            request::RequestResetRequest,
            request::NewUserRequest,
            request::NewUserRegistrationRequest,
            request::NewRoleRequest,
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
            request::WhoamiRequestParam,
            request::WhoamiRequestParams,

            response::ApiKeyResponse,
            response::ApiKeysResponse,
            response::AppVersionResponse,
            response::BlacklistResponse,
            response::BlacklistedIp,
            response::LoginTimeResponse,
            response::ClientResponse,
            response::DynamicClientResponse,
            response::ClientSecretResponse,
            response::EncKeysResponse,
            response::HealthResponse,
            response::JWKSCerts,
            response::JWKSPublicKeyCerts,
            response::Argon2ParamsResponse,
            response::PasswordPolicyResponse,
            response::ProviderResponse,
            response::ProviderLookupResponse,
            response::ScopeResponse,
            response::SessionResponse,
            response::SessionInfoResponse,
            response::TokenInfo,
            response::UserAttrConfigResponse,
            response::UserAttrValueResponse,
            response::UserAttrValuesResponse,
            response::Userinfo,
            response::UserValuesResponse,
            response::UserAccountTypeResponse,
            response::UserResponse,
            response::WebauthnAuthStartResponse,
            response::WebauthnLoginFinishResponse,
            response::WebauthnLoginResponse,
            response::WebIdResponse,

            rauthy_models::AddressClaim,
            rauthy_models::JktClaim,
            rauthy_models::JwtTokenType,
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
        (name = "health", description = "Ping, Health, Ready Check"),
        (name = "blacklist", description = "IP Blacklist endpoints"),
        (name = "api_keys", description = "API Keys endpoints"),
        (name = "generic", description = "Generic endpoints"),
        (name = "webid", description = "WebID endpoints"),
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
        // contact.email = Some("".to_string());
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
