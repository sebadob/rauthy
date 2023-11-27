use crate::config::Config;
use axum::routing::get;
use axum::Router;
use rauthy_client::oidc_config::{JwtClaim, JwtClaimTyp, RauthyConfig};
use rauthy_client::provider::OidcProvider;
use std::collections::HashSet;
use std::sync::Arc;
use rauthy_client::{DangerAcceptInvalidCerts, RauthyHttpsOnly};
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
    rauthy_client::init(None, RauthyHttpsOnly::No, DangerAcceptInvalidCerts::Yes).await?;

    let config = RauthyConfig {
        // If this is Some(_), the principal will have a .is_admin field being set correctly, if
        // this claim matches.
        admin_claim: Some(JwtClaim {
            typ: JwtClaimTyp::Roles,
            value: "admin".to_string(),
        }),
        // This claim must always exist for every single user. Without this claim, a user would
        // not have access to this app. This is used, because usually you never want to just have
        // all your OIDC users to have access to a certain application.
        user_claim: JwtClaim {
            typ: JwtClaimTyp::Groups,
            value: "user".to_string(),
        },
        // In almost all cases, this should just match the `client_id`
        allowed_audiences: HashSet::from(["dev-test".to_string()]),
        client_id: "dev-test".to_string(),
        // If set to 'false', tokens with a non-verified email address will be rejected.
        email_verified: true,
        // The issuer URL from your Rauthy deployment
        iss: "https://rauthy.local/auth/v1".to_string(),
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
        secret: None,
        // secret: Some("secretCopiedFromTheRauthyUiIfIsConfidentialClient".to_string(),),
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
    let listener = tokio::net::TcpListener::bind(addr).await.expect("port 3000 to be free");
    info!("Server listening on {:?}", addr);

    axum::serve(listener, routes).await?;

    Ok(())
}
