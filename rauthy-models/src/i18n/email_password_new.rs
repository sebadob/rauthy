use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailPasswordNew<'a> {
    pub subject: &'a str,
    pub header: &'a str,
    pub click_link: &'a str,
    pub validity: &'a str,
    pub expires: &'a str,
    pub button_text: &'a str,
}

impl SsrJson for I18nEmailPasswordNew<'_> {
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

impl I18nEmailPasswordNew<'_> {
    fn build_en() -> Self {
        Self {
            subject: "New Password",
            header: "New password for",
            click_link: "Click the link below to get forwarded to the password form.",
            validity: "This link is only valid for a short period of time for security reasons.",
            expires: "Link expires:",
            button_text: "Set Password",
        }
    }

    fn build_de() -> Self {
        Self {
            subject: "Neues Passwort",
            header: "Neues Passwort für",
            click_link: "Klicken Sie auf den unten stehenden Link um ein neues Passwort zu setzen.",
            validity: "Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig.",
            expires: "Link gültig bis:",
            button_text: "Passwort Setzen",
        }
    }
}
