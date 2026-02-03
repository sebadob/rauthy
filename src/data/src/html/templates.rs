use crate::language::Language;
use crate::rauthy_config::RauthyConfig;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use askama::Template;
use rauthy_api_types::generic::PasswordPolicyResponse;
use rauthy_common::constants::HEADER_HTML;
use serde::Serialize;
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub enum FrontendAction {
    Refresh,
    MfaLogin(String),
    None,
}

impl Display for FrontendAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontendAction::Refresh => write!(f, "Refresh"),
            FrontendAction::MfaLogin(s) => write!(f, "MfaLogin {s}"),
            FrontendAction::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TplClientData {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct TplPasswordReset {
    pub csrf_token: String,
    pub magic_link_id: String,
    pub needs_mfa: bool,
    pub password_policy: PasswordPolicyResponse,
    pub user_id: String,
}

// If you add new values to this template, make sure to also create a
// matching constant in the UI and make proper use of it:
// -> frontend/src/utils/constants.ts -> TPL_* values
#[derive(Debug)]
pub enum HtmlTemplate {
    AdminButtonHide(bool),
    AtprotoId(String),
    /// Auth providers as pre-built, cached JSON value
    AuthProviders(String),
    ClientName(String),
    ClientUrl(String),
    ClientLogoUpdated(Option<i64>),
    CsrfToken(String),
    EmailOld(String),
    EmailNew(String),
    ErrorDetails(Cow<'static, str>),
    ErrorText(Cow<'static, str>),
    DeviceUserCodeLength(u8),
    IsRegOpen(bool),
    LoginAction(FrontendAction),
    PasswordReset(TplPasswordReset),
    RestrictedEmailDomain(String),
    StatusCode(StatusCode),
    UserValues,
}

impl HtmlTemplate {
    /// Returns the `id` that will be used for the HTML `<template>` element.
    pub fn id(&self) -> &'static str {
        match self {
            Self::AdminButtonHide(_) => "tpl_admin_btn_hide",
            Self::AtprotoId(_) => "tpl_atproto_id",
            Self::AuthProviders(_) => "tpl_auth_providers",
            Self::ClientName(_) => "tpl_client_name",
            Self::ClientUrl(_) => "tpl_client_url",
            Self::ClientLogoUpdated(_) => "tpl_client_logo_updated",
            Self::CsrfToken(_) => "tpl_csrf_token",
            Self::EmailOld(_) => "tpl_email_old",
            Self::EmailNew(_) => "tpl_email_new",
            Self::ErrorDetails(_) => "tpl_error_details",
            Self::ErrorText(_) => "tpl_error_text",
            Self::DeviceUserCodeLength(_) => "tpl_device_user_code_length",
            Self::IsRegOpen(_) => "tpl_is_reg_open",
            Self::LoginAction(_) => "tpl_login_action",
            Self::PasswordReset(_) => "tpl_password_reset",
            Self::RestrictedEmailDomain(_) => "tpl_restricted_email_domain",
            Self::StatusCode(_) => "tpl_status_code",
            Self::UserValues => "tpl_user_values_config",
        }
    }

    // TODO find a way to borrow values dynamically, no matter the type
    //  -> does askama accept generic traits like `Display`?
    pub fn inner(&self) -> String {
        match self {
            Self::AdminButtonHide(i) => i.to_string(),
            Self::AtprotoId(i) => i.to_string(),
            Self::AuthProviders(i) => i.to_string(),
            Self::ClientName(i) => i.to_string(),
            Self::ClientUrl(i) => i.to_string(),
            Self::ClientLogoUpdated(i) => i.map(|i| i.to_string()).unwrap_or_default(),
            Self::CsrfToken(i) => i.to_string(),
            Self::EmailOld(i) => i.to_string(),
            Self::EmailNew(i) => i.to_string(),
            Self::ErrorDetails(i) => i.to_string(),
            Self::ErrorText(i) => i.to_string(),
            Self::DeviceUserCodeLength(i) => i.to_string(),
            Self::IsRegOpen(i) => i.to_string(),
            Self::LoginAction(i) => i.to_string(),
            Self::PasswordReset(i) => serde_json::to_string(i).unwrap(),
            Self::StatusCode(i) => i.to_string(),
            Self::RestrictedEmailDomain(i) => i.to_string(),
            Self::UserValues => {
                serde_json::to_string(&RauthyConfig::get().vars.user_values).unwrap()
            }
        }
    }
}

#[derive(Default, Template)]
#[template(path = "html/index.html")]
pub struct IndexHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl IndexHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = IndexHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates: &[
                HtmlTemplate::AdminButtonHide(RauthyConfig::get().vars.access.admin_button_hide),
                HtmlTemplate::IsRegOpen(RauthyConfig::get().vars.user_registration.enable),
            ],
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/account.html")]
pub struct AccountHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AccountHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64, templates: &[HtmlTemplate]) -> String {
        let res = AccountHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates,
        };
        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin.html")]
pub struct AdminHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/device.html")]
pub struct DeviceHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl DeviceHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = DeviceHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates: &[HtmlTemplate::DeviceUserCodeLength(
                RauthyConfig::get().vars.device_grant.user_code_length as u8,
            )],
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/fedcm.html")]
pub struct FedCMHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl FedCMHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = FedCMHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Debug, Default, Template)]
#[template(path = "html/error.html")]
pub struct ErrorHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl ErrorHtml<'_> {
    pub fn build<C>(
        lang: &Language,
        theme_ts: i64,
        status_code: StatusCode,
        details_text: C,
    ) -> String
    where
        C: Into<Cow<'static, str>>,
    {
        let res = ErrorHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates: &[
                HtmlTemplate::StatusCode(status_code),
                HtmlTemplate::ErrorText(details_text.into()),
            ],
        };

        res.render().unwrap()
    }

    pub fn response(body: String, status_code: StatusCode) -> HttpResponse {
        HttpResponseBuilder::new(status_code)
            .insert_header(HEADER_HTML)
            .body(body)
    }
}

