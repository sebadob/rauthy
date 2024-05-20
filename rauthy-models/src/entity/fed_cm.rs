use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The struct for the `.well-known` endpoint for FedCM provider lookups
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct WebIdentity {
    pub provider_urls: Vec<String>,
}

impl WebIdentity {
    pub fn new(issuer: &str) -> Self {
        Self {
            provider_urls: vec![format!("{}/fed_cm/config.json", issuer)],
        }
    }
}
