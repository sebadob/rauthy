use crate::entity::auth_providers::AuthProviderTemplate;
use crate::entity::colors::Colors;
use crate::entity::password::PasswordPolicy;
use crate::entity::sessions::Session;
use crate::language::Language;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use rauthy_api_types::generic::PasswordPolicyResponse;
use rauthy_common::constants::{
    DEVICE_GRANT_USER_CODE_LENGTH, HEADER_HTML, OPEN_USER_REG, USER_REG_DOMAIN_RESTRICTION,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rinja_actix::Template;
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
            FrontendAction::MfaLogin(s) => write!(f, "MfaLogin {}", s),
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
// -> frontend/src/utils/constants.js -> TPL_* values
#[derive(Debug)]
pub enum HtmlTemplate {
    /// Auth providers as pre-built, cached JSON value
    AuthProviders(String),
    ClientName(String),
    ClientUrl(String),
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
}

impl HtmlTemplate {
    /// This function is only used during local dev to async resolve template values that are
    /// rendered into the HTML directly in prod.
    ///
    /// TODO maybe deactivate completely without debug_assertions?
    pub async fn build_from_str(s: &str, session: Option<Session>) -> Result<Self, ErrorResponse> {
        match s {
            "tpl_auth_providers" => {
                let json = AuthProviderTemplate::get_all_json_template().await?;
                Ok(Self::AuthProviders(json))
            }
            "tpl_csrf_token" => {
                if let Some(s) = session {
                    Ok(Self::CsrfToken(s.csrf_token))
                } else {
                    Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "no session",
                    ))
                }
            }
            "tpl_device_user_code_length" => {
                Ok(Self::DeviceUserCodeLength(*DEVICE_GRANT_USER_CODE_LENGTH))
            }
            "tpl_email_old" => Ok(Self::EmailOld("OLD@EMAIL.LOCAL".to_string())),
            "tpl_email_new" => Ok(Self::EmailOld("NEW@EMAIL.LOCAL".to_string())),
            "tpl_is_reg_open" => Ok(Self::IsRegOpen(*OPEN_USER_REG)),
            // the LoginAction requires a complex logic + validation.
            // Simply always return None during local dev.
            "tpl_login_action" => Ok(Self::LoginAction(FrontendAction::None)),
            // "tpl_client_name" => todo!("extract info from referrer?"),
            // "tpl_client_url" => todo!("extract info from referrer?"),
            "tpl_restricted_email_domain" => Ok(Self::RestrictedEmailDomain(
                USER_REG_DOMAIN_RESTRICTION.clone().unwrap_or_default(),
            )),
            "tpl_password_reset" => {
                if let Some(s) = session {
                    let password_policy = PasswordPolicy::find().await?;

                    // we can't easily return the complete and correct templates, but it should
                    // be good enough to be able to work with it
                    Ok(Self::PasswordReset(TplPasswordReset {
                        csrf_token: "CSRF_TOKEN_TEMPLATE".to_string(),
                        magic_link_id: "MAGIC_LINK:ID".to_string(),
                        needs_mfa: false,
                        password_policy: PasswordPolicyResponse::from(password_policy),
                        user_id: s.user_id.unwrap_or_default(),
                    }))
                } else {
                    Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "no session",
                    ))
                }
            }
            _ => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "invalid template id",
            )),
        }
    }

    /// Returns the `id` that will be used for the HTML `<template>` element.
    pub fn id(&self) -> &'static str {
        match self {
            Self::AuthProviders(_) => "tpl_auth_providers",
            Self::ClientName(_) => "tpl_client_name",
            Self::ClientUrl(_) => "tpl_client_url",
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
        }
    }

    // TODO find a way to borrow values dynamically, no matter the type
    // -> does rinja accept generic traits like `Display`?
    pub fn inner(&self) -> String {
        match self {
            Self::AuthProviders(i) => i.to_string(),
            Self::ClientName(i) => i.to_string(),
            Self::ClientUrl(i) => i.to_string(),
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
        }
    }
}

