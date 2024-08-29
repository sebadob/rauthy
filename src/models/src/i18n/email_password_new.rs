use crate::i18n::SsrJson;
use crate::language::Language;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::env;

static TPL_EN_PASSWORD_NEW_SUBJECT: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_PASSWORD_NEW_SUBJECT").ok());
static TPL_EN_PASSWORD_NEW_HEADER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_PASSWORD_NEW_HEADER").ok());
static TPL_EN_PASSWORD_NEW_TEXT: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_PASSWORD_NEW_TEXT").ok());
static TPL_EN_PASSWORD_NEW_CLICK_LINK: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_PASSWORD_NEW_CLICK_LINK").ok());
static TPL_EN_PASSWORD_NEW_VALIDITY: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_PASSWORD_NEW_VALIDITY").ok());
static TPL_EN_PASSWORD_NEW_EXPIRES: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_PASSWORD_NEW_EXPIRES").ok());
static TPL_EN_PASSWORD_NEW_BUTTON: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_PASSWORD_NEW_BUTTON").ok());
static TPL_EN_PASSWORD_NEW_FOOTER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_PASSWORD_NEW_FOOTER").ok());

static TPL_DE_PASSWORD_NEW_SUBJECT: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_PASSWORD_NEW_SUBJECT").ok());
static TPL_DE_PASSWORD_NEW_HEADER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_PASSWORD_NEW_HEADER").ok());
static TPL_DE_PASSWORD_NEW_TEXT: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_PASSWORD_NEW_TEXT").ok());
static TPL_DE_PASSWORD_NEW_CLICK_LINK: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_PASSWORD_NEW_CLICK_LINK").ok());
static TPL_DE_PASSWORD_NEW_VALIDITY: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_PASSWORD_NEW_VALIDITY").ok());
static TPL_DE_PASSWORD_NEW_EXPIRES: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_PASSWORD_NEW_EXPIRES").ok());
static TPL_DE_PASSWORD_NEW_BUTTON: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_PASSWORD_NEW_BUTTON").ok());
static TPL_DE_PASSWORD_NEW_FOOTER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_PASSWORD_NEW_FOOTER").ok());

static TPL_ZH_HANS_PASSWORD_NEW_SUBJECT: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_PASSWORD_NEW_SUBJECT").ok());
static TPL_ZH_HANS_PASSWORD_NEW_HEADER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_PASSWORD_NEW_HEAZH_HANSR").ok());
static TPL_ZH_HANS_PASSWORD_NEW_TEXT: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_PASSWORD_NEW_TEXT").ok());
static TPL_ZH_HANS_PASSWORD_NEW_CLICK_LINK: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_PASSWORD_NEW_CLICK_LINK").ok());
static TPL_ZH_HANS_PASSWORD_NEW_VALIDITY: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_PASSWORD_NEW_VALIDITY").ok());
static TPL_ZH_HANS_PASSWORD_NEW_EXPIRES: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_PASSWORD_NEW_EXPIRES").ok());
static TPL_ZH_HANS_PASSWORD_NEW_BUTTON: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_PASSWORD_NEW_BUTTON").ok());
static TPL_ZH_HANS_PASSWORD_NEW_FOOTER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_PASSWORD_NEW_FOOTER").ok());

#[derive(Debug, Serialize)]
pub struct I18nEmailPasswordNew<'a> {
    pub subject: &'a str,
    pub header: &'a str,
    pub text: Option<&'a str>,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
    pub footer: Option<&'a str>,
}

impl SsrJson for I18nEmailPasswordNew<'_> {
    fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(),
            Language::De => Self::build_de(),
            Language::ZhHans => Self::build_zh_hans(),
        }
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl I18nEmailPasswordNew<'_> {
    fn build_en() -> Self {
        Self {
            subject: TPL_EN_PASSWORD_NEW_SUBJECT
                .as_deref()
                .unwrap_or("New Password"),
            header: TPL_EN_PASSWORD_NEW_HEADER
                .as_deref()
                .unwrap_or("New password for"),
            text: TPL_EN_PASSWORD_NEW_TEXT.as_deref(),
            click_link: TPL_EN_PASSWORD_NEW_CLICK_LINK
                .as_deref()
                .unwrap_or("Click the link below to get forwarded to the password form."),
            validity: TPL_EN_PASSWORD_NEW_VALIDITY.as_deref().unwrap_or(
                "This link is only valid for a short period of time for security reasons.",
            ),
            expires: TPL_EN_PASSWORD_NEW_EXPIRES
                .as_deref()
                .unwrap_or("Link expires:"),
            button_text: TPL_EN_PASSWORD_NEW_BUTTON
                .as_deref()
                .unwrap_or("Set Password"),
            footer: TPL_EN_PASSWORD_NEW_FOOTER.as_deref(),
        }
    }

    fn build_de() -> Self {
        Self {
            subject: TPL_DE_PASSWORD_NEW_SUBJECT
                .as_deref()
                .unwrap_or("Neues Passwort"),
            header: TPL_DE_PASSWORD_NEW_HEADER
                .as_deref()
                .unwrap_or("Neues Passwort für"),
            text: TPL_DE_PASSWORD_NEW_TEXT.as_deref(),
            click_link: TPL_DE_PASSWORD_NEW_CLICK_LINK.as_deref().unwrap_or(
                "Klicken Sie auf den unten stehenden Link um ein neues Passwort zu setzen.",
            ),
            validity: TPL_DE_PASSWORD_NEW_VALIDITY
                .as_deref()
                .unwrap_or("Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig."),
            expires: TPL_DE_PASSWORD_NEW_EXPIRES
                .as_deref()
                .unwrap_or("Link gültig bis:"),
            button_text: TPL_DE_PASSWORD_NEW_BUTTON
                .as_deref()
                .unwrap_or("Passwort Setzen"),
            footer: TPL_DE_PASSWORD_NEW_FOOTER.as_deref(),
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            subject: TPL_ZH_HANS_PASSWORD_NEW_SUBJECT
                .as_deref()
                .unwrap_or("新密码"),
            header: TPL_ZH_HANS_PASSWORD_NEW_HEADER
                .as_deref()
                .unwrap_or("新密码："),
            text: TPL_ZH_HANS_PASSWORD_NEW_TEXT.as_deref(),
            click_link: TPL_ZH_HANS_PASSWORD_NEW_CLICK_LINK
                .as_deref()
                .unwrap_or("点击下方链接以打开密码设置表单。"),
            validity: TPL_ZH_HANS_PASSWORD_NEW_VALIDITY
                .as_deref()
                .unwrap_or("出于安全考虑，此链接仅在短时间内有效。"),
            expires: TPL_ZH_HANS_PASSWORD_NEW_EXPIRES
                .as_deref()
                .unwrap_or("链接过期时间："),
            button_text: TPL_ZH_HANS_PASSWORD_NEW_BUTTON
                .as_deref()
                .unwrap_or("设置密码"),
            footer: TPL_ZH_HANS_PASSWORD_NEW_FOOTER.as_deref(),
        }
    }
}
