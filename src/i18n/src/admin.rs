use rauthy_common::content_hash::content_hash;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Translations specific to only the Admin UI.
#[derive(Debug, Serialize, Deserialize)]
// Pure-grouping struct (no leaf `&'a str` fields): serde cannot infer the relationship between
// `'de` and `'a`, so state it explicitly. In all places where we need to escape values like \n,
// we need to use a `Cow<_>`.
#[serde(rename_all = "camelCase", bound(deserialize = "'de: 'a"))]
pub struct I18nAdmin<'a> {
    // TODO this is snake_case in the current UI when it should be camelCase
    #[serde(rename = "api_key")]
    pub api_key: ApiKey<'a>,
    pub attrs: Attrs<'a>,
    pub backup: Backup<'a>,
    pub clients: Clients<'a>,
    pub common: AdminCommon<'a>,
    pub docs: Docs<'a>,
    pub editor: Editor<'a>,
    pub email: Email<'a>,
    pub error: Error<'a>,
    pub events: Events<'a>,
    pub groups: Groups<'a>,
    pub jwks: Jwks<'a>,
    pub kv: Kv<'a>,
    pub nav: Nav<'a>,
    pub options: Options<'a>,
    pub pam: Pam<'a>,
    pub password_policy: PasswordPolicy<'a>,
    pub providers: Providers<'a>,
    pub roles: Roles<'a>,
    pub scopes: Scopes<'a>,
    pub search: Search<'a>,
    pub sessions: Sessions<'a>,
    pub tabs: Tabs<'a>,
    pub tos: Tos<'a>,
    pub users: Users<'a>,
    pub validation: Validation<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey<'a> {
    pub delete1: &'a str,
    pub expires: &'a str,
    pub generate1: &'a str,
    pub generate2: Cow<'a, str>,
    /// inserted as html
    pub generate3: Cow<'a, str>,
    /// inserted as html
    pub generate4: &'a str,
    /// inserted as html
    pub generate5: &'a str,
    pub key_name: &'a str,
    pub limited_validity: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs<'a> {
    pub delete1: &'a str,
    pub default_value: &'a str,
    pub desc: &'a str,
    pub make_editable: &'a str,
    pub make_editable_p1: &'a str,
    /// inserted as html
    pub make_editable_p2: Cow<'a, str>,
    pub make_editable_p3: Cow<'a, str>,
    pub name: &'a str,
    pub user_editable: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backup<'a> {
    pub create_backup: &'a str,
    pub disabled_desc: &'a str,
    pub last_modified: &'a str,
    pub local: &'a str,
    pub name: &'a str,
    pub size: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clients<'a> {
    pub backchannel_logout: &'a str,
    pub branding: ClientsBranding<'a>,
    pub claims: &'a str,
    pub claims_at_root: &'a str,
    pub claims_at_root_warning: Cow<'a, str>,
    pub claims_desc: Cow<'a, str>,
    pub confidential: &'a str,
    pub confidential_no_secret: &'a str,
    pub config: &'a str,
    pub delete1: &'a str,
    pub desc_auth_code: Cow<'a, str>,
    pub desc_client_uri: Cow<'a, str>,
    pub allowed_resources: &'a str,
    pub default_aud: &'a str,
    pub desc_allowed_resources: &'a str,
    pub desc_default_aud: &'a str,
    pub desc_group_prefix: Cow<'a, str>,
    pub desc_name: Cow<'a, str>,
    pub desc_origin: Cow<'a, str>,
    #[serde(rename = "descPKCE")]
    pub desc_pkce: Cow<'a, str>,
    #[serde(rename = "descPKCEEnforce")]
    pub desc_pkce_enforce: Cow<'a, str>,
    /// inserted as html
    pub desc_uri: Cow<'a, str>,
    #[serde(rename = "errConfidentialPKCE")]
    pub err_confidential_pkce: Cow<'a, str>,
    pub force_mfa: &'a str,
    pub group_login_prefix: &'a str,
    pub name: &'a str,
    pub password_flow_mfa_warn: Cow<'a, str>,
    pub scim: ClientsScim<'a>,
    pub scopes: ClientsScopes<'a>,
    pub secret: ClientsSecret<'a>,
    pub token_lifetime: ClientsTokenLifetime<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientsBranding<'a> {
    pub desc_hsl: Cow<'a, str>,
    /// inserted as html
    pub desc_full_css: Cow<'a, str>,
    pub desc_variables: Cow<'a, str>,
    pub favicon_preview_alt: &'a str,
    pub favicon_upload: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientsScim<'a> {
    pub base_uri: Cow<'a, str>,
    pub desc: &'a str,
    pub enable: &'a str,
    pub group_sync: &'a str,
    pub group_sync_prefix: &'a str,
    pub group_sync_prefix_desc: Cow<'a, str>,
    pub req_desc: &'a str,
    pub req_li1: &'a str,
    pub req_li2: Cow<'a, str>,
    pub req_li3: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientsScopes<'a> {
    pub allowed: &'a str,
    pub default: &'a str,
    /// inserted as html
    pub desc: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientsSecret<'a> {
    pub do_cache: &'a str,
    pub cache_duration: &'a str,
    pub generate: &'a str,
    pub rotate_desc1: Cow<'a, str>,
    pub rotate_desc2: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientsTokenLifetime<'a> {
    pub p1: &'a str,
    pub p2: Cow<'a, str>,
    pub p3: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminCommon<'a> {
    pub account: &'a str,
    pub add_new: &'a str,
    pub back: &'a str,
    pub caution: &'a str,
    pub contact: &'a str,
    pub copied_to_clip: &'a str,
    pub details: &'a str,
    pub edit: &'a str,
    pub enabled: &'a str,
    pub filter: &'a str,
    pub from: &'a str,
    pub information: &'a str,
    pub language: &'a str,
    pub loading: &'a str,
    pub json_meta: &'a str,
    pub name: &'a str,
    pub name_exists_already: &'a str,
    pub note: &'a str,
    pub no_entries: &'a str,
    pub preview: &'a str,
    pub reset: &'a str,
    pub search_options: &'a str,
    pub until: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Docs<'a> {
    pub book: &'a str,
    pub encryption: &'a str,
    pub enc_keys: DocsEncKeys<'a>,
    pub hashing: DocsHashing<'a>,
    pub openapi: &'a str,
    pub openapi_note: Cow<'a, str>,
    pub source: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocsEncKeys<'a> {
    pub header: &'a str,
    pub key_active: &'a str,
    pub keys_available: &'a str,
    pub migrate: &'a str,
    pub migrate_to_key: &'a str,
    pub p1: Cow<'a, str>,
    pub p2: Cow<'a, str>,
    pub p3: Cow<'a, str>,
    pub p_not_possible: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocsHashing<'a> {
    pub calculate: &'a str,
    pub curr_values_head: &'a str,
    pub curr_values1: &'a str,
    pub curr_values_note: Cow<'a, str>,
    pub curr_values_threads_access: &'a str,
    pub login_time_head: &'a str,
    pub login_time1: Cow<'a, str>,
    pub login_time2: Cow<'a, str>,
    /// inserted as html
    pub m_cost1: Cow<'a, str>,
    /// inserted as html
    pub m_cost2: Cow<'a, str>,
    /// inserted as html
    pub m_cost3: &'a str,
    /// inserted as html
    pub p_cost1: Cow<'a, str>,
    /// inserted as html
    pub p_cost2: Cow<'a, str>,
    /// inserted as html
    pub t_cost1: Cow<'a, str>,
    /// inserted as html
    pub t_cost2: Cow<'a, str>,
    pub utility_head: &'a str,
    pub utility1: Cow<'a, str>,
    /// inserted as html
    pub utility2: Cow<'a, str>,
    pub time: &'a str,
    pub target_time: &'a str,
    pub tune: &'a str,
    pub p_details: Cow<'a, str>,
    pub p_tune: Cow<'a, str>,
    pub p_utility: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Editor<'a> {
    pub bold: &'a str,
    pub code: &'a str,
    pub heading1: &'a str,
    pub heading2: &'a str,
    pub heading3: &'a str,
    pub italic: &'a str,
    pub link: &'a str,
    pub list_bullet: &'a str,
    pub list_tasks: &'a str,
    pub list_numbered: &'a str,
    pub paragraph: &'a str,
    pub quote: &'a str,
    pub remove_fmt: &'a str,
    pub strike_through: &'a str,
    pub text_area: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email<'a> {
    pub cancel_job: &'a str,
    /// MUST be in the order: 'none', 'in_group', 'not_in_group', 'has_role', 'has_not_role'
    pub filter_type: Vec<&'a str>,
    pub immediate: &'a str,
    pub jobs: &'a str,
    pub scheduled: &'a str,
    pub send_all_users: &'a str,
    pub send_all_users_filtered: &'a str,
    pub send_mail: &'a str,
    pub subject: &'a str,
    pub user_filter: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error<'a> {
    /// inserted as html
    pub needs_admin_role: Cow<'a, str>,
    /// inserted as html
    pub no_admin: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Events<'a> {
    pub event_level: &'a str,
    pub event_type: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Groups<'a> {
    pub delete1: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Jwks<'a> {
    pub alg: &'a str,
    pub p1: &'a str,
    pub p2: Cow<'a, str>,
    pub p3: Cow<'a, str>,
    #[serde(rename = "type")]
    pub typ: &'a str,
    pub rotate_keys: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kv<'a> {
    /// inserted as html
    pub access_test_desc: Cow<'a, str>,
    pub add_new_key: &'a str,
    pub add_new_ns: &'a str,
    pub add_new_value: &'a str,
    pub del_confirm: &'a str,
    pub del_ns_msg: &'a str,
    pub encrypted_desc: Cow<'a, str>,
    /// MUST contain `{{ key }}` to be replaced with the key to delete
    pub delete_confirm_msg: &'a str,
    pub help: KvHelp<'a>,
    pub key: &'a str,
    pub load_all_values: &'a str,
    pub store_encrypted: &'a str,
    /// MUT be in order: data, access, edit, delete
    pub tabs: Vec<&'a str>,
    pub test_cmd: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KvHelp<'a> {
    pub help: &'a str,
    /// MUST be in order:
    /// - test access key
    /// - show all keys
    /// - show all keys + values
    /// - set a key + value
    /// - get the value for a key
    /// - delete a key + value
    pub ops: Vec<&'a str>,
    pub p1: Cow<'a, str>,
    /// inserted as html
    pub p2: Cow<'a, str>,
    /// inserted as html
    pub p3: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nav<'a> {
    pub api_keys: &'a str,
    pub attributes: &'a str,
    pub blacklist: &'a str,
    pub clients: &'a str,
    pub config: &'a str,
    pub docs: &'a str,
    pub events: &'a str,
    pub groups: &'a str,
    pub providers: &'a str,
    pub roles: &'a str,
    pub scopes: &'a str,
    pub sessions: &'a str,
    pub users: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options<'a> {
    pub expires: &'a str,
    pub last_seen: &'a str,
    pub state: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pam<'a> {
    pub add_group: &'a str,
    pub add_host: &'a str,
    pub add_user: &'a str,
    pub delete_host: &'a str,
    pub delete_user: &'a str,
    pub group_desc_generic: Cow<'a, str>,
    pub group_desc_host: Cow<'a, str>,
    pub group_desc_local: Cow<'a, str>,
    pub group_desc_user: Cow<'a, str>,
    pub group_desc_wheel: Cow<'a, str>,
    pub group_name: &'a str,
    pub groups: &'a str,
    pub group_type: &'a str,
    pub host_aliases: &'a str,
    pub host_local_pwd_only: &'a str,
    pub host_local_pwd_only_info: Cow<'a, str>,
    pub ip_addresses: &'a str,
    pub member: &'a str,
    pub name_exists_already: &'a str,
    pub notes: &'a str,
    pub secret_show: &'a str,
    pub secret_rotate: &'a str,
    pub user_email: &'a str,
    pub username: &'a str,
    pub username_new_desc: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordPolicy<'a> {
    pub config_desc: &'a str,
    pub reset_set0: &'a str,
    pub valid_for_days: &'a str,
    pub validity_new: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
