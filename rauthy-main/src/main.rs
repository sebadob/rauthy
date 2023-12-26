// Copyright 2023 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

use actix_web::rt::System;
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use cryptr::{EncKeys, EncValue};
use prometheus::Registry;
use rauthy_common::constants::{
    CACHE_NAME_12HR, CACHE_NAME_AUTH_CODES, CACHE_NAME_DPOP_NONCES, CACHE_NAME_EPHEMERAL_CLIENTS,
    CACHE_NAME_LOGIN_DELAY, CACHE_NAME_POW, CACHE_NAME_SESSIONS, CACHE_NAME_WEBAUTHN,
    CACHE_NAME_WEBAUTHN_DATA, DPOP_NONCE_EXP, EPHEMERAL_CLIENTS_CACHE_LIFETIME, POW_EXP,
    RAUTHY_VERSION, SWAGGER_UI_EXTERNAL, SWAGGER_UI_INTERNAL, WEBAUTHN_DATA_EXP, WEBAUTHN_REQ_EXP,
};
use rauthy_common::error_response::ErrorResponse;
use rauthy_common::password_hasher;
use rauthy_common::utils::decrypt_legacy;
use rauthy_handlers::middleware::ip_blacklist::RauthyIpBlacklistMiddleware;
use rauthy_handlers::middleware::logging::RauthyLoggingMiddleware;
use rauthy_handlers::middleware::principal::RauthyPrincipalMiddleware;
use rauthy_handlers::openapi::ApiDoc;
use rauthy_handlers::{
    api_keys, blacklist, clients, events, generic, groups, oidc, roles, scopes, sessions, users,
};
use rauthy_models::app_state::{AppState, Caches};
use rauthy_models::email::EMail;
use rauthy_models::entity::api_keys::ApiKeyEntity;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::jwk::Jwk;
use rauthy_models::events::event::Event;
use rauthy_models::events::health_watch::watch_health;
use rauthy_models::events::listener::EventListener;
use rauthy_models::events::notifier::EventNotifier;
use rauthy_models::events::{init_event_vars, ip_blacklist_handler};
use rauthy_models::migration::check_restore_backup;
use rauthy_models::{email, ListenScheme};
use spow::pow::Pow;
use sqlx::query;
use std::error::Error;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::Duration;
use std::{env, thread};
use tokio::sync::mpsc;
use tokio::time;
use tracing::{debug, error, info};
use utoipa_swagger_ui::SwaggerUi;

use crate::cache_notify::handle_notify;
use crate::logging::setup_logging;

