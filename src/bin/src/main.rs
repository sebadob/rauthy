// Copyright 2025 Sebastian Dobe <sebastiandobe@mailbox.org>

use crate::logging::setup_logging;
use actix_web::web;
use cryptr::EncKeys;
use rauthy_common::constants::{BUILD_TIME, RAUTHY_VERSION};
use rauthy_common::password_hasher;
use rauthy_handlers::openapi::ApiDoc;
use rauthy_handlers::swagger_ui::{
    OPENAPI_CONFIG, OPENAPI_JSON, SWAGGER_UI_ENABLE, SWAGGER_UI_PUBLIC,
};
use rauthy_models::app_state::AppState;
use rauthy_models::database::{Cache, DB};
use rauthy_models::email;
use rauthy_models::email::EMail;
use rauthy_models::entity::pictures::UserPicture;
use rauthy_models::events::health_watch::watch_health;
use rauthy_models::events::init_event_vars;
use rauthy_models::events::listener::EventListener;
use rauthy_models::events::notifier::EventNotifier;
use spow::pow::Pow;
use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use tracing::{debug, error, info};

#[cfg(all(feature = "jemalloc", not(target_env = "msvc")))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod dummy_data;
mod init_lazy_vars;
mod logging;
mod server;
mod tls;
mod version_migration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut test_mode = false;
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 && args[1] == "test" {
            test_mode = true;
            dotenvy::from_filename_override("rauthy-test.cfg").ok();
        } else {
            let local_test = env::var("LOCAL_TEST")
                .unwrap_or_else(|_| "false".to_string())
                .parse::<bool>()
                .expect("Cannot parse LOCAL_TEST as bool");

            if local_test {
                eprintln!(
                    "Using insecure config for testing - DO NOT USE IN PRODUCTION or on remote machines"
                );
                dotenvy::from_filename("rauthy-local_test.cfg")
                    .expect("'rauthy-local_test.cfg' not found");
            } else if let Err(err) = dotenvy::from_filename("rauthy.cfg") {
                let cwd = env::current_dir()
                    .map(|pb| pb.to_str().unwrap_or_default().to_string())
                    .unwrap_or_default();
                eprintln!(
                    "'{}/rauthy.cfg' not found, using environment variables only for configuration: {:?}",
                    cwd, err
                );
            }

            dotenvy::dotenv_override().ok();
        }
    }
    init_lazy_vars::trigger();

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
        time::sleep(Duration::from_micros(20)).await;
    }

    let log_level = setup_logging();
    info!("Starting Rauthy v{} ({})", RAUTHY_VERSION, *BUILD_TIME);
    info!("Log Level set to '{}'", log_level);
    if test_mode {
        info!("Application started in Integration Test Mode");
    }

    match EncKeys::from_env() {
        Ok(keys) => {
            // for the PoWs, we just use our active keys as b64
            Pow::init_bytes(keys.get_key(&keys.enc_key_active).unwrap());
            keys.init().unwrap()
        }
        Err(err) => {
            error!(
                r#"The `ENC_KEYS`are not correctly set up. Please take a look at the documentation:
https://sebadob.github.io/rauthy/config/encryption.html"#
            );
            panic!("{}", err);
        }
    }

    DB::init()
        .await
        .expect("Error starting the database / cache layer");

    debug!("Starting E-Mail handler");
    let (tx_email, rx_email) = mpsc::channel::<EMail>(16);
    tokio::spawn(email::sender(rx_email, test_mode));

    let (tx_events, rx_events) = flume::unbounded();
    let (tx_events_router, rx_events_router) = flume::unbounded();

    debug!("Initializing AppState");
    let app_state = web::Data::new(
        AppState::new(
            tx_email.clone(),
            tx_events.clone(),
            tx_events_router.clone(),
        )
        .await?,
    );

    debug!("Applying database migrations");
    DB::migrate(&app_state)
        .await
        .expect("Database migration error");

    debug!("Starting Events handler");
    init_event_vars().unwrap();
    EventNotifier::init_notifiers(tx_email).await.unwrap();
    tokio::spawn(EventListener::listen(
        tx_events_router,
        rx_events_router,
        rx_events,
    ));

    debug!("Starting Password Hasher");
    tokio::spawn(password_hasher::run());

    debug!("Starting health watch");
    tokio::spawn(watch_health(app_state.tx_events.clone()));

    match env::var("SCHED_DISABLE")
        .unwrap_or_else(|_| String::from("false"))
        .as_str()
    {
        "true" => {
            info!("Schedulers are disabled");
        }
        _ => {
            debug!("Starting Schedulers");
            tokio::spawn(rauthy_schedulers::spawn(app_state.clone()));
        }
    };

    version_migration::manual_version_migrations()
        .await
        .expect("Error during Rauthy version migration");

    UserPicture::test_config().await.unwrap();

    // We need to clear the HTML cache to make sure the correct static
    // assets are referenced after an app upgrade with a newly built UI.
    DB::hql().clear_cache(Cache::Html).await.unwrap();

    if *SWAGGER_UI_ENABLE {
        OPENAPI_JSON.set(ApiDoc::build(&app_state).to_json()?)?;
        let _ = *OPENAPI_CONFIG;
        let _ = *SWAGGER_UI_PUBLIC;
    }

    let metrics_enable = env::var("METRICS_ENABLE")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("Cannot parse METRICS_ENABLE to bool");
    if metrics_enable {
        server::server_with_metrics(app_state).await?;
    } else {
        server::server_without_metrics(app_state).await?;
    }

    DB::hql().shutdown().await.unwrap();

    Ok(())
}
