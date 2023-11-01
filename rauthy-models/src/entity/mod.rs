use crate::app_state::DbPool;
use sqlx::query;

pub mod api_keys;
pub mod app_version;
pub mod auth_codes;
pub mod clients;
pub mod colors;
pub mod config;
pub mod db_version;
pub mod dpop_proof;
pub mod groups;
pub mod jwk;
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
pub mod webauthn;

pub async fn is_db_alive(db: &DbPool) -> bool {
    query("SELECT 1").execute(db).await.is_ok()
}