mod cache_notify;
mod logging;
mod schedulers;
mod tls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
    // This sleep is just a test. On some terminals, the banner gets mixed up with the first other
    // logs. We dont care about rauthy's startup time being 1ms longer.
    time::sleep(Duration::from_millis(1)).await;

    // setup logging
    let mut test_mode = false;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "test" {
        test_mode = true;
        dotenvy::from_filename("rauthy.test.cfg").ok();
    } else {
        dotenvy::from_filename("rauthy.cfg").expect("'rauthy.cfg' error");
    }

    let log_level = setup_logging();

    info!("Starting Rauthy v{}", RAUTHY_VERSION);
    info!("Log Level set to '{}'", log_level);
    if test_mode {
        info!("Application started in Integration Test Mode");
    }

    // init encryption keys and pow secrets
    match EncKeys::from_env() {
        Ok(keys) => {
            // for the PoWs, we just use our active keys as b64
            Pow::init_bytes(keys.get_key(&keys.enc_key_active).unwrap());

            keys.init().unwrap()
        }
        Err(err) => {
            error!(
                r#"

    If you are migrating from an earlier version, you must convert your `ENC_KEYS` before starting
    any version v0.20+

    To do this, you need to:

    1. Install cryptr - https://github.com/sebadob/cryptr
    If you have Rust available on your system, just execute:

    cargo install cryptr --features cli --locked

    Otherwise, pre-built binaries do exist for either Linux or Windows: https://github.com/sebadob/cryptr/tree/main/out

    2. Execute:

    cryptr keys convert legacy-string

    3. Paste your current ENC_KEYS into the command line.

    For instance, if you have
    ENC_KEYS="bVCyTsGaggVy5yqQ/S9n7oCen53xSJLzcsmfdnBDvNrqQ63r4 q6u26onRvXVG4427/3CEC8RJWBcMkrBMkRXgx65AmJsNTghSA"
    in your config, paste
    bVCyTsGaggVy5yqQ/S9n7oCen53xSJLzcsmfdnBDvNrqQ63r4 q6u26onRvXVG4427/3CEC8RJWBcMkrBMkRXgx65AmJsNTghSA

    If you provide your ENC_KEYS via a Kubernetes secret, you need to do a base64 decode first.
    For instance, if your secret looks something like this
    ENC_KEYS: YlZDeVRzR2FnZ1Z5NXlxUS9TOW43b0NlbjUzeFNKTHpjc21mZG5CRHZOcnFRNjNyNCBxNnUyNm9uUnZYVkc0NDI3LzNDRUM4UkpXQmNNa3JCTWtSWGd4NjVBbUpzTlRnaFNB
    Then decode via shell or any tool your like:
    echo -n YlZDeVRzR2FnZ1Z5NXlxUS9TOW43b0NlbjUzeFNKTHpjc21mZG5CRHZOcnFRNjNyNCBxNnUyNm9uUnZYVkc0NDI3LzNDRUM4UkpXQmNNa3JCTWtSWGd4NjVBbUpzTlRnaFNB | base64 -d
    ... and paste the decoded value into cryptr

    4. cryptr will output the correct format for either usage in config or as kubernetes secret again

    5. Paste the new format into your Rauthy config / secret and restart.

    "#
            );
            panic!("{}", err);
        }
    }

    // check if a backup should be restored
    if let Err(err) = check_restore_backup().await {
        error!("\nError restoring backup:\n\n{}\n", err.message);
        panic!("{:?}", err);
    }

    // caches
    let (tx_health_state, mut cache_config) = redhac::CacheConfig::new();

    // "infinity" cache
    cache_config.spawn_cache(
        CACHE_NAME_12HR.to_string(),
        redhac::TimedCache::with_lifespan(43200),
        Some(32),
    );

    // auth codes
    cache_config.spawn_cache(
        CACHE_NAME_AUTH_CODES.to_string(),
        redhac::TimedCache::with_lifespan(300 + *WEBAUTHN_REQ_EXP),
        Some(64),
    );

    // DPoP nonces
    cache_config.spawn_cache(
        CACHE_NAME_DPOP_NONCES.to_string(),
        redhac::TimedCache::with_lifespan(*DPOP_NONCE_EXP as u64),
        None,
    );

    // ephemeral clients
    cache_config.spawn_cache(
        CACHE_NAME_EPHEMERAL_CLIENTS.to_string(),
        redhac::TimedCache::with_lifespan(*EPHEMERAL_CLIENTS_CACHE_LIFETIME),
        None,
    );

    // sessions
    cache_config.spawn_cache(
        CACHE_NAME_SESSIONS.to_string(),
        redhac::TimedCache::with_lifespan(
            env::var("SESSION_LIFETIME")
                .unwrap_or_else(|_| String::from("14400"))
                .trim()
                .parse::<u64>()
                .expect("SESSION_LIFETIME cannot be parsed to u64 - bad format"),
        ),
        Some(64),
    );

    // PoWs
    cache_config.spawn_cache(
        CACHE_NAME_POW.to_string(),
        redhac::TimedCache::with_lifespan(*POW_EXP as u64),
        Some(16),
    );

    // Users
    cache_config.spawn_cache(
        CACHE_NAME_POW.to_string(),
        redhac::TimedCache::with_lifespan_and_capacity(
            env::var("CACHE_USERS_LIFESPAN")
                .unwrap_or_else(|_| String::from("28800"))
                .trim()
                .parse::<u64>()
                .expect("CACHE_USERS_LIFESPAN cannot be parsed to u64 - bad format"),
            env::var("CACHE_USERS_SIZE")
                .unwrap_or_else(|_| String::from("100"))
                .trim()
                .parse::<usize>()
                .expect("CACHE_USERS_SIZE cannot be parsed to usize - bad format"),
        ),
        Some(16),
    );

    // webauthn requests
    cache_config.spawn_cache(
        CACHE_NAME_WEBAUTHN.to_string(),
        redhac::TimedCache::with_lifespan(*WEBAUTHN_REQ_EXP),
        Some(32),
    );
    cache_config.spawn_cache(
        CACHE_NAME_WEBAUTHN_DATA.to_string(),
        redhac::TimedCache::with_lifespan(*WEBAUTHN_DATA_EXP),
        Some(32),
    );

    // login delay cache
    cache_config.spawn_cache(
        CACHE_NAME_LOGIN_DELAY.to_string(),
        redhac::SizedCache::with_size(1),
        Some(16),
    );

    // The ha cache must be started after all entries have been added to the cache map
    let (tx_notify, rx_notify) = mpsc::channel(64);
    redhac::start_cluster(tx_health_state, &mut cache_config, Some(tx_notify), None).await?;

    // email sending
    let (tx_email, rx_email) = mpsc::channel::<EMail>(16);
    tokio::spawn(email::sender(rx_email, test_mode));

    // build the application state
    let caches = Caches {
        ha_cache_config: cache_config.clone(),
    };

    let (tx_events, rx_events) = flume::unbounded();
    let (tx_events_router, rx_events_router) = flume::unbounded();
    let (tx_ip_blacklist, rx_ip_blacklist) = flume::unbounded();

    let app_state = web::Data::new(
        AppState::new(
            tx_email.clone(),
            tx_events.clone(),
            tx_events_router.clone(),
            tx_ip_blacklist.clone(),
            caches,
        )
        .await?,
    );

    // events listener
    init_event_vars().unwrap();
    EventNotifier::init_notifiers(tx_email).await.unwrap();
    tokio::spawn(EventListener::listen(
        tx_ip_blacklist.clone(),
        tx_events_router,
        rx_events_router,
        rx_events,
        app_state.db.clone(),
    ));

    // spawn password hash limiter
    tokio::spawn(password_hasher::run());

    // spawn ip blacklist handler
    tokio::spawn(ip_blacklist_handler::run(tx_ip_blacklist, rx_ip_blacklist));

    // spawn remote cache notification service
    tokio::spawn(handle_notify(app_state.clone(), rx_notify));

    // TODO remove with v0.21
    // migrate existing encrypted values to `cryptr`
    V20_migrate_to_cryptr(&app_state)
        .await
        .expect("TEMP_migrate_to_cryptr to succeed");

    // spawn health watcher
    tokio::spawn(watch_health(
        app_state.db.clone(),
        app_state.tx_events.clone(),
        app_state.caches.ha_cache_config.rx_health_state.clone(),
    ));

    // schedulers
    match env::var("SCHED_DISABLE")
        .unwrap_or_else(|_| String::from("false"))
        .as_str()
    {
        "true" => {
            info!("Schedulers are disabled");
        }
        _ => {
            tokio::spawn(schedulers::scheduler_main(app_state.clone()));
        }
    };

    // make sure, that all caches are cleared from possible inconsistent leftovers from the migrations
    if let Err(err) = redhac::clear_caches(&cache_config).await {
        error!("Error clearing cache after migrations: {}", err.error);
    }

    // actix web
    let state = app_state.clone();
    let actix = thread::spawn(move || {
        let actix_system = actix_web::rt::System::new();
        actix_system.block_on(actix_main(state)).map_err(|e| {
            error!("{}", e);
        })
    });

    actix.join().unwrap().unwrap();
    app_state.caches.ha_cache_config.shutdown().await.unwrap();

    Ok(())
}