// Pure-grouping struct (no leaf `&'a str` fields): serde cannot infer the
// relationship between `'de` and `'a`, so state it explicitly.
#[serde(rename_all = "camelCase", bound(deserialize = "'de: 'a"))]
pub struct Providers<'a> {
    pub config: ProvidersConfig<'a>,
    pub delete: ProvidersDelete<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvidersConfig<'a> {
    pub allow_insecure_tls: &'a str,
    pub auto_link: &'a str,
    pub auto_link_desc1: Cow<'a, str>,
    pub auto_link_desc2: Cow<'a, str>,
    pub client_name: &'a str,
    pub cust_root_ca: &'a str,
    /// inserted as html
    pub desc_auth_method: Cow<'a, str>,
    pub desc_client_id: &'a str,
    pub desc_client_name: &'a str,
    pub desc_client_secret: Cow<'a, str>,
    pub desc_scope: Cow<'a, str>,
    pub err_no_auth_method: &'a str,
    pub err_confidential: &'a str,
    pub json_path: ProvidersJsonPath<'a>,
    pub lookup: &'a str,
    pub path_admin_claim: &'a str,
    pub path_mfa_claim: &'a str,
    pub root_pem_cert: &'a str,
    pub map_mfa: Cow<'a, str>,
    pub map_user: &'a str,
    pub value_admin_claim: &'a str,
    pub value_mfa_claim: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvidersJsonPath<'a> {
    pub p1: &'a str,
    /// inserted as html
    pub p2: Cow<'a, str>,
    /// inserted as html
    pub p3: &'a str,
    /// inserted as html
    pub p4: &'a str,
    /// inserted as html
    pub p5: Cow<'a, str>,
    /// inserted as html
    pub p6: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvidersDelete<'a> {
    pub are_you_sure: &'a str,
    pub force_delete: &'a str,
    pub is_in_use1: &'a str,
    pub is_in_use2: Cow<'a, str>,
    pub linked_users: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roles<'a> {
    /// inserted as html
    pub admin_no_mod: &'a str,
    pub delete1: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scopes<'a> {
    pub claims_at_root: &'a str,
    pub claims_at_root_warning: Cow<'a, str>,
    pub default_no_mod: &'a str,
    pub delete1: &'a str,
    pub delete_default: &'a str,
    pub mapping1: &'a str,
    pub mapping2: Cow<'a, str>,
    pub name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search<'a> {
    pub order_by: &'a str,
    pub order_change_to_asc: &'a str,
    pub order_change_to_desc: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sessions<'a> {
    pub invalidate_all: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tabs<'a> {
    pub config: &'a str,
    pub delete: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tos<'a> {
    pub accepted: &'a str,
    pub add_new_to_s: &'a str,
    pub add_new_to_s_from_current: &'a str,
    pub added: &'a str,
    pub check_status: &'a str,
    pub immutable: Cow<'a, str>,
    pub none_exist: &'a str,
    pub opt_until: TosOptUntil<'a>,
    pub tos: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TosOptUntil<'a> {
    pub desc: Cow<'a, str>,
    pub enable: &'a str,
    pub label: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Users<'a> {
    pub anti_lockout: UsersAntiLockout<'a>,
    pub attributes: &'a str,
    pub delete_user: &'a str,
    pub desc_attr: &'a str,
    pub force_logout: Cow<'a, str>,
    pub group_admin: UsersGroupAdmin<'a>,
    pub last_login: &'a str,
    pub manual_init_desc: Cow<'a, str>,
    pub manual_init: &'a str,
    pub mfa: UsersMfa<'a>,
    pub pk_only1: &'a str,
    pub pk_only2: &'a str,
    pub pk_only3: Cow<'a, str>,
    pub pwd_no_init: &'a str,
    pub pwd_send_email_btn: &'a str,
    pub pwd_send_email_desc: &'a str,
    pub save_password: &'a str,
    pub self_service_desc: &'a str,
    pub send_reset_email: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersAntiLockout<'a> {
    pub rule: &'a str,
    pub delete: &'a str,
    pub disable: &'a str,
    pub rauthy_admin: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersGroupAdmin<'a> {
    pub not_managed_title: &'a str,
    pub not_managed_desc: Cow<'a, str>,
    pub add_to_groups: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
