// Copyright 2025 Sebastian Dobe <sebastiandobe@mailbox.org>

use crate::logging::setup_logging;
use rauthy_common::constants::{BUILD_TIME, RAUTHY_VERSION};
use rauthy_common::password_hasher;
use rauthy_handlers::openapi::ApiDoc;
use rauthy_handlers::swagger_ui::{OPENAPI_CONFIG, OPENAPI_JSON};
use rauthy_models::database::{Cache, DB};
use rauthy_models::email;
use rauthy_models::email::EMail;
use rauthy_models::entity::pictures::UserPicture;
use rauthy_models::events::health_watch::watch_health;
use rauthy_models::events::listener::EventListener;
use rauthy_models::events::notifier::EventNotifier;
use rauthy_models::rauthy_config::RauthyConfig;
use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use tracing::{debug, error, info, warn};

#[cfg(all(feature = "jemalloc", not(target_env = "msvc")))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod dummy_data;
mod init_static_vars;
mod logging;
mod server;
mod tls;
mod version_migration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (config_file, test_mode) = {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 && args[1] == "test" {
            ("config-test.toml", true)
        } else {
            dotenvy::dotenv().ok();
            let local_test = env::var("LOCAL_TEST")
                .unwrap_or_else(|_| "false".to_string())
                .parse::<bool>()
                .expect("Cannot parse LOCAL_TEST as bool");

            if local_test {
                eprintln!("!!! Using insecure config for testing - DO NOT USE IN PRODUCTION !!!");
                ("config-local-test.toml", false)
            } else {
                ("config.toml", false)
            }
        }
    };

    let (tx_email, rx_email) = mpsc::channel::<EMail>(16);
    let (tx_events, rx_events) = flume::unbounded();
    let (tx_events_router, rx_events_router) = flume::unbounded();

    // TODO after everything has been migrated, keep only the AppConfig and get rid of AppState
    //  and all constants
    info!("Initializing AppConfig");

    let (rauthy_config, node_config) = RauthyConfig::build(
        config_file,
        tx_email.clone(),
        tx_events.clone(),
        tx_events_router.clone(),
    )
    .await?;
    rauthy_config.init_static();
    init_static_vars::trigger();

    if !logging::is_log_fmt_json() {
        println!(
            r#"
                                                  88
                                            ,d    88
                                            88    88
        8b,dPPYba, ,adPPYYba, 88       88 MM88MMM 88,dPPYba,  8b       d8
        88P'   "Y8 ""     `Y8 88       88   88    88P'    "8a `8b     d8'
        88         ,adPPPPP88 88       88   88    88       88  `8b   d8'
        88         88,    ,88 "8a,   ,a88   88,   88       88   `8b,d8'
        88         `"8bbdP"Y8  `"YbbdP'Y8   "Y888 88       88     Y88'
                                                                  d8'
                                                                 d8'
            "#
        );
        // On some terminals, the banner gets mixed up
        // with the first other logs without this short sleep.
        time::sleep(Duration::from_micros(10)).await;
    }

    let log_level = setup_logging();
    info!("Starting Rauthy v{} ({})", RAUTHY_VERSION, *BUILD_TIME);
    info!("Log Level set to '{}'", log_level);
    if test_mode {
        warn!("Application started in Integration Test Mode");
    }

    DB::init(node_config)
        .await
        .expect("Error starting the database / cache layer");

    // TODO check all init code from here on - we don't want any possible panics because of
    //  mis-config after DB init to avoid WAL corruption.

    debug!("Starting E-Mail handler");
    tokio::spawn(email::sender(rx_email, test_mode));

    debug!("Applying database migrations");
    DB::migrate().await.expect("Database migration error");

    debug!("Starting Events handler");
    EventNotifier::init_notifiers(tx_email).await.unwrap();
    tokio::spawn(EventListener::listen(
        tx_events_router,
        rx_events_router,
        rx_events,
    ));

    debug!("Starting Password Hasher");
    tokio::spawn(password_hasher::run());

    debug!("Starting health watch");
    tokio::spawn(watch_health());

    // Loop, because you could get into a race condition when recovery a HA Leader after lost volume
    while let Err(err) = version_migration::manual_version_migrations().await {
        error!("Error during version migration: {:?}", err);
        time::sleep(Duration::from_secs(1)).await;
    }

    UserPicture::test_config().await.unwrap();

    // We need to clear the HTML cache to make sure the correct static
    // assets are referenced after an app upgrade with a newly built UI.
    DB::hql().clear_cache(Cache::Html).await.unwrap();

    {
        let cfg = &RauthyConfig::get().vars.server;
        if cfg.swagger_ui_enable {
            OPENAPI_JSON.set(ApiDoc::build().to_json()?)?;
            let _ = *OPENAPI_CONFIG;
        }
    }

    if RauthyConfig::get().vars.server.metrics_enable {
        server::server_with_metrics().await?;
    } else {
        server::server_without_metrics().await?;
    }

    if RauthyConfig::get().is_ha_cluster {
        // this short sleep smoothes out K8s rolling releases and makes sure there is enough
        // time for in-memory cache replication before the next node is shutting down
        time::sleep(Duration::from_secs(5)).await;
    }
    DB::hql().shutdown().await.unwrap();

    Ok(())
}