// #[actix_web::main]
async fn actix_main(app_state: web::Data<AppState>) -> std::io::Result<()> {
    debug!(
        "Actix Main Thread is running on {:?}",
        thread::current().id()
    );

    let listen_scheme = app_state.listen_scheme.clone();
    let listen_addr = app_state.listen_addr.clone();

    // custom number of workers
    let mut workers = env::var("HTTP_WORKERS")
        .unwrap_or_else(|_| String::from("0"))
        .parse::<usize>()
        .expect("Unable to parse HTTP_WORKERS");
    if workers == 0 {
        workers = num_cpus::get();
    }

    // OpenAPI / Swagger
    let swagger = SwaggerUi::new("/docs/v1/swagger-ui/{_:.*}")
        .url("/docs/v1/api-doc/openapi.json", ApiDoc::build(&app_state))
        .config(
            utoipa_swagger_ui::Config::from("../api-doc/openapi.json").try_it_out_enabled(false),
        );

    // Prometheus metrics
    let metrics_enable = env::var("METRICS_ENABLE")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .expect("Cannot parse METRICS_ENABLE to bool");
    let pub_metrics = if metrics_enable {
        let shared_registry = Registry::new();
        let metrics = PrometheusMetricsBuilder::new("api")
            .registry(shared_registry.clone())
            .endpoint("/metrics")
            .exclude("/favicon.ico")
            .exclude("/metrics")
            .build()
            .unwrap();

        let swagger_clone = swagger.clone();
        thread::spawn(move || {
            let addr = env::var("METRICS_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string());
            let port = env::var("METRICS_PORT").unwrap_or_else(|_| "9090".to_string());
            if let Err(err) = Ipv4Addr::from_str(&addr) {
                let msg = format!("Error parsing METRICS_ADDR: {}", err);
                error!(msg);
                panic!("{}", msg);
            }
            let addr_full = format!("{}:{}", addr, port);

            info!("Metrics available on: http://{}/metrics", addr_full);
            let srv = if *SWAGGER_UI_INTERNAL {
                info!(
                    "Serving Swagger UI internally on: http://{}/docs/v1/swagger-ui/",
                    addr_full
                );
                HttpServer::new(move || {
                    App::new()
                        .wrap(metrics.clone())
                        .service(swagger_clone.clone())
                })
                .workers(1)
                .bind(addr_full)
                .unwrap()
                .run()
            } else {
                HttpServer::new(move || App::new().wrap(metrics.clone()))
                    .workers(1)
                    .bind(addr_full)
                    .unwrap()
                    .run()
            };
            System::new().block_on(srv).unwrap();
        });

        PrometheusMetricsBuilder::new("rauthy")
            .registry(shared_registry)
            // no endpoint means it will not expose one, only collect data
            .exclude("/favicon.ico")
            .exclude("/metrics")
            .build()
            .unwrap()
    } else {
        PrometheusMetricsBuilder::new("rauthy")
            // no endpoint means it will not expose one, only collect data
            .exclude_regex(".*")
            .build()
            .unwrap()
    };

    // send start event
    app_state
        .tx_events
        .send_async(Event::rauthy_started())
        .await
        .unwrap();

    // Note: all .wrap's are executed in reverse order -> the last .wrap is executed as the first
    // one for any new request
    let server = HttpServer::new(move || {
        let mut app = App::new()
            // .data shares application state for all workers
            .app_data(app_state.clone())
            .wrap(RauthyPrincipalMiddleware)
            .wrap(RauthyLoggingMiddleware)
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("x-frame-options", "SAMEORIGIN"))
                    .add(("x-xss-protection", "1;mode=block"))
                    .add(("x-content-type-options", "nosniff"))
                    .add(("X-Robots-Tag", "noindex, nofollow"))
                    .add((
                        "strict-transport-security",
                        "max-age=31536000;includeSubDomains",
                    ))
                    .add(("referrer-policy", "no-referrer"))
                    .add(("x-robots-tag", "none"))
                    .add((
                        "content-security-policy",
                        "frame-ancestors 'none'; object-src 'none';",
                    ))
                    .add(("cache-control", "no-store"))
                    .add(("pragma", "no-cache")),
            )
            .wrap(pub_metrics.clone())
            .service(generic::redirect)
            // Important: Do not move this middleware do need the least amount of computing
            // for blacklisted IPs -> middlewares are executed in reverse order -> this one first
            .wrap(RauthyIpBlacklistMiddleware)
            .service(
                web::scope("/auth")
                    .service(generic::redirect_v1)
                    .service(users::get_user_webid)
                    .service(
                        web::scope("/v1")
                            .service(api_keys::get_api_keys)
                            .service(api_keys::post_api_key)
                            .service(api_keys::put_api_key)
                            .service(api_keys::delete_api_key)
                            .service(api_keys::get_api_key_test)
                            .service(api_keys::put_api_key_secret)
                            .service(blacklist::get_blacklist)
                            .service(blacklist::post_blacklist)
                            .service(blacklist::delete_blacklist)
                            .service(events::post_events)
                            .service(events::sse_events)
                            .service(events::post_event_test)
                            .service(generic::get_index)
                            .service(generic::get_account_html)
                            .service(generic::get_admin_html)
                            .service(generic::get_admin_api_keys_html)
                            .service(generic::get_admin_attr_html)
                            .service(generic::get_admin_blacklist_html)
                            .service(generic::get_admin_clients_html)
                            .service(generic::get_admin_config_html)
                            .service(generic::get_admin_docs_html)
                            .service(generic::get_admin_groups_html)
                            .service(generic::get_admin_roles_html)
                            .service(generic::get_admin_scopes_html)
                            .service(generic::get_admin_sessions_html)
                            .service(generic::get_admin_users_html)
                            .service(generic::get_auth_check)
                            .service(generic::get_auth_check_admin)
                            .service(generic::post_i18n)
                            .service(generic::post_update_language)
                            .service(generic::get_version)
                            .service(oidc::get_authorize)
                            .service(oidc::post_authorize)
                            .service(oidc::post_authorize_refresh)
                            .service(oidc::get_callback_html)
                            .service(oidc::get_certs)
                            .service(oidc::get_cert_by_kid)
                            .service(oidc::get_logout)
                            .service(oidc::post_logout)
                            .service(oidc::rotate_jwk)
                            .service(oidc::get_session_info)
                            .service(oidc::get_session_xsrf)
                            .service(clients::get_clients)
                            .service(clients::get_client_by_id)
                            .service(clients::get_client_colors)
                            .service(clients::put_client_colors)
                            .service(clients::delete_client_colors)
                            .service(clients::get_client_logo)
                            .service(clients::put_client_logo)
                            .service(clients::delete_client_logo)
                            .service(clients::get_client_secret)
                            .service(clients::post_clients)
                            .service(clients::put_clients)
                            .service(clients::put_generate_client_secret)
                            .service(clients::delete_client)
                            .service(generic::get_login_time)
                            .service(users::get_users)
                            .service(users::get_users_register)
                            .service(users::post_users_register)
                            .service(users::get_cust_attr)
                            .service(users::post_cust_attr)
                            .service(users::put_cust_attr)
                            .service(users::delete_cust_attr)
                            .service(users::get_user_by_id)
                            .service(users::get_user_attr)
                            .service(users::put_user_attr)
                            .service(users::get_user_webid_data)
                            .service(users::put_user_webid_data)
                            .service(users::get_user_email_confirm)
                            .service(users::post_user_self_convert_passkey)
                            .service(generic::post_password_hash_times)
                            .service(sessions::get_sessions)
                            .service(sessions::delete_sessions)
                            .service(sessions::delete_sessions_for_user)
                            .service(users::get_user_password_reset)
                            .service(users::put_user_password_reset)
                            .service(users::get_user_by_email)
                            .service(users::post_users)
                            .service(users::put_user_by_id)
                            .service(users::put_user_self)
                            .service(users::delete_user_by_id)
                            .service(users::post_user_password_request_reset)
                            .service(users::get_user_webauthn_passkeys)
                            .service(users::post_webauthn_reg_start)
                            .service(users::post_webauthn_reg_finish)
                            .service(users::post_webauthn_auth_start)
                            .service(users::post_webauthn_auth_finish)
                            .service(users::delete_webauthn)
                            .service(generic::get_password_policy)
                            .service(generic::put_password_policy)
                            .service(generic::get_pow)
                            .service(groups::get_groups)
                            .service(groups::post_group)
                            .service(groups::put_group)
                            .service(groups::delete_group)
                            .service(roles::get_roles)
                            .service(roles::post_role)
                            .service(roles::put_role)
                            .service(roles::delete_role)
                            .service(scopes::get_scopes)
                            .service(scopes::post_scope)
                            .service(scopes::put_scope)
                            .service(scopes::delete_scope)
                            .service(oidc::post_token)
                            .service(oidc::post_token_info)
                            .service(oidc::get_userinfo)
                            .service(generic::get_enc_keys)
                            .service(generic::post_migrate_enc_key)
                            .service(generic::ping)
                            .service(oidc::post_validate_token)
                            .service(oidc::get_well_known)
                            .service(generic::get_health)
                            .service(generic::get_ready)
                            .service(generic::whoami)
                            .service(generic::get_static_assets),
                    ),
            );

        if *SWAGGER_UI_EXTERNAL {
            app = app.service(swagger.clone());
        }

        app
    })
    // overwrites the number of worker threads -> default == available cpu cores
    .workers(workers)
    .shutdown_timeout(10);

    match listen_scheme {
        ListenScheme::Http => {
            server
                .bind(format!("{}:{}", &listen_addr, get_http_port()))?
                .run()
                .await
        }

        ListenScheme::Https => {
            server
                .bind_rustls_021(
                    format!("{}:{}", &listen_addr, get_https_port()),
                    tls::load_tls().await,
                )?
                .run()
                .await
        }

        ListenScheme::HttpHttps => {
            server
                .bind(format!("{}:{}", &listen_addr, get_http_port()))?
                .bind_rustls_021(
                    format!("{}:{}", &listen_addr, get_https_port()),
                    tls::load_tls().await,
                )?
                .run()
                .await
        }
    }
}

