use crate::entity::auth_providers::AuthProviderTemplate;
use crate::entity::colors::Colors;
use crate::entity::password::PasswordPolicy;
use crate::language::Language;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use rauthy_common::constants::{
    DEVICE_GRANT_USER_CODE_LENGTH, HEADER_HTML, OPEN_USER_REG, USER_REG_DOMAIN_RESTRICTION,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rinja_actix::Template;
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

// If you add new values to this template, make sure to also create a
// matching constant in the UI and make proper use of it:
// -> frontend/src/utils/constants.js -> TPL_* values
#[derive(Debug)]
pub enum HtmlTemplate {
    /// Auth providers as JSON value
    AuthProviders(String),
    ErrorDetails(Cow<'static, str>),
    ErrorText(Cow<'static, str>),
    StatusCode(StatusCode),
}

impl HtmlTemplate {
    pub async fn build_from_str(s: &str) -> Result<Self, ErrorResponse> {
        // TODO make sure to check the session in case of sensitive id's like csrf token

        match s {
            "auth_providers" => {
                let json = AuthProviderTemplate::get_all_json_template().await?;
                Ok(Self::AuthProviders(json))
            }
            _ => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "invalid template id",
            )),
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::AuthProviders(_) => "auth_providers",
            Self::ErrorDetails(_) => "error_details",
            Self::ErrorText(_) => "error_text",
            Self::StatusCode(_) => "status_code",
        }
    }

    pub fn inner(&self) -> &str {
        match self {
            Self::AuthProviders(s) => s,
            Self::ErrorDetails(s) => s.as_ref(),
            Self::ErrorText(s) => s.as_ref(),
            Self::StatusCode(s) => s.as_str(),
        }
    }
}

#[derive(Default, Template)]
#[template(path = "html/index.html")]
pub struct IndexHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    csrf_token: &'a str,
    data: &'a str,
    action: bool,
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
            csrf_token: "",
            data: &OPEN_USER_REG.to_string(),
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
#[template(path = "html/account.html")]
pub struct AccountHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
            ..Default::default()
        };
        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin.html")]
pub struct AdminHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: bool,
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
            csrf_token: "",
            data: &DEVICE_GRANT_USER_CODE_LENGTH.to_string(),
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
#[template(path = "html/fedcm.html")]
pub struct FedCMHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    csrf_token: &'a str,
    data: &'a str,
    action: bool,
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
            csrf_token: "",
            data: &DEVICE_GRANT_USER_CODE_LENGTH.to_string(),
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
    csrf_token: &'a str,
    data: &'a str,
    action: bool,
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
            ..Default::default()
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
    csrf_token: &'a str,
    data: &'a str,
    action: bool,
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
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Debug, Default, Template)]
#[template(path = "html/error/error/error.html")]
pub struct Error2Html<'a> {
    lang: &'a str,
    client_id: &'a str,
    csrf_token: &'a str,
    data: &'a str,
    action: bool,
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
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Debug, Default, Template)]
#[template(path = "html/error/error/error/error.html")]
pub struct Error3Html<'a> {
    lang: &'a str,
    client_id: &'a str,
    csrf_token: &'a str,
    data: &'a str,
    action: bool,
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
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/api_keys.html")]
pub struct AdminApiKeysHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    pub fn build(
        client_name: &Option<String>,
        csrf_token: &str,
        action: FrontendAction,
        colors: &Colors,
        lang: &Language,
        templates: &[HtmlTemplate],
    ) -> String {
        let mut res = AuthorizeHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            csrf_token,
            action: &action.to_string(),
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
            ..Default::default()
        };

        if client_name.is_some() {
            res.data = client_name.as_ref().unwrap();
        }

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/oidc/callback.html")]
pub struct CallbackHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: &'a str,
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
    csrf_token: &'a str,
    data: &'a str,
    action: bool,
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
    pub fn build(csrf_token: &str, set_logout: bool, colors: &Colors, lang: &Language) -> String {
        let res = LogoutHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            csrf_token,
            action: set_logout,
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
#[template(path = "html/users/{id}/reset/reset.html")]
pub struct PwdResetHtml<'a> {
    lang: &'a str,
    client_id: &'a str,
    csrf_token: &'a str,
    data: &'a str,
    action: bool,
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
    pub fn build(
        csrf_token: &str,
        password_rules: &PasswordPolicy,
        colors: &Colors,
        lang: &Language,
        needs_mfa: bool,
    ) -> String {
        let data = format!(
            "{},{},{},{},{},{},{},{}",
            password_rules.length_min,
            password_rules.length_max,
            password_rules.include_lower_case.unwrap_or(-1),
            password_rules.include_upper_case.unwrap_or(-1),
            password_rules.include_digits.unwrap_or(-1),
            password_rules.include_special.unwrap_or(-1),
            password_rules.not_recently_used.unwrap_or(-1),
            needs_mfa,
        );

        let res = PwdResetHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            csrf_token,
            data: &data,
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
    csrf_token: &'a str,
    data: &'a str,
    action: bool,
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
    pub fn build(colors: &Colors, lang: &Language, email_old: &str, email_new: &str) -> String {
        let data = format!("{},{}", email_old, email_new,);

        UserEmailChangeConfirmHtml {
            lang: lang.as_str(),
            client_id: "rauthy",
            data: &data,
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
    csrf_token: &'a str,
    data: &'a str,
    action: bool,
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
            data: USER_REG_DOMAIN_RESTRICTION
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or(""),
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
        }
        .render()
        .expect("rendering register.html")
    }
}
