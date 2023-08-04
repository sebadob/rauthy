use crate::i18n::password_policy::I18nPasswordPolicy;
use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nAccount<'a> {
    account: &'a str,
    change_password: &'a str,
    created: &'a str,
    email: &'a str,
    email_verified: &'a str,
    enabled: &'a str,
    family_name: &'a str,
    generate_random: &'a str,
    given_name: &'a str,
    groups: &'a str,
    invalid_input: &'a str,
    last_login: &'a str,
    mfa: I18nAccountMfa<'a>,
    mfa_activated: &'a str,
    nav_info: &'a str,
    nav_edit: &'a str,
    nav_mfa: &'a str,
    nav_logout: &'a str,
    never: &'a str,
    password_confirm: &'a str,
    password_curr: &'a str,
    password_curr_req: &'a str,
    password_new: &'a str,
    password_new_req: &'a str,
    password_no_match: &'a str,
    password_expiry: &'a str,
    password_policy: I18nPasswordPolicy<'a>,
    password_policy_follow: &'a str,
    roles: &'a str,
    save: &'a str,
    user: &'a str,
    valid_email: &'a str,
    valid_given_name: &'a str,
    valid_family_name: &'a str,
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
            created: "created",
            email: "E-Mail",
            email_verified: "E-Mail verified",
            enabled: "Enabled",
            family_name: "Family Name",
            generate_random: "Generate Random",
            given_name: "Given Name",
            groups: "Groups",
            invalid_input: "Invalid Input",
            last_login: "Last Login",
            mfa: I18nAccountMfa::build_en(),
            mfa_activated: "MFA activated",
            nav_info: "Info",
            nav_edit: "Edit",
            nav_mfa: "MFA",
            nav_logout: "Logout",
            never: "Never",
            password_confirm: "Confirm Password",
            password_curr: "Current Password",
            password_curr_req: "Current password is required",
            password_new: "New Password",
            password_new_req: "New password is required",
            password_no_match: "Password verification is required",
            password_expiry: "Password expiry",
            password_policy: I18nPasswordPolicy::build_en(),
            password_policy_follow: "You must follow the password policy",
            roles: "Roles",
            save: "Save",
            user: "User",
            valid_email: "Valid E-Mail format",
            valid_given_name: "Your given name with 2 - 32 non-special characters",
            valid_family_name: "Your family name with 2 - 32 non-special characters",
        }
    }

    fn build_de() -> Self {
        Self {
            account: "Account",
            change_password: "Passwort wechseln",
            created: "erstellt",
            email: "E-Mail",
            email_verified: "E-Mail verifiziert",
            enabled: "Aktiviert",
            family_name: "Nachname",
            generate_random: "Zufällig generiert",
            given_name: "Vorname",
            groups: "Gruppen",
            invalid_input: "Ungültige Eingaben",
            last_login: "Letzter Login",
            mfa: I18nAccountMfa::build_de(),
            mfa_activated: "MFA aktiviert",
            nav_info: "Info",
            nav_edit: "Editieren",
            nav_mfa: "MFA",
            nav_logout: "Logout",
            never: "Niemals",
            password_confirm: "Passwort bestätigen",
            password_curr: "Derzeitiges Passwort",
            password_curr_req: "Derzeitiges Passwort ist notwendig",
            password_new: "Neues Passwort",
            password_new_req: "Neues Passwort ist notwendig",
            password_no_match: "Passwörter stimmen nicht überein",
            password_expiry: "Passwort Ablauf",
            password_policy: I18nPasswordPolicy::build_de(),
            password_policy_follow: "Befolgen Sie die Passwort Regeln",
            roles: "Rollen",
            save: "Speichern",
            user: "Benutzer",
            valid_email: "Gültige E-Mail Adresse",
            valid_given_name: "Vorname, 2 - 32 Buchstaben, keine Sonderzeichen",
            valid_family_name: "Nachname, 2 - 32  Buchstaben, keine Sonderzeichen",
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

    delete: &'a str,
    error_reg: &'a str,
    invalid_key_used: &'a str,
    no_key: &'a str,
    register: &'a str,
    security_key: &'a str,
    test: &'a str,
    test_error: &'a str,
    test_success: &'a str,
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

            delete: "Delete",
            error_reg: "Error starting the Registration process",
            invalid_key_used: "Invalid Key used",
            no_key: "No Security key registered on this slot",
            register: "Register",
            security_key: "Security Key",
            test: "Test",
            test_error: "Error starting the Test",
            test_success: "Test successful",
        }
    }

    fn build_de() -> Self {
        Self {
            p_1: "Wenn Sie mehrere Browser parallel nutzen möchten, wie z.B. Chrome und Firefox, \
            sollten Sie die Registrierung mit Firefox durchführen.",
            p_2: "Unter bestimmten Bedingungen (z.B. Mac OS) verlangt Firefox (und Andere) derzeit \
            aus Kompatibilitätsgründen nicht die Sicherheits-PIN von Geräten, die diese Technologie \
            unterstützen, während Chrome-basierte Browser diese immer auf jedem System abfragen.",
            p_3: "Sollten Sie ein Gerät haben, was diese Technologie unterstützt, so wird es unter \
            Umständen nicht im Firefox (Mac OS) funktionieren, sollte die Registrierung mit Chrome \
            durchgeführt worden sein. Andersherum funktioniert es allerdings in jedem Fall.",
            p_4: "Sie können pro Account zwei Schlüssel registrieren.",

            delete: "Löschen",
            error_reg: "Fehler beim Starten der Registrierung",
            invalid_key_used: "Ungültiger Sicherheitsschlüssel benutzt",
            no_key: "Es wurde in diesem Speicher noch kein Sicherheitsschlüssel registriert",
            register: "Registrieren",
            security_key: "Sicherheitsschlüssel",
            test: "Test",
            test_error: "Fehler beim Starten des Tests",
            test_success: "Test erfolgreich",
        }
    }
}
