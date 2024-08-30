use hiqlite::NodeConfig;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub static HIQLITE: OnceLock<hiqlite::Client> = OnceLock::new();

/// Cache Index for the `hiqlite` cache layer
#[derive(Debug, Serialize, Deserialize, hiqlite::EnumIter, hiqlite::ToPrimitive)]
pub enum Cache {
    App,
    AuthCode,
    DeviceCode,
    AuthProviderCallback,
    DynamicClient,
    DPoPNonce,
    EphemeralClient,
    IPRateLimit,
    Session,
    PoW,
    User,
    WebauthnReq,
}

/// Builds the NodeConfig, starts the Hiqlite node and sets the static `HIQLITE` client.
pub async fn start_cache() -> Result<(), ErrorResponse> {
    if HIQLITE.get().is_some() {
        panic!("cache::start_cache() must only be called once");
    }

    let config = NodeConfig::from_env();
    let client = hiqlite::start_node_with_cache::<Cache>(config).await?;

    let _ = HIQLITE.set(client);

    Ok(())
}