// Pure-grouping struct (no leaf `&'a str` fields): serde cannot infer the
// relationship between `'de` and `'a`, so state it explicitly.
#[serde(rename_all = "camelCase", bound(deserialize = "'de: 'a"))]
pub struct UsersMfa<'a> {
    pub otp: UsersOtp<'a>,
    pub webauthn: UsersWebAuthn<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersOtp<'a> {
    pub title: &'a str,
    pub mfa_delete1: &'a str,
    pub mfa_delete2: Cow<'a, str>,
    pub no_mfa_otps: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersWebAuthn<'a> {
    pub title: &'a str,
    pub mfa_delete1: &'a str,
    pub mfa_delete2: Cow<'a, str>,
    pub no_mfa_keys: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Validation<'a> {
    pub css: &'a str,
    pub origin: &'a str,
    pub uri: &'a str,
    pub redirect_uri: ValidationRedirectUri<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationRedirectUri<'a> {
    pub comma: &'a str,
    pub control_char: &'a str,
    pub fragment: &'a str,
    /// `{{ KEY }}` is replaced with the reserved query parameter
    pub reserved_key: Cow<'a, str>,
    /// like `reserved_key`, for a post-logout redirect URI
    pub reserved_key_logout: Cow<'a, str>,
}

impl I18nAdmin<'_> {
    pub fn content_hash(&self) -> String {
        content_hash(serde_json::to_string(self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A snapshot of the real English Admin translation, generated from
    /// `frontend/src/i18n/admin/en.ts`. It is the ground truth for both the shape
    /// (which keys exist) and the exact camelCase spelling of every key.
    const EN_ADMIN: &str = include_str!("en_admin.json");

    #[test]
    fn deserializes_the_real_en_translation() {
        // If any field is missing from the struct, or its serialized name does not
        // match a key in the translation, this fails. A successful parse proves the
        // struct's shape and key names line up with the real UI translation.
        let admin: I18nAdmin = serde_json::from_str(EN_ADMIN).expect("en_admin.json must parse");

        // Spot-check that a couple of values actually made it into the right place,
        // so this does not pass vacuously on an all-empty struct.
        assert_eq!(admin.common.account, "Account");
        assert!(!admin.api_key.key_name.is_empty());
    }

    #[test]
    fn round_trip_is_identical_to_original() {
        let original: serde_json::Value =
            serde_json::from_str(EN_ADMIN).expect("en_admin.json must parse to a Value");
        let admin: I18nAdmin = serde_json::from_str(EN_ADMIN).expect("en_admin.json must parse");
        let reserialized: serde_json::Value = serde_json::to_value(&admin).expect("serialize back");

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
        let admin: I18nAdmin = serde_json::from_str(EN_ADMIN).expect("en_admin.json must parse");
        let v: serde_json::Value = serde_json::to_value(&admin).expect("serialize back");

        let expected_paths: &[&str] = &[
            // snake_case until the frontend api_key -> apiKey rename lands (see I18nAdmin TODO)
            "api_key.delete1",
            "attrs.makeEditableP1",
            "clients.tokenLifetime.p1",
            "docs.hashing.mCost1",
            "passwordPolicy.resetSet0",
            "providers.config.jsonPath.p6",
            "users.mfa.otp.mfaDelete1",
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
