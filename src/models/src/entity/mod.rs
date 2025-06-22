use crate::database::DB;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;

pub mod api_keys;
pub mod app_version;
pub mod atproto;
pub mod auth_codes;
mod auth_provider_cust_impl;
pub mod auth_providers;
pub mod clients;
pub mod clients_dyn;
pub mod clients_scim;
pub mod config;
pub mod continuation_token;
pub mod db_version;
pub mod devices;
pub mod dpop_proof;
pub mod failed_backchannel_logout;
pub mod failed_login_counter;
pub mod failed_scim_tasks;
pub mod fed_cm;
pub mod forward_auth;
pub mod groups;
pub mod ip_blacklist;
pub mod ip_rate_limit;
pub mod jwk;
pub mod jwk_token_validation;
pub mod logos;
pub mod magic_links;
pub mod password;
pub mod pictures;
pub mod pow;
pub mod principal;
pub mod refresh_tokens;
pub mod refresh_tokens_devices;
pub mod roles;
pub mod scim_types;
pub mod scopes;
pub mod sessions;
pub mod theme;
pub mod user_attr;
pub mod user_login_states;
pub mod users;
pub mod users_values;
pub mod webauthn;
pub mod webids;
pub mod well_known;

pub async fn is_db_alive() -> bool {
    if is_hiqlite() {
        // execute returning instead of query to make sure the leader is reachable in HA deployment
        DB::hql()
            .execute_returning("SELECT 1", params!())
            .await
            .is_ok()
    } else {
        DB::pg_query_one_row("SELECT 1", &[]).await.is_ok()
    }
}
