use crate::i18n::password_policy::I18nPasswordPolicy;
use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nAccount<'a> {
    account: &'a str,
    change_password: &'a str,
    created_at: &'a str,
    email: &'a str,
    email_verified: &'a str,
    family_name: &'a str,
    generate_random: &'a str,
    given_name: &'a str,
    groups: &'a str,
    last_login: &'a str,
    logout: &'a str,
    mfa: I18nAccountMfa<'a>,
    mfa_activated: &'a str,
    password_expiry: &'a str,
    password_policy: I18nPasswordPolicy<'a>,
    roles: &'a str,
    save: &'a str,
    user: &'a str,
}

impl SsrJson for I18nAccount<'_> {
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

impl I18nAccount<'_> {
    fn build_en() -> Self {
        Self {
            account: "Account",
            change_password: "Change Password",
            created_at: "created at",
            email: "E-Mail",
            email_verified: "E-Mail verified",
            family_name: "Family Name",
            generate_random: "Generate Random",
            given_name: "Given Name",
            groups: "Groups",
            last_login: "Last Login",
            logout: "Logout",
            mfa: I18nAccountMfa::build_en(),
            mfa_activated: "MFA activated",
            password_expiry: "Password expiry",
            password_policy: I18nPasswordPolicy::build_en(),
            roles: "Roles",
            save: "Save",
            user: "User",
        }
    }

    fn build_de() -> Self {
        Self {
            account: "Account",
            change_password: "Passwort wechseln",
            created_at: "erstellt am",
            email: "E-Mail",
            email_verified: "E-Mail verifiziert",
            family_name: "Nachname",
            generate_random: "Zufällig generiert",
            given_name: "Vorname",
            groups: "Gruppen",
            last_login: "Letzter Login",
            logout: "Logout",
            mfa: I18nAccountMfa::build_de(),
            mfa_activated: "MFA aktiviert",
            password_expiry: "Passwort Ablauf",
            password_policy: I18nPasswordPolicy::build_de(),
            roles: "Rollen",
            save: "Speichern",
            user: "Benutzer",
        }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nAccountMfa<'a> {
    p_1: &'a str,
    p_2: &'a str,
    p_3: &'a str,
    p_4: &'a str,
    no_key: &'a str,
    register: &'a str,
    test: &'a str,
}

impl SsrJson for I18nAccountMfa<'_> {
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

impl I18nAccountMfa<'_> {
    fn build_en() -> Self {
        Self {
            p_1: "If you plan on using your MFA key with multiple Browsers like Firefox and a \
            Chrome based Browsers, you should do the registration with Firefox only.",
            p_2: "Firefox does sometimes not ask for an additional PIN, if the device supports this \
            method for compatibility reasons. Chrome bases variants on the other hand do always try \
            to use an additional PIN confirmation, if it exists. ",
            p_3: "If a device, that supports a PIN, was registered with a Chrome based browser, it \
            will not work in Firefox at the time of writing. If it was registered with Firefox \
            though, it will work with Chrome too. ",
            p_4: "You can register two Keys for your account.",
            no_key: "No Security key registered on this slot",
            register: "Register",
            test: "Test",
        }
    }

    fn build_de() -> Self {
        Self {
            account: "Account",
            change_password: "Passwort wechseln",
            created_at: "erstellt am",
            email: "E-Mail",
            email_verified: "E-Mail verifiziert",
            family_name: "Nachname",
            generate_random: "Zufällig generiert",
            given_name: "Vorname",
            groups: "Gruppen",
            last_login: "Letzter Login",
            mfa_activated: "MFA aktiviert",
            password_expiry: "Passwort Ablauf",
            password_policy: I18nPasswordPolicy::build(&Language::De),
            roles: "Rollen",
            save: "Speichern",
            user: "Benutzer",
        }
    }
}
