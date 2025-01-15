use crate::language::Language;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::env;

static TPL_EN_RESET_SUBJECT: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_RESET_SUBJECT").ok());
static TPL_EN_RESET_HEADER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_RESET_HEADER").ok());
static TPL_EN_RESET_TEXT: Lazy<Option<String>> = Lazy::new(|| env::var("TPL_EN_RESET_TEXT").ok());
static TPL_EN_RESET_CLICK_LINK: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_RESET_CLICK_LINK").ok());
static TPL_EN_RESET_VALIDITY: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_RESET_VALIDITY").ok());
static TPL_EN_RESET_EXPIRES: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_RESET_EXPIRES").ok());
static TPL_EN_RESET_BUTTON: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_RESET_BUTTON").ok());
static TPL_EN_RESET_FOOTER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_EN_RESET_FOOTER").ok());

static TPL_DE_RESET_SUBJECT: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_RESET_SUBJECT").ok());
static TPL_DE_RESET_HEADER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_RESET_HEADER").ok());
static TPL_DE_RESET_TEXT: Lazy<Option<String>> = Lazy::new(|| env::var("TPL_DE_RESET_TEXT").ok());
static TPL_DE_RESET_CLICK_LINK: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_RESET_CLICK_LINK").ok());
static TPL_DE_RESET_VALIDITY: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_RESET_VALIDITY").ok());
static TPL_DE_RESET_EXPIRES: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_RESET_EXPIRES").ok());
static TPL_DE_RESET_BUTTON: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_RESET_BUTTON").ok());
static TPL_DE_RESET_FOOTER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_DE_RESET_FOOTER").ok());

static TPL_ZH_HANS_RESET_SUBJECT: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_RESET_SUBJECT").ok());
static TPL_ZH_HANS_RESET_HEADER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_RESET_HEADER").ok());
static TPL_ZH_HANS_RESET_TEXT: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_RESET_TEXT").ok());
static TPL_ZH_HANS_RESET_CLICK_LINK: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_RESET_CLICK_LINK").ok());
static TPL_ZH_HANS_RESET_VALIDITY: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_RESET_VALIDITY").ok());
static TPL_ZH_HANS_RESET_EXPIRES: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_RESET_EXPIRES").ok());
static TPL_ZH_HANS_RESET_BUTTON: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_RESET_BUTTON").ok());
static TPL_ZH_HANS_RESET_FOOTER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_ZH_HANS_RESET_FOOTER").ok());

static TPL_KO_RESET_SUBJECT: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_KO_RESET_SUBJECT").ok());
static TPL_KO_RESET_HEADER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_KO_RESET_HEADER").ok());
static TPL_KO_RESET_TEXT: Lazy<Option<String>> = Lazy::new(|| env::var("TPL_KO_RESET_TEXT").ok());
static TPL_KO_RESET_CLICK_LINK: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_KO_RESET_CLICK_LINK").ok());
static TPL_KO_RESET_VALIDITY: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_KO_RESET_VALIDITY").ok());
static TPL_KO_RESET_EXPIRES: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_KO_RESET_EXPIRES").ok());
static TPL_KO_RESET_BUTTON: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_KO_RESET_BUTTON").ok());
static TPL_KO_RESET_FOOTER: Lazy<Option<String>> =
    Lazy::new(|| env::var("TPL_KO_RESET_FOOTER").ok());

#[derive(Debug, Serialize)]
pub struct I18nEmailReset<'a> {
    pub subject: &'a str,
    pub header: &'a str,
    pub text: Option<&'a str>,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
    pub footer: Option<&'a str>,
}

impl I18nEmailReset<'_> {
    pub fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(),
            Language::De => Self::build_de(),
            Language::ZhHans => Self::build_zh_hans(),
            Language::Ko => Self::build_ko(),
        }
    }
}

