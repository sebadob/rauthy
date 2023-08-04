use crate::entity::colors::Colors;
use crate::entity::password::PasswordPolicy;
use crate::i18n::account::I18nAccount;
use crate::i18n::authorize::I18nAuthorize;
use crate::i18n::SsrJson;
use crate::language::Language;
use askama_actix::Template;
use rauthy_common::constants::{OPEN_USER_REG, USER_REG_DOMAIN_RESTRICTION};
use rauthy_common::utils::get_rand;

#[derive(Debug, Clone)]
pub enum FrontendAction {
    Refresh,
    MfaLogin(String),
    None,
}

impl ToString for FrontendAction {
    fn to_string(&self) -> String {
        match self {
            FrontendAction::Refresh => "Refresh".to_string(),
            FrontendAction::MfaLogin(s) => format!("MfaLogin {}", s),
            FrontendAction::None => "None".to_string(),
        }
    }
}

#[derive(Default, Template)]
#[template(path = "html/index.html")]
pub struct IndexHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: bool,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl IndexHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = IndexHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/account.html")]
pub struct AccountHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AccountHtml<'_> {
    pub fn build(colors: &Colors, lang: &Language) -> (String, String) {
        let nonce = nonce();

        let res = AccountHtml {
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
            nonce: &nonce,
            i18n: I18nAccount::build(lang).as_json(),
            ..Default::default()
        };
        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin.html")]
pub struct AdminHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AdminHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = AdminHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/attributes.html")]
pub struct AdminAttributesHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AdminAttributesHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = AdminAttributesHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/clients.html")]
pub struct AdminClientsHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AdminClientsHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = AdminClientsHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/config.html")]
pub struct AdminConfigHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AdminConfigHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = AdminConfigHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/docs.html")]
pub struct AdminDocsHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AdminDocsHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = AdminDocsHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/groups.html")]
pub struct AdminGroupsHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AdminGroupsHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = AdminGroupsHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/roles.html")]
pub struct AdminRolesHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AdminRolesHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = AdminRolesHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/scopes.html")]
pub struct AdminScopesHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AdminScopesHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = AdminScopesHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/sessions.html")]
pub struct AdminSessionsHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AdminSessionsHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = AdminSessionsHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/admin/users.html")]
pub struct AdminUsersHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AdminUsersHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = AdminUsersHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/oidc/authorize.html")]
pub struct AuthorizeHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl AuthorizeHtml<'_> {
    pub fn build(
        client_name: &Option<String>,
        csrf_token: &str,
        action: FrontendAction,
        colors: &Colors,
        lang: &Language,
    ) -> (String, String) {
        let nonce = nonce();

        let mut res = AuthorizeHtml {
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
            nonce: &nonce,
            i18n: I18nAuthorize::build(lang).as_json(),
            ..Default::default()
        };
        if client_name.is_some() {
            res.data = client_name.as_ref().unwrap();
        }

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/oidc/callback.html")]
pub struct CallbackHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: &'a str,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl CallbackHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = CallbackHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/oidc/logout.html")]
pub struct LogoutHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: bool,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl LogoutHtml<'_> {
    pub fn build(csrf_token: &str, set_logout: bool, colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let res = LogoutHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/users/{id}/reset/reset.html")]
pub struct PwdResetHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: bool,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl PwdResetHtml<'_> {
    // If the email is Some(_), this means that the user has webauthn enabled and does not need
    // to provide the email manually
    pub fn build(
        csrf_token: &str,
        password_rules: &PasswordPolicy,
        email: Option<&String>,
        colors: &Colors,
    ) -> (String, String) {
        let nonce = nonce();

        let mail = if let Some(e) = email { e } else { "undefined" };
        let data = format!(
            "{},{},{},{},{},{},{},{}",
            password_rules.length_min,
            password_rules.length_max,
            password_rules.include_lower_case.unwrap_or(-1),
            password_rules.include_upper_case.unwrap_or(-1),
            password_rules.include_digits.unwrap_or(-1),
            password_rules.include_special.unwrap_or(-1),
            password_rules.not_recently_used.unwrap_or(-1),
            mail,
        );

        let res = PwdResetHtml {
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
            nonce: &nonce,
            ..Default::default()
        };

        (res.render().unwrap(), nonce)
    }
}

#[derive(Default, Template)]
#[template(path = "html/users/register.html")]
pub struct UserRegisterHtml<'a> {
    pub csrf_token: &'a str,
    pub data: &'a str,
    pub action: bool,
    pub col_act1: &'a str,
    pub col_act1a: &'a str,
    pub col_act2: &'a str,
    pub col_act2a: &'a str,
    pub col_acnt: &'a str,
    pub col_acnta: &'a str,
    pub col_ok: &'a str,
    pub col_err: &'a str,
    pub col_glow: &'a str,
    pub col_gmid: &'a str,
    pub col_ghigh: &'a str,
    pub col_text: &'a str,
    pub col_bg: &'a str,
    pub nonce: &'a str,
    pub i18n: String,
}

impl UserRegisterHtml<'_> {
    pub fn build(colors: &Colors) -> (String, String) {
        let nonce = nonce();

        let body = UserRegisterHtml {
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
            nonce: &nonce,
            ..Default::default()
        }
        .render()
        .expect("rendering register.html");

        (body, nonce)
    }
}

fn nonce() -> String {
    get_rand(16)
}
