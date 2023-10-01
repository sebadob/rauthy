use crate::i18n::password_policy::I18nPasswordPolicy;
use crate::i18n::SsrJson;
use crate::language::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nAccount<'a> {
    account: &'a str,
    acc_type_passkey_text_1: &'a str,
    acc_type_passkey_text_2: &'a str,
    acc_type_passkey_text_3: &'a str,
    cancel: &'a str,
    change_password: &'a str,
    convert_account: &'a str,
    convert_account_p_1: &'a str,
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
    nav_password: &'a str,
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
    user_expiry: &'a str,
    user_verified_tooltip: &'a str,
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
            acc_type_passkey_text_1: r#"This is account is currently a passkey only account.
This means, that you do not have any password, because you don't need one."#,
            acc_type_passkey_text_2: r#"You can convert your account and add a password. But keep
in mind, that this implies, that you need to verify each new device with the password additionally.
You then cannot just log in on any device, where you have not entered the password beforehand at
least once."#,
            acc_type_passkey_text_3: "Do you want to convert your account and add a password?",
            cancel: "Cancel",
            change_password: "Change Password",
            convert_account: "Convert Account",
            convert_account_p_1: r#"You can convert your account to a Passkey-Only account.
This conversion deletes your password and you can and must only ever login with your registered
passkeys. Keep in mind, that only passkeys with the additional User Verification will be accepted.
If you passkeys support this, you will find a small symbol behind the name of the key on the 'MFA'
page."#,
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
            nav_password: "Password",
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
            user_expiry: "User Expires",
            user_verified_tooltip: "Secured with fingerprint or PIN",
            valid_email: "Valid E-Mail format",
            valid_given_name: "Your given name with 2 - 32 non-special characters",
            valid_family_name: "Your family name with 2 - 32 non-special characters",
        }
    }

    fn build_de() -> Self {
        Self {
            account: "Account",
            acc_type_passkey_text_1: r#"Dies ist ein "Passkey-Only" Account. Das bedeutet, dass
dieser Account kein Passwort hat und auch keines benötigt."#,
            acc_type_passkey_text_2: r#"Der Account kann in einen Passwort-Account umgewandelt
werden. Das würde allerdings bedeuten, dass ein Login auf einem neuen Gerät ohne vorherige, zumindest
einmalige zusätzliche Verifizierung des Passwortes nicht mehr möglich sein wird."#,
            acc_type_passkey_text_3:
                "Soll dieser Account gewandelt und ein Passwort hinzugefügt werden?",
            cancel: "Abbrechen",
            change_password: "Passwort wechseln",
            convert_account: "Account Umwandeln",
            convert_account_p_1: r#"Dieser Account kann in einen Passkey-Only Account umgewandelt
werden. Diese Umwandling löscht das Passwort und erlaubt den alleinigen Login mit den registrieren
Passkeys. Nur Passkeys mit zusätzlicher Benutzerverifizierung werden akzeptiert. Diese sind auf der
'MFA' Seite durch das zusätzliche Symbol hinter dem Passkey Namen gekennzeichnet."#,
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
            nav_password: "Passwort",
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
            user_expiry: "Benutzer Ablauf",
            user_verified_tooltip: "Abgesichert durch Fingerabdruck oder PIN",
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

    delete: &'a str,
    error_reg: &'a str,
    invalid_key_used: &'a str,
    last_used: &'a str,
    no_key: &'a str,
    register: &'a str,
    register_new: &'a str,
    registerd: &'a str,
    registerd_keys: &'a str,
    passkey_name: &'a str,
    passkey_name_err: &'a str,
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
    pub(crate) fn build_en() -> Self {
        Self {
            p_1: "If you plan on using your MFA key with multiple systems like Windows and \
            Android, you should do the registration with Android.",
            p_2: "Android is the platform with the least supported features for the passwordless \
            technology. Keys you register with Android work elsewhere too. However, this does not \
            apply the other way around.",

            delete: "Delete",
            error_reg: "Error starting the Registration process",
            invalid_key_used: "Invalid Key used",
            last_used: "Last used",
            no_key: "No Security key registered on this slot",
            register: "Register",
            register_new: "Register New Key",
            registerd: "Registered",
            registerd_keys: "Registered Keys",
            passkey_name: "Passkey Name",
            passkey_name_err: "2 - 32 non-special characters",
            test: "Test",
            test_error: "Error starting the Test",
            test_success: "Test successful",
        }
    }

    pub(crate) fn build_de() -> Self {
        Self {
            p_1: "Wenn Sie mehrere Systeme parallel nutzen möchten, wie z.B. Windows und Android, \
            sollten Sie die Registrierung mit Android durchführen.",
            p_2: "Android ist diejenige Plattform, die derzeit die wenigsten Features der \
            passwortlosen Technologie unterstützt. Schlüssel, die dort registriert werden, \
            funktionieren auf anderen Geräten gleichermaßen. Dies gilt jedoch nicht andersherum.",

            delete: "Löschen",
            error_reg: "Fehler beim Starten der Registrierung",
            invalid_key_used: "Ungültiger Sicherheitsschlüssel benutzt",
            last_used: "Zuletzt genutzt",
            no_key: "Es wurde in diesem Speicher noch kein Sicherheitsschlüssel registriert",
            register: "Registrieren",
            register_new: "Neuen Key Registrieren",
            registerd: "Registriert",
            registerd_keys: "Registrierte Keys",
            passkey_name: "Passkey Name",
            passkey_name_err: "2 - 32 Buchstaben, keine Sonderzeichen",
            test: "Test",
            test_error: "Fehler beim Starten des Tests",
            test_success: "Test erfolgreich",
        }
    }
}