impl I18nEmailReset<'_> {
    fn build_en() -> Self {
        Self {
            subject: TPL_EN_RESET_SUBJECT
                .as_deref()
                .unwrap_or("Password Reset Request"),
            header: TPL_EN_RESET_HEADER
                .as_deref()
                .unwrap_or("Password reset request for"),
            text: TPL_EN_RESET_TEXT.as_deref(),
            click_link: TPL_EN_RESET_CLICK_LINK
                .as_deref()
                .unwrap_or("Click the link below to get forwarded to the password request form."),
            validity: TPL_EN_RESET_VALIDITY.as_deref().unwrap_or(
                "This link is only valid for a short period of time for security reasons.",
            ),
            expires: TPL_EN_RESET_EXPIRES.as_deref().unwrap_or("Link expires:"),
            button_text: TPL_EN_RESET_BUTTON.as_deref().unwrap_or("Reset Password"),
            footer: TPL_EN_RESET_FOOTER.as_deref(),
        }
    }

    fn build_de() -> Self {
        Self {
            subject: TPL_DE_RESET_SUBJECT
                .as_deref()
                .unwrap_or("Passwort Reset angefordert"),
            header: TPL_DE_RESET_HEADER
                .as_deref()
                .unwrap_or("Passwort Reset angefordert für"),
            text: TPL_DE_RESET_TEXT.as_deref(),
            click_link: TPL_DE_RESET_CLICK_LINK
                .as_deref()
                .unwrap_or("Klicken Sie auf den unten stehenden Link für den Passwort Reset."),
            validity: TPL_DE_RESET_VALIDITY
                .as_deref()
                .unwrap_or("Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig."),
            expires: TPL_DE_RESET_EXPIRES
                .as_deref()
                .unwrap_or("Link gültig bis:"),
            button_text: TPL_DE_RESET_BUTTON
                .as_deref()
                .unwrap_or("Passwort Zurücksetzen"),
            footer: TPL_DE_RESET_FOOTER.as_deref(),
        }
    }

    fn build_zh_hans() -> Self {
        Self {
            subject: TPL_ZH_HANS_RESET_SUBJECT
                .as_deref()
                .unwrap_or("密码重置请求"),
            header: TPL_ZH_HANS_RESET_HEADER
                .as_deref()
                .unwrap_or("密码重置请求："),
            text: TPL_ZH_HANS_RESET_TEXT.as_deref(),
            click_link: TPL_ZH_HANS_RESET_CLICK_LINK
                .as_deref()
                .unwrap_or("点击下方链接以打开密码重置表单。"),
            validity: TPL_ZH_HANS_RESET_VALIDITY
                .as_deref()
                .unwrap_or("出于安全考虑，此链接仅在短时间内有效。"),
            expires: TPL_ZH_HANS_RESET_EXPIRES
                .as_deref()
                .unwrap_or("链接过期时间"),
            button_text: TPL_ZH_HANS_RESET_BUTTON.as_deref().unwrap_or("重置密码"),
            footer: TPL_ZH_HANS_RESET_FOOTER.as_deref(),
        }
    }

    fn build_ko() -> Self {
        Self {
            subject: TPL_KO_RESET_SUBJECT
                .as_deref()
                .unwrap_or("비밀번호 초기화 요청"),
            header: TPL_KO_RESET_HEADER
                .as_deref()
                .unwrap_or("비밀번호 초기화 요청:"),
            text: TPL_KO_RESET_TEXT.as_deref(),
            click_link: TPL_KO_RESET_CLICK_LINK
                .as_deref()
                .unwrap_or("비밀번호 초기화 요청 창으로 이동하려면, 아래의 링크를 클릭해 주세요."),
            validity: TPL_KO_RESET_VALIDITY
                .as_deref()
                .unwrap_or("이 링크는 보안상의 이유로 짧은 시간 동안에만 유효합니다."),
            expires: TPL_KO_RESET_EXPIRES.as_deref().unwrap_or("링크 만료일:"),
            button_text: TPL_KO_RESET_BUTTON.as_deref().unwrap_or("비밀번호 초기화"),
            footer: TPL_KO_RESET_FOOTER.as_deref(),
        }
    }
}
