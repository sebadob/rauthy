use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nRegister<'a> {
    domain_allowed: &'a str,
    domain_err: &'a str,
    domain_restricted: &'a str,
    email: &'a str,
    email_bad_format: &'a str,
    email_check: &'a str,
    family_name: &'a str,
    given_name: &'a str,
    regex_name: &'a str,
    register: &'a str,
    required: &'a str,
    success: &'a str,
    user_reg: &'a str,
}

impl SsrJson for I18nRegister<'_> {
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

impl I18nRegister<'_> {
    fn build_en() -> Self {
        Self {
            domain_allowed: "Allowed domain:",
            domain_err: "E-Mail domain not allowed",
            domain_restricted: "E-Mail domains are restricted",
            email: "E-Mail",
            email_bad_format: "Bad E-Mail format",
            email_check: "Please check your E-Mail inbox",
            family_name: "Family Name",
            given_name: "Given Name",
            regex_name: "Name with 2 - 32 non-special characters",
            register: "Register",
            required: "Required",
            success: "Registration successful",
            user_reg: "User Registration",
        }
    }

    fn build_de() -> Self {
        Self {
            domain_allowed: "Erlaubte Domain:",
            domain_err: "E-Mail Domain ist nicht erlaubt",
            domain_restricted: "E-Mail Domains sind beschränkt",
            email: "E-Mail",
            email_bad_format: "Ungültiges E-Mail Format",
            email_check: "Bitte prüfen Sie Ihren E-Mail Posteingang",
            family_name: "Nachname",
            given_name: "Vorname",
            regex_name: "Name mit 2 - 32 Buchstaben ohne Sonderzeichen",
            register: "Registrieren",
            required: "Notwendig",
            success: "Registrierung erfolgreich",
            user_reg: "Benutzer Registrierung",
        }
    }
}