fn get_http_port() -> String {
    let port = env::var("LISTEN_PORT_HTTP").unwrap_or_else(|_| "8080".to_string());
    info!("HTTP listen port: {}", port);
    port
}

fn get_https_port() -> String {
    let port = env::var("LISTEN_PORT_HTTPS").unwrap_or_else(|_| "8443".to_string());
    info!("HTTPS listen port: {}", port);
    port
}

// TODO remove with v0.21
async fn V20_migrate_to_cryptr(app_state: &Data<AppState>) -> Result<(), ErrorResponse> {
    // check `config` table if the migration has been done already
    let db_id = "cryptr_migration";
    if sqlx::query!("SELECT * FROM config WHERE id = $1", db_id)
        .fetch_one(&app_state.db)
        .await
        .is_ok()
    {
        debug!("Secrets have already been migrated - skipping");
        return Ok(());
    }

    info!("Starting secrets migration");

    let keys = EncKeys::get_static();
    let active_key = &keys.enc_key_active;

    // if not, migrate values wrapped in a transaction to not screw up something
    let mut txn = app_state.db.begin().await?;

    // migrate clients
    let clients = Client::find_all(app_state).await?;
    for mut client in clients {
        if !client.confidential {
            continue;
        }

        let kid = client.secret_kid.as_ref().unwrap();
        let key = keys.get_key(kid)?;

        let dec = decrypt_legacy(client.secret.as_ref().unwrap(), key)?;
        let enc = EncValue::encrypt(dec.as_slice())?.into_bytes().to_vec();

        client.secret = Some(enc);
        client.secret_kid = Some(active_key.clone());
        client.save(app_state, Some(&mut txn)).await?;
    }

    // migrate Jwks
    let jwks = sqlx::query_as!(Jwk, "select * from jwks")
        .fetch_all(&app_state.db)
        .await?;
    for jwk in jwks {
        let key = keys.get_key(&jwk.enc_key_id)?;
        let dec = decrypt_legacy(jwk.jwk.as_slice(), key)?;
        let enc = EncValue::encrypt(dec.as_slice())?.into_bytes().to_vec();

        sqlx::query!(
            "update jwks set enc_key_id = $1, jwk = $2 where kid = $3",
            active_key,
            enc,
            jwk.kid,
        )
        .execute(&mut *txn)
        .await?;
    }

    // migrate ApiKey's
    let api_keys = ApiKeyEntity::find_all(app_state).await?;
    for mut api_key in api_keys {
        // secret
        let key = keys.get_key(&api_key.enc_key_id)?;
        let dec = decrypt_legacy(api_key.secret.as_slice(), key)?;
        api_key.secret = EncValue::encrypt(dec.as_slice())?.into_bytes().to_vec();

        // access rights
        let dec = decrypt_legacy(api_key.access.as_slice(), key)?;
        api_key.access = EncValue::encrypt(dec.as_slice())?.into_bytes().to_vec();

        query!(
            r#"UPDATE api_keys
            SET secret = $1, expires = $2, enc_key_id = $3, access = $4
            WHERE name = $5"#,
            api_key.secret,
            api_key.expires,
            active_key,
            api_key.access,
            api_key.name,
        )
        .execute(&mut *txn)
        .await?;
    }

    // save the state in `config` table to only do all this once
    #[cfg(not(feature = "sqlite"))]
    sqlx::query!("INSERT INTO config (id, data) VALUES ($1, $2)", db_id, &[])
        .execute(&mut *txn)
        .await?;
    #[cfg(feature = "sqlite")]
    sqlx::query!("INSERT INTO config (id, data) VALUES ($1, $2)", db_id, 1)
        .execute(&mut *txn)
        .await?;

    txn.commit().await?;

    info!("Secrets migration successful");

    // TODO maybe encrypt webauthn data while we are doing it already anyway?

    // TODO check the user's database and convert all existing emails to lowercase only to avoid
    // duplicate issues if a user is added more than once just with an uppercase character

    Ok(())
}
