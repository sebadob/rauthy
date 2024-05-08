use crate::config::Config;
use axum::routing::get;
use axum::Router;
use rauthy_client::oidc_config::{ClaimMapping, JwtClaim, JwtClaimTyp, RauthyConfig};
use rauthy_client::provider::OidcProvider;
use rauthy_client::{DangerAcceptInvalidCerts, RauthyHttpsOnly};
use std::collections::HashSet;
use std::sync::Arc;
use tracing::{info, subscriber, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod routes;
pub mod templates;

// I often use something like DEV_MODE to make local dev easier in certain places and really
// secure in production. In a real app, you would read this value in from a config or env.
pub const DEV_MODE: bool = true;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // we need to start the backend for the rauthy-client
    rauthy_client::init_with(None, RauthyHttpsOnly::No, DangerAcceptInvalidCerts::Yes).await?;

    let config = RauthyConfig {
        // Sets the .is_admin field for the principal based on the `ClaimMapping`.
        admin_claim: ClaimMapping::Or(vec![JwtClaim {
            typ: JwtClaimTyp::Roles,
            value: "admin".to_string(),
        }]),
        // Sets the .is_user field for the principal based on the `ClaimMapping`.
        // Without this claim, a user would not have access to this app. This is
        // used, because usually you never want to just have all your OIDC users to
        // have access to a certain application.
        user_claim: ClaimMapping::Or(vec![JwtClaim {
            typ: JwtClaimTyp::Groups,
            value: "user".to_string(),
        }]),
        // In almost all cases, this should just match the `client_id`
        allowed_audiences: HashSet::from(["dev-test".to_string()]),
        client_id: "test".to_string(),
        // client_id: "dev-test".to_string(),
        // If set to 'false', tokens with a non-verified email address will be rejected.
        email_verified: true,
        // The issuer URL from your Rauthy deployment
        iss: "https://iam.sebastiandobe.de/auth/v1".to_string(),
        // iss: "https://rauthy.local/auth/v1".to_string(),
        // The scopes you want to request. The only mandatory which always needs to exist is
        // `openid`, the rest is optional and depending on your needs.
        scope: vec![
            "openid".to_string(),
            "email".to_string(),
            "profile".to_string(),
            "groups".to_string(),
        ],
        // If set to None, the client will be treated as a public client and not provide any
        // secret to the /token endpoint after the callback. Set a secret for confidential clients.
        // secret: None,
        secret: Some(
            "IXuojGQT9nGbnoB4MHS6RiRdTL6yCr0kf6F3uGwUjkc1tzstXIGEUCkzJdGkOoOz".to_string(),
        ),
    };
    // The redirect_uri here must match the URI of this application, where we accept and handle
    // the callback after a successful login.
    OidcProvider::setup_from_config(config, "http://localhost:3000/callback").await?;

    let config = Config::new().await?;

    let routes = Router::new()
        .route("/", get(routes::get_index))
        .route("/auth_check", get(routes::get_auth_check))
        .route("/callback", get(routes::get_callback))
        .route("/protected", get(routes::get_protected))
        // in production, you should add middlewares here with safe default resposne headers
        .with_state(Arc::new(config));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("port 3000 to be free");
    info!("Server listening on {:?}", addr);

    axum::serve(listener, routes).await?;

    Ok(())
}
