use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nAuthorize<'a> {
    email: &'a str,
    email_bad_format: &'a str,
    email_required: &'a str,
    email_sent_msg: &'a str,
    invalid_credentials: &'a str,
    invalid_key_used: &'a str,
    login: &'a str,
    mfa_ack: &'a str,
    password: &'a str,
    password_forgotten: &'a str,
    password_request: &'a str,
    password_required: &'a str,
    provide_mfa: &'a str,
    request_expires: &'a str,
}

impl SsrJson for I18nAuthorize<'_> {
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

impl I18nAuthorize<'_> {
    fn build_en() -> Self {
        Self {
            email: "E-Mail",
            email_bad_format: "Bad E-Mail format",
            email_required: "E-Mail is required",
            email_sent_msg: "If your E-Mail exists, a request has been sent",
            invalid_credentials: "Invalid credentials",
            invalid_key_used: "Invalid Key",
            login: "Login",
            mfa_ack: "Acknowledged",
            password: "Password",
            password_forgotten: "Password forgotten?",
            password_request: "Request",
            password_required: "Password is required",
            provide_mfa: "Please login with your MFA device",
            request_expires: "Request expires",
        }
    }

    fn build_de() -> Self {
        Self {
            email: "E-Mail",
            email_bad_format: "Inkorrektes E-Mail Format",
            email_required: "E-Mail ist notwendig",
            email_sent_msg: "Sollte Ihre Adresse registriert sein, wurde eine Nachricht versandt",
            invalid_credentials: "Ungültige Zugangsdaten",
            invalid_key_used: "Ungültiger Sicherheitsschlüssel",
            login: "Login",
            mfa_ack: "Bestätigt",
            password: "Password",
            password_forgotten: "Password vergessen?",
            password_request: "Anfordern",
            password_required: "Password ist notwendig",
            provide_mfa: "Bitte stellen Sie Ihr MFA Gerät zur Verfügung",
            request_expires: "Anfrage läuft ab",
        }
    }
}