// The error template is defined 3 more times.
// This may look a bit ugly here in the code, but is actually better for the efficiency and
// performance down the road. The same error template is being pre-compiled 4 times with just
// slightly different resource links. This makes it possible to just server the correct error
// page in every location without the need to dynamically modify the path for each render.

#[derive(Debug, Default, Template)]
#[template(path = "html/error/error.html")]
pub struct Error1Html<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl Error1Html<'_> {
    pub fn build<C>(
        lang: &Language,
        theme_ts: i64,
        status_code: StatusCode,
        details_text: C,
    ) -> String
    where
        C: Into<Cow<'static, str>>,
    {
        let res = Error1Html {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates: &[
                HtmlTemplate::StatusCode(status_code),
                HtmlTemplate::ErrorText(details_text.into()),
            ],
        };

        res.render().unwrap()
    }
}

#[derive(Debug, Default, Template)]
#[template(path = "html/error/error/error.html")]
pub struct Error2Html<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl Error2Html<'_> {
    pub fn build<C>(
        lang: &Language,
        theme_ts: i64,
        status_code: StatusCode,
        details_text: C,
    ) -> String
    where
        C: Into<Cow<'static, str>>,
    {
        let res = Error2Html {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates: &[
                HtmlTemplate::StatusCode(status_code),
                HtmlTemplate::ErrorText(details_text.into()),
            ],
        };

        res.render().unwrap()
    }
}

