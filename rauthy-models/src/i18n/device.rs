use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nDevice<'a> {
    desc: &'a str,
    err_too_short: &'a str,
    err_too_long: &'a str,
    invalid_input: &'a str,
    submit: &'a str,
    title: &'a str,
    user_code: &'a str,
}

impl SsrJson for I18nDevice<'_> {
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

impl I18nDevice<'_> {
    fn build_en() -> Self {
        Self {
            desc: "Please enter the {{count}} character user code from your device.",
            err_too_short: "Input too short",
            err_too_long: "Input too long",
            invalid_input: "Invalid Input",
            submit: "Submit",
            title: "Device Authorization",
            user_code: "User Code",
        }
    }

    fn build_de() -> Self {
        Self {
            desc: "Bitte den {{count}}-stelligen vom Geräte angezeigten Benutzer Code eingeben.",
            err_too_short: "Eingabe zu kurz",
            err_too_long: "Eingabe zu lang",
            invalid_input: "Ungültige Eingabe",
            submit: "Absenden",
            title: "Gerät Authorisierung",
            user_code: "Benutzer Code",
        }
    }
}
