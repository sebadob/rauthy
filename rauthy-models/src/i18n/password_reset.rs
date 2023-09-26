use crate::i18n::password_policy::I18nPasswordPolicy;
use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nPasswordReset<'a> {
    password_policy: I18nPasswordPolicy<'a>,

    bad_format: &'a str,
    email: &'a str,
    generate: &'a str,
    password: &'a str,
    password_confirm: &'a str,
    password_no_match: &'a str,
    required: &'a str,
    save: &'a str,
    success_1: &'a str,
    success_2: &'a str,
}

impl SsrJson for I18nPasswordReset<'_> {
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

impl I18nPasswordReset<'_> {
    pub fn build_en() -> Self {
        Self {
            password_policy: I18nPasswordPolicy::build_en(),

            bad_format: "Bad Format",
            email: "E-Mail",
            generate: "Generate",
            password: "Password",
            password_confirm: "Password Confirm",
            password_no_match: "Passwords do not match",
            required: "Required",
            save: "Save",
            success_1: "The password has been updated successfully.",
            success_2: "You will be redirected to your account shortly.",
        }
    }

    pub fn build_de() -> Self {
        Self {
            password_policy: I18nPasswordPolicy::build_de(),

            bad_format: "Ungültiges Format",
            email: "E-Mail",
            generate: "Generieren",
            password: "Passwort",
            password_confirm: "Passwort bestätigen",
            password_no_match: "Passwörter stimmen nicht überein",
            required: "Notwendig",
            save: "Speichern",
            success_1: "Das Passwort wurde erfolgreich zurückgesetzt.",
            success_2: "Sie werden in Kürze zu Ihrem Account weitergeleitet.",
        }
    }
}
