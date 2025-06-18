use serde::Deserialize;
use utoipa::ToSchema;

pub mod api_keys;
pub mod auth_providers;
pub mod blacklist;
pub mod clients;
mod cust_validation;
pub mod events;
pub mod fed_cm;
pub mod forward_auth;
pub mod generic;
pub mod groups;
pub mod oidc;
pub mod roles;
pub mod scopes;
pub mod sessions;
pub mod themes;
pub mod users;

#[derive(Deserialize, ToSchema)]
pub struct PatchOp {
    pub put: Vec<PatchValue>,
    pub del: Vec<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct PatchValue {
    pub key: String,
    pub value: serde_json::Value,
}
