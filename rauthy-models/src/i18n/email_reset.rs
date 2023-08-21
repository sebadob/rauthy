use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailReset<'a> {
    pub subject: &'a str,
    pub header: &'a str,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
}

impl SsrJson for I18nEmailReset<'_> {
    fn build(lang: &Language) -> Self {
        match lang {
            Language::En => Self::build_en(),
            Language::De => Self::build_de(),
        }
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl I18nEmailReset<'_> {
    fn build_en() -> Self {
        Self {
            subject: "Password Reset Request",
            header: "Password reset request for",
            click_link: "Click the link below to get forwarded to the password request form.",
            validity: "This link is only valid for a short period of time for security reasons.",
            expires: "Link expires:",
            button_text: "Reset Password",
        }
    }

    fn build_de() -> Self {
        Self {
            subject: "Passwort Reset angefordert",
            header: "Passwort Reset angefordert für",
            click_link: "Klicken Sie auf den unten stehenden Link für den Passwort Reset.",
            validity: "Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig.",
            expires: "Link gültig bis:",
            button_text: "Passwort Zurücksetzen",
        }
    }
}
