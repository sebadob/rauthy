use crate::i18n::account::I18nAccountMfa;
use crate::i18n::password_policy::I18nPasswordPolicy;
use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nPasswordReset<'a> {
    password_policy: I18nPasswordPolicy<'a>,

    account_login: &'a str,
    bad_format: &'a str,
    email: &'a str,
    email_err: &'a str,
    fido_link: &'a str,
    generate: &'a str,
    mfa: I18nAccountMfa<'a>,
    new_acc_desc_1: &'a str,
    new_acc_desc_2: &'a str,
    new_account: &'a str,
    password: &'a str,
    passwordless: &'a str,
    password_confirm: &'a str,
    password_no_match: &'a str,
    required: &'a str,
    save: &'a str,
    success_1: &'a str,
    success_2: &'a str,
    success_3: &'a str,
    success_passkey_1: &'a str,
    success_passkey_2: &'a str,
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

            account_login: "Account Login",
            bad_format: "Bad Format",
            email: "E-Mail",
            email_err: "E-Mail does not match",
            fido_link: "https://fidoalliance.org/fido2",
            generate: "Generate",
            mfa: I18nAccountMfa::build_en(),
            new_acc_desc_1:
                "You have the option between two account types: passwordless or traditional password",
            new_acc_desc_2: r#"The passwordless account is always preferred, because it provides
a way stronger security. You will need at least one passkey (Yubikey, Apple Touch ID, Windows Hello,
...) to create such an account. Your device must embreace the Fido2 standard. For more information
about this, you may follow this link: "#,
            new_account: "New Account",
            password: "Password",
            passwordless: "FIDO Passkey",
            password_confirm: "Password Confirm",
            password_no_match: "Passwords do not match",
            required: "Required",
            save: "Save",
            success_1: "The password has been updated successfully.",
            success_2: "You will be redirected shortly.",
            success_3: "If you are not being redirected, please click here:",
            success_passkey_1: "Your new passkey has been registered successfully.",
            success_passkey_2: r#"Please log into your account and register a second backup key as
soon as possible. With a passkey only account, you wil not be able to use a password reset via
E-Mail in case you lose your current key."#,
        }
    }

    pub fn build_de() -> Self {
        Self {
            password_policy: I18nPasswordPolicy::build_de(),

            account_login: "Account Login",
            bad_format: "Ungültiges Format",
            email: "E-Mail",
            email_err: "E-Mail stimmt nicht überein",
            fido_link: "https://fidoalliance.org/fido2",
            generate: "Generieren",
            mfa: I18nAccountMfa::build_de(),
            new_acc_desc_1: r#"Sie haben die Option zwischen zwei Account Typen zu wählen: Passwortlos oder
traditionalles Passwort."#,
            new_acc_desc_2: r#"Der passwortlose Account Typ ist immer zu bevorzugen. Er bietet einen
sehr viel höheren Sicherheitsstandard als traditionelle Passwörter und gleichzeitig einen einfacheren
und schnelleren Login.
Dazu wird mindestens ein Passkey (Yubikey, Apple Touch ID, Windows Hello, ...) benötigt, welcher dem
FIDO2 Standard gerecht wird. Für weitere Informationen können Sie diesem Link folgen: "#,
            new_account: "Neuer Account",
            password: "Passwort",
            passwordless: "FIDO Passkey",
            password_confirm: "Passwort bestätigen",
            password_no_match: "Passwörter stimmen nicht überein",
            required: "Notwendig",
            save: "Speichern",
            success_1: "Das Passwort wurde erfolgreich zurückgesetzt.",
            success_2: "Sie werden in Kürze weitergeleitet.",
            success_3: "Sollte Sie nicht weitergeleitet werden, klicken Sie bitte hier:",
            success_passkey_1: "Der neue Passkey wurde erfolgreich registriert.",
            success_passkey_2: r#"Bitte loggen Sie sich direkt in Ihren Account ein und registrieren
Sie mindestens einen weiteren Backup Passkey. Ein passwortloser Account kann nicht den Passwort
Reset via E-Mail nutzen für den Fall, dass der derzeitige Passkey abhanden kommt."#,
        }
    }
}
