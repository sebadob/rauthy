use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nDevice<'a> {
    accept: &'a str,
    close_window: &'a str,
    decline: &'a str,
    desc: &'a str,
    desc_scopes: &'a str,
    err_too_short: &'a str,
    err_too_long: &'a str,
    invalid_input: &'a str,
    is_accepted: &'a str,
    is_declined: &'a str,
    submit: &'a str,
    title: &'a str,
    user_code: &'a str,
    wrong_or_expired: &'a str,
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
            accept: "Accept",
            close_window: "You can close this window now.",
            decline: "Decline",
            desc: "Please enter the {{count}} character user code from your device.",
            desc_scopes: "The device requests access to:",
            err_too_short: "Input too short",
            err_too_long: "Input too long",
            invalid_input: "Invalid Input",
            is_accepted: "The request has been accepted.",
            is_declined: "The request has been declined.",
            submit: "Submit",
            title: "Device Authorization",
            user_code: "User Code",
            wrong_or_expired: "Wrong or expired code",
        }
    }

    fn build_de() -> Self {
        Self {
            accept: "Akzeptieren",
            close_window: "Dieses Fenster kann nun geschlossen werden.",
            decline: "Ablehnen",
            desc: "Bitte den {{count}}-stelligen vom Geräte angezeigten Benutzer Code eingeben.",
            desc_scopes: "Das Gerät fragt Zugang an zu:",
            err_too_short: "Eingabe zu kurz",
            err_too_long: "Eingabe zu lang",
            invalid_input: "Ungültige Eingabe",
            is_accepted: "Die Anfrage wurde akzeptiert",
            is_declined: "Die Anfrage wurde abgewiesen",
            submit: "Absenden",
            title: "Gerät Authorisierung",
            user_code: "Benutzer Code",
            wrong_or_expired: "Ungültiger oder abgelaufener Code",
        }
    }
}