#[derive(Default, Template)]
#[template(path = "html/index.html")]
pub struct IndexHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl IndexHtml<'_> {
    pub fn build(colors: &Colors, lang: &Language) -> String {
        let res = IndexHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
            templates: &[HtmlTemplate::IsRegOpen(*OPEN_USER_REG)],
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/account.html")]
pub struct AccountHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AccountHtml<'_> {
    pub fn build(colors: &Colors, lang: &Language, templates: &[HtmlTemplate]) -> String {
        let res = AccountHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl DeviceHtml<'_> {
    pub fn build(colors: &Colors, lang: &Language) -> String {
        let res = DeviceHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
            templates: &[HtmlTemplate::DeviceUserCodeLength(
                *DEVICE_GRANT_USER_CODE_LENGTH,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl FedCMHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = FedCMHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl ErrorHtml<'_> {
    pub fn build<C>(
        colors: &Colors,
        lang: &Language,
        status_code: StatusCode,
        details_text: C,
    ) -> String
    where
        C: Into<Cow<'static, str>>,
    {
        let res = ErrorHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl Error1Html<'_> {
    pub fn build<C>(
        colors: &Colors,
        lang: &Language,
        status_code: StatusCode,
        details_text: C,
    ) -> String
    where
        C: Into<Cow<'static, str>>,
    {
        let res = Error1Html {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl Error2Html<'_> {
    pub fn build<C>(
        colors: &Colors,
        lang: &Language,
        status_code: StatusCode,
        details_text: C,
    ) -> String
    where
        C: Into<Cow<'static, str>>,
    {
        let res = Error2Html {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl Error3Html<'_> {
    pub fn build<C>(
        colors: &Colors,
        lang: &Language,
        status_code: StatusCode,
        details_text: C,
    ) -> String
    where
        C: Into<Cow<'static, str>>,
    {
        let res = Error3Html {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminApiKeysHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminApiKeysHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminAttributesHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminAttributesHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminBlacklistHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminBlacklistHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminClientsHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminClientsHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/config.html")]
pub struct AdminConfigHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminConfigHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminConfigHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminDocsHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminDocsHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminEventsHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminEventsHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminGroupsHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminGroupsHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminRolesHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminRolesHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminScopesHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminScopesHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminSessionsHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminSessionsHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AdminUsersHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = AdminUsersHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl AuthorizeHtml<'_> {
    pub fn build(colors: &Colors, lang: &Language, templates: &[HtmlTemplate]) -> String {
        let res = AuthorizeHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl CallbackHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = CallbackHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl crate::html_templates::ProvidersHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        let res = crate::html_templates::ProvidersHtml {
            lang: "en",
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl ProviderCallbackHtml<'_> {
    pub fn build(colors: &Colors, lang: &Language) -> String {
        let res = ProviderCallbackHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl LogoutHtml<'_> {
    pub fn build(csrf_token: String, colors: &Colors, lang: &Language) -> String {
        let res = LogoutHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl PwdResetHtml<'_> {
    // If the email is Some(_), this means that the user has webauthn enabled and does not need
    // to provide the email manually
    pub fn build(colors: &Colors, lang: &Language, template: TplPasswordReset) -> String {
        let res = PwdResetHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
            templates: &[HtmlTemplate::PasswordReset(template)],
        };

        res.render().unwrap()
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl UserEmailChangeConfirmHtml<'_> {
    pub fn build(colors: &Colors, lang: &Language, templates: &[HtmlTemplate]) -> String {
        UserEmailChangeConfirmHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
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
    col_act1: &'a str,
    col_act1a: &'a str,
    col_act2: &'a str,
    col_act2a: &'a str,
    col_acnt: &'a str,
    col_acnta: &'a str,
    col_ok: &'a str,
    col_err: &'a str,
    col_glow: &'a str,
    col_gmid: &'a str,
    col_ghigh: &'a str,
    col_text: &'a str,
    col_bg: &'a str,
    templates: &'a [HtmlTemplate],
}

impl UserRegisterHtml<'_> {
    pub fn build(colors: &Colors, lang: &Language) -> String {
        UserRegisterHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            col_act1: &colors.act1,
            col_act1a: &colors.act1a,
            col_act2: &colors.act2,
            col_act2a: &colors.act2a,
            col_acnt: &colors.acnt,
            col_acnta: &colors.acnta,
            col_ok: &colors.ok,
            col_err: &colors.err,
            col_glow: &colors.glow,
            col_gmid: &colors.gmid,
            col_ghigh: &colors.ghigh,
            col_text: &colors.text,
            col_bg: &colors.bg,
            templates: &[HtmlTemplate::RestrictedEmailDomain(
                USER_REG_DOMAIN_RESTRICTION.clone().unwrap_or_default(),
            )],
        }
        .render()
        .expect("rendering register.html")
    }
}
