use hiqlite::NodeConfig;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static CLIENT: OnceLock<hiqlite::Client> = OnceLock::new();

/// Cache Index for the `hiqlite` cache layer
#[derive(Debug, Serialize, Deserialize, hiqlite::EnumIter, hiqlite::ToPrimitive)]
pub enum Cache {
    App,
    AuthCode,
    DeviceCode,
    AuthProviderCallback,
    ClientDynamic,
    ClientEphemeral,
    DPoPNonce,
    IPRateLimit,
    Session,
    PoW,
    User,
    Webauthn,
}

pub struct DB;

impl DB {
    /// Builds the NodeConfig and starts the Hiqlite node
    pub async fn init() -> Result<(), ErrorResponse> {
        if CLIENT.get().is_some() {
            panic!("cache::start_cache() must only be called once");
        }

        let config = NodeConfig::from_env();
        let client = hiqlite::start_node_with_cache::<Cache>(config).await?;

        let _ = CLIENT.set(client);

        Ok(())
    }

    pub fn client() -> &'static hiqlite::Client {
        CLIENT
            .get()
            .expect("cache::start_cache() must be called at startup")
    }
}