#[derive(Debug, Default, Template)]
#[template(path = "html/error/error/error/error.html")]
pub struct Error3Html<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl Error3Html<'_> {
    pub fn build<C>(
        lang: &Language,
        theme_ts: i64,
        status_code: StatusCode,
        details_text: C,
    ) -> String
    where
        C: Into<Cow<'static, str>>,
    {
        let res = Error3Html {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates: &[
                HtmlTemplate::StatusCode(status_code),
                HtmlTemplate::ErrorText(details_text.into()),
            ],
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/api_keys.html")]
pub struct AdminApiKeysHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminApiKeysHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminApiKeysHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/attributes.html")]
pub struct AdminAttributesHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminAttributesHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminAttributesHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/blacklist.html")]
pub struct AdminBlacklistHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminBlacklistHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminBlacklistHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/clients.html")]
pub struct AdminClientsHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminClientsHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminClientsHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/config/argon2.html")]
pub struct AdminConfigArgon2Html<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminConfigArgon2Html<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminConfigArgon2Html {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/config/backups.html")]
pub struct AdminConfigBackupsHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminConfigBackupsHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminConfigBackupsHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/config/encryption.html")]
pub struct AdminConfigEncryptionHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminConfigEncryptionHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminConfigEncryptionHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/config/jwks.html")]
pub struct AdminConfigJwksHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminConfigJwksHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminConfigJwksHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/config/policy.html")]
pub struct AdminConfigPolicyHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminConfigPolicyHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminConfigPolicyHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/docs.html")]
pub struct AdminDocsHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminDocsHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminDocsHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/events.html")]
pub struct AdminEventsHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminEventsHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminEventsHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/groups.html")]
pub struct AdminGroupsHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminGroupsHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminGroupsHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/roles.html")]
pub struct AdminRolesHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminRolesHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminRolesHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/scopes.html")]
pub struct AdminScopesHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminScopesHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminScopesHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/pam.html")]
pub struct AdminPAMHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminPAMHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminPAMHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/sessions.html")]
pub struct AdminSessionsHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminSessionsHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminSessionsHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/users.html")]
pub struct AdminUsersHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AdminUsersHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = AdminUsersHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/oidc/authorize.html")]
pub struct AuthorizeHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl AuthorizeHtml<'_> {
    pub fn build(
        lang: &Language,
        client_id: &str,
        theme_ts: i64,
        templates: &[HtmlTemplate],
    ) -> String {
        let res = AuthorizeHtml {
            lang: lang.as_str(),
            client_id,
            theme_ts,
            templates,
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/oidc/callback.html")]
pub struct CallbackHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl CallbackHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = CallbackHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/providers.html")]
pub struct ProvidersHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl ProvidersHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = ProvidersHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/providers/callback.html")]
pub struct ProviderCallbackHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl ProviderCallbackHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        let res = ProviderCallbackHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/oidc/logout.html")]
pub struct LogoutHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl LogoutHtml<'_> {
    pub fn build(csrf_token: String, lang: &Language, theme_ts: i64) -> String {
        let res = LogoutHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates: &[HtmlTemplate::CsrfToken(csrf_token)],
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/users/{id}/reset/reset.html")]
pub struct PwdResetHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl PwdResetHtml<'_> {
    // If the email is Some(_), this means that the user has webauthn enabled and does not need
    // to provide the email manually
    pub fn build(lang: &Language, theme_ts: i64, template: TplPasswordReset) -> String {
        let res = PwdResetHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates: &[HtmlTemplate::PasswordReset(template)],
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "error/blocked.html")]
pub struct BlockedHtml;

impl BlockedHtml {
    pub fn build() -> String {
        BlockedHtml.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "error/429.html")]
pub struct TooManyRequestsHtml {
    pub ip: String,
    pub exp: i64,
}

impl TooManyRequestsHtml {
    pub fn build(ip: String, exp: i64) -> String {
        TooManyRequestsHtml { ip, exp }.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/users/{id}/email_confirm/email_confirm.html")]
pub struct UserEmailChangeConfirmHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl UserEmailChangeConfirmHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64, templates: &[HtmlTemplate]) -> String {
        UserEmailChangeConfirmHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates,
        }
        .render()
        .expect("rendering email_confirm.html")
    }
}

#[derive(Default, Template)]
#[template(path = "html/users/register.html")]
pub struct UserRegisterHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl UserRegisterHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64, auth_providers: HtmlTemplate) -> String {
        UserRegisterHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates: &[
                HtmlTemplate::RestrictedEmailDomain(
                    RauthyConfig::get()
                        .vars
                        .user_registration
                        .domain_restriction
                        .clone()
                        .unwrap_or_default(),
                ),
                HtmlTemplate::UserValues,
                auth_providers,
            ],
        }
        .render()
        .expect("rendering register.html")
    }
}

#[derive(Default, Template)]
#[template(path = "html/users/{id}/revoke/revoke.html")]
pub struct UserRevokeHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl UserRevokeHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64) -> String {
        UserRevokeHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates: &[],
        }
        .render()
        .expect("rendering revoke.html")
    }
}

#[derive(Default, Template)]
#[template(path = "html/users/password_reset.html")]
pub struct UserPasswordResetHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    theme_ts: i64,
    templates: &'a [HtmlTemplate],
}

impl UserPasswordResetHtml<'_> {
    pub fn build(lang: &Language, theme_ts: i64, logo_updated: Option<i64>) -> String {
        UserPasswordResetHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            theme_ts,
            templates: &[HtmlTemplate::ClientLogoUpdated(logo_updated)],
        }
        .render()
        .expect("rendering password_reset.html")
    }
}
