use crate::config::Config;
use axum::routing::get;
use axum::Router;
use rauthy_client::oidc_config::{JwtClaim, JwtClaimTyp, RauthyConfig};
use rauthy_client::provider::OidcProvider;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, Level};

mod config;
mod routes;
pub mod templates;

// If you plan to use CSRF protection headers, declare a const and add it to the sensitive
// headers down below to never leak these in debug logs
pub const XSRF_HEADER: &str = "X-XSRF";

// I often use something like DEV_MODE to make local dev easier in certain places and really
// secure in production. In a real app, you would read this value in from a config or env.
pub const DEV_MODE: bool = true;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // we need to start the backend for the rauthy-client
    rauthy_client::init(None, false, true).await?;

    let config = RauthyConfig {
        admin_claim: Some(JwtClaim {
            typ: JwtClaimTyp::Roles,
            value: "admin".to_string(),
        }),
        user_claim: JwtClaim {
            typ: JwtClaimTyp::Groups,
            value: "user".to_string(),
        },
        allowed_audiences: HashSet::from(["dev-test".to_string()]),
        client_id: "dev-test".to_string(),
        email_verified: true,
        iss: "https://iam.rauthy.local/auth/v1".to_string(),
        // iss: "https://iam.sebastiandobe.de/auth/v1".to_string(),
        scope: vec![
            "openid".to_string(),
            "email".to_string(),
            "profile".to_string(),
            "groups".to_string(),
        ],
        secret: None,
        // secret: Some("secretCopiedFromTheRauthyUiIfIsConfidentialClient".to_string(),),
    };
    OidcProvider::setup_from_config(config, "http://localhost:3000/callback").await?;

    let config = Config::new().await?;

    let routes = Router::new()
        .route("/", get(routes::get_index))
        .route("/auth_check", get(routes::get_auth_check))
        .route("/callback", get(routes::get_callback))
        .route("/protected", get(routes::get_protected))
        .with_state(Arc::new(config));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await?;

    Ok(())
}
