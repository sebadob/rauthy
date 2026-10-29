use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Translations for all user-facing parts of the UI. The Admin UI makes use of them as well instead
/// of defining duplicates.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nCommon<'a> {
    pub lang: &'a str,
    pub common: Common<'a>,
    pub account: Account<'a>,
    pub authorize: Authorize<'a>,
    pub device: Device<'a>,
    pub email_change: EmailChange<'a>,
    pub error: Error<'a>,
    pub index: Index<'a>,
    pub logout: Logout<'a>,
    pub mfa: Mfa<'a>,
    pub pagination: Pagination<'a>,
    pub password_policy: PasswordPolicy<'a>,
    pub password_reset: PasswordReset<'a>,
    pub register: Register<'a>,
    pub tos: Tos<'a>,
    pub user_revoke: UserRevoke<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Common<'a> {
    pub accept: &'a str,
    pub active_theme: &'a str,
    pub authenticate: &'a str,
    pub cancel: &'a str,
    pub change_theme: &'a str,
    pub close: &'a str,
    pub copy_to_clip: &'a str,
    pub delete: &'a str,
    pub details: &'a str,
    pub edit: &'a str,
    pub email: &'a str,
    pub err_too_short: &'a str,
    pub err_too_long: &'a str,
    pub expand_content: &'a str,
    pub hide: &'a str,
    pub hours: &'a str,
    pub invalid_input: &'a str,
    pub legend: &'a str,
    pub max_file_size: &'a str,
    pub minutes: &'a str,
    pub month: &'a str,
    pub months: Vec<&'a str>,
    pub never: &'a str,
    pub password: &'a str,
    pub refresh: &'a str,
    pub required: &'a str,
    pub save: &'a str,
    pub search: &'a str,
    pub seconds: &'a str,
    pub select_i18n: &'a str,
    pub show: &'a str,
    pub summary: &'a str,
    pub timezone: &'a str,
    pub week_days_short: Vec<&'a str>,
    pub year: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account<'a> {
    pub account: &'a str,
    pub acc_type: &'a str,
    pub acc_type_passkey_text1: Cow<'a, str>,
    pub acc_type_passkey_text2: Cow<'a, str>,
    pub acc_type_passkey_text3: &'a str,
    pub access_exp: &'a str,
    pub access_renew: &'a str,
    pub access_renew_delete: &'a str,
    pub birthdate: &'a str,
    pub can_modify_for: &'a str,
    pub city: &'a str,
    pub change_password: &'a str,
    pub convert_account: &'a str,
    pub convert_account_p1: Cow<'a, str>,
    pub country: &'a str,
    pub delete_account: AccountDelete<'a>,
    pub device_id: &'a str,
    pub device_name: &'a str,
    pub devices: &'a str,
    pub devices_desc: &'a str,
    pub email_update_confirm: Cow<'a, str>,
    pub email_verified: &'a str,
    pub family_name: &'a str,
    pub federated_convert_password1: Cow<'a, str>,
    pub federated_convert_password2: Cow<'a, str>,
    pub generate_random: &'a str,
    pub given_name: &'a str,
    pub groups: &'a str,
    pub key: &'a str,
    pub key_unique: &'a str,
    pub last_login: &'a str,
    pub mfa_activated: &'a str,
    pub nav_info: &'a str,
    pub nav_edit: &'a str,
    pub nav_mfa: &'a str,
    pub nav_logout: &'a str,
    pub nav_back_to_admin: &'a str,
    pub other: &'a str,
    pub pam: AccountPam<'a>,
    pub passkeys: AccountPasskeys<'a>,
    pub password_confirm: &'a str,
    pub password_curr: &'a str,
    pub password_curr_req: &'a str,
    pub password_new: &'a str,
    pub password_new_req: &'a str,
    pub password_no_match: &'a str,
    pub password_expiry: &'a str,
    pub password_policy_follow: &'a str,
    pub password_reset: &'a str,
    pub phone: &'a str,
    pub preferred_username: AccountPreferredUsername<'a>,
    pub provider_link: &'a str,
    pub provider_link_desc: Cow<'a, str>,
    pub provider_unlink: &'a str,
    pub provider_unlink_desc: Cow<'a, str>,
    pub reg_date: &'a str,
    pub reg_ip: &'a str,
    pub roles: &'a str,
    pub street: &'a str,
    pub user: &'a str,
    pub user_created: &'a str,
    pub user_enabled: &'a str,
    pub user_expiry: &'a str,
    pub user_verified_tooltip: &'a str,
    pub resident_key_tooltip: Cow<'a, str>,
    pub aaguid_tooltip: &'a str,
    pub web_id_desc: Cow<'a, str>,
    pub web_id_desc_data: &'a str,
    pub web_id_expert_mode: &'a str,
    pub zip: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDelete<'a> {
    pub delete_account: &'a str,
    pub delete_account_desc: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPam<'a> {
    pub add_ssh_key: &'a str,
    pub comment: &'a str,
    pub generate_password: &'a str,
    pub key_added: &'a str,
    pub key_type: &'a str,
    pub username: &'a str,
    pub valid_for: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPasskeys<'a> {
    pub missing_attestation: &'a str,
    // TODO probably rename in the UI -> cleaner
    #[serde(rename = "type")]
    pub typ: &'a str,
    /// [Default, Resident Key]
    pub types: Vec<&'a str>,
    pub rk_warning: Cow<'a, str>,
    /// [Default, Resident Key]
    pub types_desc: Vec<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPreferredUsername<'a> {
    pub desc: Cow<'a, str>,
    pub force_overwrite: &'a str,
    pub immutable: &'a str,
    pub immutable_info: &'a str,
    pub not_available: &'a str,
    pub preferred_username: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorize<'a> {
    pub client_force_mfa: Cow<'a, str>,
    pub client_group_prefix_forbidden: &'a str,
    pub email: &'a str,
    pub email_bad_format: &'a str,
    pub email_required: &'a str,
    pub email_sent_msg: &'a str,
    pub http429: &'a str,
    pub invalid_credentials: &'a str,
    pub invalid_key_used: &'a str,
    pub login: &'a str,
    pub mfa_ack: &'a str,
    pub navigate_to_account: &'a str,
    pub needs_user_update: Cow<'a, str>,
    pub or_login_with: &'a str,
    pub passkey_not_discovered: Cow<'a, str>,
    pub password: &'a str,
    pub password_expired: &'a str,
    pub password_forgotten: &'a str,
    pub password_request: &'a str,
    pub password_required: &'a str,
    pub password_reset_desc: Cow<'a, str>,
    pub password_reset_success: &'a str,
    pub expecting_passkey: &'a str,
    pub expecting_otp: &'a str,
    pub request_expires: &'a str,
    pub request_expired: &'a str,
    pub sign_up: &'a str,
    pub valid_email: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device<'a> {
    pub accept: &'a str,
    pub auto_redirect_account: &'a str,
    pub close_window: &'a str,
    pub decline: &'a str,
    pub desc: &'a str,
    pub desc_scopes: &'a str,
    pub is_accepted: &'a str,
    pub is_declined: &'a str,
    pub submit: &'a str,
    pub title: &'a str,
    pub user_code: &'a str,
    pub wrong_or_expired: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailChange<'a> {
    pub title: &'a str,
    pub text_changed: &'a str,
    pub text_login: &'a str,
    pub to: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error<'a> {
    pub details: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index<'a> {
    pub register: &'a str,
    pub account_login: &'a str,
    pub admin_login: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logout<'a> {
    pub logout: &'a str,
    pub confirm_msg: &'a str,
    pub cancel: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mfa<'a> {
    pub webauthn: MfaWebauthn<'a>,
    pub otp: MfaOtp<'a>,
    pub request_canceled: &'a str,
    pub error_reg: &'a str,
    pub last_used: &'a str,
    pub no_key: &'a str,
    pub re_authenticate_passkey: &'a str,
    pub re_authenticate_pwd: &'a str,
    pub re_authenticate_otp: &'a str,
    pub register: &'a str,
    pub registered: &'a str,
    pub registered_keys: &'a str,
    pub registered_otps: &'a str,
    pub passkey_name: &'a str,
    pub passkey_name_err: &'a str,
    pub password_invalid: &'a str,
    pub test: &'a str,
    pub test_error: &'a str,
    pub test_success: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MfaWebauthn<'a> {
    pub title: &'a str,
    pub p1: Cow<'a, str>,
    pub p2: Cow<'a, str>,
    pub p3: &'a str,
    pub register_new: &'a str,
    pub doc_link_text: &'a str,
    pub unsupported_text: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MfaOtp<'a> {
    pub title: &'a str,
    pub title_email: &'a str,
    pub activation_code: &'a str,
    pub code: &'a str,
    pub register_new: &'a str,
    pub resend_otp: &'a str,
    pub session_expires_in: &'a str,
    pub session_expired: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination<'a> {
    pub entries: &'a str,
    pub goto_page: &'a str,
    pub goto_page_next: &'a str,
    pub goto_page_prev: &'a str,
    pub pagination: &'a str,
    pub show_count: &'a str,
    pub total: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordPolicy<'a> {
    pub password_policy: &'a str,
    pub length_min: &'a str,
    pub length_max: &'a str,
    pub lowercase_min: &'a str,
    pub uppercase_min: &'a str,
    pub digits_min: &'a str,
    pub special_min: &'a str,
    pub not_recent: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordReset<'a> {
    pub account_login: &'a str,
    pub bad_format: &'a str,
    pub fido_link: &'a str,
    pub generate: &'a str,
    pub new_acc_desc1: &'a str,
    pub new_acc_desc2: Cow<'a, str>,
    pub new_account: &'a str,
    pub password_reset: &'a str,
    pub password: &'a str,
    pub passwordless: &'a str,
    pub password_confirm: &'a str,
    pub password_no_match: &'a str,
    pub required: &'a str,
    pub save: &'a str,
    pub success1: &'a str,
    pub success2: &'a str,
    pub success3: &'a str,
    pub success_passkey1: &'a str,
    pub success_passkey2: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Register<'a> {
    pub already_registered_email: &'a str,
    pub already_registered_username: &'a str,
    pub domain_allowed: &'a str,
    pub domain_err: &'a str,
    pub domain_restricted: &'a str,
    pub email: &'a str,
    pub email_bad_format: &'a str,
    pub email_check: &'a str,
    pub or_register_with: &'a str,
    pub regex_name: &'a str,
    pub register: &'a str,
    pub success: &'a str,
    pub user_reg: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tos<'a> {
    pub accept_opt_until: &'a str,
    pub deny: &'a str,
    pub tos: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRevoke<'a> {
    pub title: &'a str,
    pub desc1: &'a str,
    pub desc2: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A snapshot of the real English common translation, generated from
    /// `frontend/src/i18n/common/en.ts`. It is the ground truth for both the shape
    /// (which keys exist) and the exact camelCase spelling of every key. Unlike the
    /// Admin translation, this one is already fully camelCase, so there are no
    /// pending snake_case -> camelCase renames to keep in sync here.
    const EN_COMMON: &str = include_str!("en_common.json");

    #[test]
    fn deserializes_the_real_en_translation() {
        // If any field is missing from the struct, or its serialized name does not
        // match a key in the translation, this fails. A successful parse proves the
        // struct's shape and key names line up with the real UI translation.
        let i18n: I18nCommon = serde_json::from_str(EN_COMMON).expect("en_common.json must parse");

        // Spot-check that a couple of values actually made it into the right place,
        // so this does not pass vacuously on an all-empty struct.
        assert_eq!(i18n.lang, "en");
        assert_eq!(i18n.common.accept, "Accept");
        assert!(!i18n.account.acc_type_passkey_text1.is_empty());
    }

    #[test]
    fn round_trip_is_identical_to_original() {
        let original: serde_json::Value =
            serde_json::from_str(EN_COMMON).expect("en_common.json must parse to a Value");
        let i18n: I18nCommon = serde_json::from_str(EN_COMMON).expect("en_common.json must parse");
        let reserialized: serde_json::Value = serde_json::to_value(&i18n).expect("serialize back");

        // Semantic (order-insensitive) equality of the two JSON trees. Because serde
        // silently drops unknown fields on deserialize, a misspelled or missing field
        // would drop a key here and make these differ; an extra field not present in
        // the translation would have failed the deserialize step above. Together they
        // guarantee the serialized Rust form is an exact match to the UI's JSON.
        assert_eq!(reserialized, original);
    }

    #[test]
    fn trailing_number_keys_keep_their_exact_camel_case() {
        // serde's `rename_all = "camelCase"` only upper-cases the character that
        // directly follows an `_`; digits are left untouched. These keys all end in a
        // digit, so this pins down exactly how they convert and guards against a
        // future change to the naming convention silently breaking them.
        let i18n: I18nCommon = serde_json::from_str(EN_COMMON).expect("en_common.json must parse");
        let v: serde_json::Value = serde_json::to_value(&i18n).expect("serialize back");

        let expected_paths: &[&str] = &[
            "account.convertAccountP1",
            "account.accTypePasskeyText3",
            "passwordReset.newAccDesc1",
            "passwordReset.successPasskey2",
            "mfa.webauthn.p3",
            "userRevoke.desc2",
        ];
        for path in expected_paths {
            let mut cur = &v;
            for part in path.split('.') {
                match cur.get(part) {
                    Some(next) => cur = next,
                    None => panic!("expected key '{part}' while resolving path '{path}'"),
                }
            }
        }
    }
}
