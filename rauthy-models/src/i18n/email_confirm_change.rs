use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct I18nEmailConfirmChange<'a> {
    pub subject: &'a str,
    pub msg: &'a str,
}

impl SsrJson for I18nEmailConfirmChange<'_> {
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

impl I18nEmailConfirmChange<'_> {
    fn build_en() -> Self {
        Self {
            subject: "E-Mail Change confirmed for",
            msg: "Your E-Mail address has been changed successfully to:",
        }
    }

    fn build_de() -> Self {
        Self {
            subject: "E-Mail Wechsel bestätigt für",
            msg: "Ihre E-Mail Adresse wurde erfolgreich geändert zu:",
        }
    }
}
