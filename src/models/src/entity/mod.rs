use crate::database::DB;
use hiqlite::params;
use rauthy_common::is_hiqlite;
use sqlx::query;

pub mod api_keys;
pub mod app_version;
pub mod auth_codes;
mod auth_provider_cust_impl;
pub mod auth_providers;
pub mod clients;
pub mod clients_dyn;
pub mod colors;
pub mod config;
pub mod continuation_token;
pub mod db_version;
pub mod devices;
pub mod dpop_proof;
pub mod fed_cm;
pub mod groups;
pub mod ip_rate_limit;
pub mod jwk;
pub mod jwk_token_validation;
pub mod logos;
pub mod magic_links;
pub mod password;
pub mod pow;
pub mod principal;
pub mod refresh_tokens;
pub mod refresh_tokens_devices;
pub mod roles;
pub mod scopes;
pub mod sessions;
pub mod theme;
pub mod user_attr;
pub mod users;
pub mod users_values;
pub mod webauthn;
pub mod webids;
pub mod well_known;

pub async fn is_db_alive() -> bool {
    if is_hiqlite() {
        // execute returning instead of query to make sure the leader is reachable in HA deployment
        DB::client()
            .execute_returning("SELECT 1", params!())
            .await
            .is_ok()
    } else {
        query("SELECT 1").execute(DB::conn()).await.is_ok()
    }
}
