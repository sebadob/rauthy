use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailResetInfo<'a> {
    pub subject: &'a str,
    pub expires_1: &'a str,
    pub expires_2: &'a str,
    pub update: &'a str,
    pub button_text: &'a str,
}

impl SsrJson for I18nEmailResetInfo<'_> {
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

impl I18nEmailResetInfo<'_> {
    fn build_en() -> Self {
        Self {
            subject: "Password is about to expire",
            expires_1: "Your password for",
            expires_2: "is about to expire:",
            update: "You can update it here:",
            button_text: "Update Password",
        }
    }

    fn build_de() -> Self {
        Self {
            subject: "Passwort läuft demnächst ab",
            expires_1: "Ihr Passwort für",
            expires_2: " läuft demnächst ab:",
            update: "Sie können es hier erneuern:",
            button_text: "Passwort Erneuern",
        }
    }
}
