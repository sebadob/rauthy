use crate::entity::colors::Colors;
use crate::entity::password::PasswordPolicy;
use askama_actix::Template;
use rauthy_common::constants::{OPEN_USER_REG, USER_REG_DOMAIN_RESTRICTION};

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
}

impl IndexHtml<'_> {
    pub fn build(colors: &Colors) -> String {
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
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/authorize.html")]
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
}

impl AuthorizeHtml<'_> {
    pub fn build(
        client_name: &Option<String>,
        csrf_token: &str,
        action: FrontendAction,
        colors: &Colors,
    ) -> String {
        // pub fn build(client_name: &Option<String>, csrf_token: &str, set_refresh: bool) -> String {
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
            ..Default::default()
        };
        if client_name.is_some() {
            res.data = client_name.as_ref().unwrap();
        }

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/logout.html")]
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
}

impl LogoutHtml<'_> {
    pub fn build(csrf_token: &str, set_logout: bool, colors: &Colors) -> String {
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
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/reset.html")]
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
}

impl PwdResetHtml<'_> {
    // If the email is Some(_), this means that the user has webauthn enabled and does not need
    // to provide the email manually
    pub fn build(
        csrf_token: &str,
        password_rules: &PasswordPolicy,
        email: Option<&String>,
        colors: &Colors,
    ) -> String {
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
            ..Default::default()
        };

        res.render().unwrap()
    }
}

#[derive(Default, Template)]
#[template(path = "html/register.html")]
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
}

impl UserRegisterHtml<'_> {
    pub fn build(colors: &Colors) -> String {
        UserRegisterHtml {
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
