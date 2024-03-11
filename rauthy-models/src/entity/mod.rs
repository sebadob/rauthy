use crate::app_state::DbPool;
use sqlx::query;

pub mod api_keys;
pub mod app_version;
pub mod auth_codes;
pub mod auth_provider;
pub mod clients;
pub mod clients_dyn;
pub mod colors;
pub mod config;
pub mod db_version;
pub mod dpop_proof;
pub mod groups;
pub mod jwk;
pub mod jwk_token_validation;
pub mod magic_links;
pub mod password;
pub mod pow;
pub mod principal;
pub mod refresh_tokens;
pub mod roles;
pub mod scopes;
pub mod sessions;
pub mod user_attr;
pub mod users;
pub mod users_values;
pub mod webauthn;
pub mod webids;
pub mod well_known;

pub async fn is_db_alive(db: &DbPool) -> bool {
    query("SELECT 1").execute(db).await.is_ok()
}
