// Copyright 2025 Sebastian Dobe <sebastiandobe@mailbox.org>

use crate::logging::setup_logging;
use actix_web::rt::System;
use actix_web::{App, HttpServer, middleware, web};
use actix_web_prom::PrometheusMetricsBuilder;
use cryptr::EncKeys;
use prometheus::Registry;
use rauthy_common::constants::{
    BUILD_TIME, RAUTHY_VERSION, SWAGGER_UI_EXTERNAL, SWAGGER_UI_INTERNAL,
};
use rauthy_common::utils::UseDummyAddress;
use rauthy_common::{HTTP_CLIENT, is_sqlite, password_hasher};
use rauthy_handlers::generic::I18N_CONFIG;
use rauthy_handlers::openapi::ApiDoc;
use rauthy_handlers::{
    api_keys, auth_providers, blacklist, clients, dev_only, events, fed_cm, generic, groups, html,
    oidc, roles, scopes, sessions, themes, users,
};
use rauthy_middlewares::csrf_protection::CsrfProtectionMiddleware;
use rauthy_middlewares::ip_blacklist::RauthyIpBlacklistMiddleware;
use rauthy_middlewares::logging::RauthyLoggingMiddleware;
use rauthy_middlewares::principal::RauthyPrincipalMiddleware;
use rauthy_models::app_state::AppState;
use rauthy_models::database::{Cache, DB};
use rauthy_models::email::EMail;
use rauthy_models::entity::pictures::UserPicture;
use rauthy_models::events::event::Event;
use rauthy_models::events::health_watch::watch_health;
use rauthy_models::events::listener::EventListener;
use rauthy_models::events::notifier::EventNotifier;
use rauthy_models::events::{init_event_vars, ip_blacklist_handler};
use rauthy_models::{ListenScheme, email};
use spow::pow::Pow;
use std::error::Error;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::Duration;
use std::{env, thread};
use tokio::sync::mpsc;
use tokio::time;
use tracing::{debug, error, info, warn};
use utoipa_swagger_ui::SwaggerUi;

mod dummy_data;
mod logging;
mod tls;
mod version_migration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut test_mode = false;
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
        } else if dotenvy::from_filename("rauthy.cfg").is_err() {
            let cwd = env::current_dir()
                .map(|pb| pb.to_str().unwrap_or_default().to_string())
                .unwrap_or_default();
            warn!(
                "'{}/rauthy.cfg' not found, using environment variables only for configuration",
                cwd
            );
        }

        dotenvy::dotenv_override().ok();
    }

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

    // init encryption keys and pow secrets
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

    // TODO Keep this check in place until v0.28.0 as info for migrations from older versions.
    debug!("Starting the persistence layer");
    if is_sqlite() {
        // Hiqlite migration has been finished.
        panic!(
            r#"

A direct SQLite connection is not supported anymore. The `DATABASE_URL` is only used for
Postgres connections. You can migrate your SQLite database to either Postgres or Hiqlite.
Hiqlite uses SQLite under the hood, but provides a Raft layer on top to make it highly available,
faster, and more resilient.

You can migrate your existing SQLite database using the `MIGRATE_DB_FROM` config variable:

    # If specified, the currently configured Database will be DELETED and OVERWRITTEN with a
    # migration from the given database with this variable. Can be used to migrate between
    # different databases.
    # !!! USE WITH CARE !!!
    #MIGRATE_DB_FROM=sqlite:data/rauthy.db

To migrate to Postgres, simply set the `DATABASE_URL` to a Postgres database.
To migrate to Hiqlite, there are a few new config variables you can set. The most important is
to set

        HIQLITE=true

Take a look at the release notes for more detailed information:
https://github.com/sebadob/rauthy/releases/tag/v0.27.0
"#
        );
    }
    DB::init()
        .await
        .expect("Error starting the database / cache layer");

    // email sending
    debug!("Starting E-Mail handler");
    let (tx_email, rx_email) = mpsc::channel::<EMail>(16);
    tokio::spawn(email::sender(rx_email, test_mode));

    let (tx_events, rx_events) = flume::unbounded();
    let (tx_events_router, rx_events_router) = flume::unbounded();
    let (tx_ip_blacklist, rx_ip_blacklist) = flume::unbounded();

    debug!("Initializing AppState");
    let app_state = web::Data::new(
        AppState::new(
            tx_email.clone(),
            tx_events.clone(),
            tx_events_router.clone(),
            tx_ip_blacklist.clone(),
        )
        .await?,
    );

    debug!("Applying database migrations");
    DB::migrate(&app_state)
        .await
        .expect("Database migration error");

    // events listener
    debug!("Starting Events handler");
    init_event_vars().unwrap();
    EventNotifier::init_notifiers(tx_email).await.unwrap();
    tokio::spawn(EventListener::listen(
        tx_ip_blacklist.clone(),
        tx_events_router,
        rx_events_router,
        rx_events,
    ));

    // spawn password hash limiter
    debug!("Starting Password Hasher");
    tokio::spawn(password_hasher::run());

    // spawn ip blacklist handler
    debug!("Starting Blacklist handler");
    tokio::spawn(ip_blacklist_handler::run(tx_ip_blacklist, rx_ip_blacklist));

    // spawn health watcher
    debug!("Starting health watch");
    tokio::spawn(watch_health(app_state.tx_events.clone()));

    // schedulers
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

    // trigger config builds to have them fast-failing on invalid config
    {
        debug!("I18n Config: {:?}", *I18N_CONFIG);
        debug!("{:?}", *HTTP_CLIENT);
    }

    if args.len() > 1 && args[1] == "dummy-data" {
        let amount = if args.len() > 2 {
            args[2].parse::<u32>().unwrap_or(100_000)
        } else {
            100_000
        };
        tokio::spawn(dummy_data::insert_dummy_data(amount));
    }

    actix_main(app_state).await?;
    DB::hql().shutdown().await.unwrap();

    Ok(())
}

async fn actix_main(app_state: web::Data<AppState>) -> std::io::Result<()> {
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
            utoipa_swagger_ui::Config::from("../api-doc/openapi.json")
                .try_it_out_enabled(false)
                .supported_submit_methods(["get"])
                .filter(true),
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
            .wrap(RauthyLoggingMiddleware)
            .app_data(app_state.clone())
            .wrap(RauthyPrincipalMiddleware)
            .wrap(CsrfProtectionMiddleware)
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("x-frame-options", "DENY"))
                    .add(("x-content-type-options", "nosniff"))
                    .add((
                        "strict-transport-security",
                        "max-age=31536000;includeSubDomains",
                    ))
                    .add(("referrer-policy", "no-referrer"))
                    .add(("x-robots-tag", "noindex, nofollow"))
                    .add((
                        "content-security-policy",
                        "frame-ancestors 'none'; object-src 'none';",
                    ))
                    .add(("cache-control", "no-store")),
            )
            .wrap(pub_metrics.clone())
            .service(oidc::get_well_known)
            .service(fed_cm::get_fed_cm_well_known)
            .service(generic::catch_all)
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
                            .service(auth_providers::post_providers)
                            .service(auth_providers::get_providers_minimal)
                            .service(auth_providers::post_provider)
                            .service(auth_providers::post_provider_login)
                            .service(auth_providers::get_provider_delete_safe)
                            .service(auth_providers::post_provider_lookup)
                            .service(auth_providers::get_provider_callback_html)
                            .service(auth_providers::post_provider_callback)
                            .service(auth_providers::delete_provider_link)
                            .service(auth_providers::put_provider)
                            .service(auth_providers::delete_provider)
                            .service(auth_providers::get_provider_img)
                            .service(auth_providers::put_provider_img)
                            .service(auth_providers::post_provider_link)
                            .service(blacklist::get_blacklist)
                            .service(blacklist::post_blacklist)
                            .service(blacklist::delete_blacklist)
                            .service(events::post_events)
                            .service(events::sse_events)
                            .service(events::post_event_test)
                            .service(dev_only::get_template)
                            .service(dev_only::post_dev_only_endpoints)
                            .service(html::get_index)
                            .service(html::get_account_html)
                            .service(html::get_admin_html)
                            .service(html::get_admin_api_keys_html)
                            .service(html::get_admin_attr_html)
                            .service(html::get_admin_blacklist_html)
                            .service(html::get_admin_clients_html)
                            .service(html::get_admin_config_argon2_html)
                            .service(html::get_admin_config_encryption_html)
                            .service(html::get_admin_config_jwks_html)
                            .service(html::get_admin_config_policy_html)
                            .service(html::get_admin_docs_html)
                            .service(html::get_admin_events_html)
                            .service(html::get_admin_groups_html)
                            .service(html::get_admin_roles_html)
                            .service(html::get_admin_scopes_html)
                            .service(html::get_admin_providers_html)
                            .service(html::get_admin_sessions_html)
                            .service(html::get_admin_users_html)
                            .service(html::get_device_html)
                            .service(html::get_fed_cm_html)
                            .service(generic::get_auth_check)
                            .service(generic::get_auth_check_admin)
                            .service(generic::post_update_language)
                            .service(generic::get_version)
                            .service(generic::get_whoami)
                            .service(oidc::get_authorize)
                            .service(oidc::post_authorize)
                            .service(oidc::post_authorize_refresh)
                            .service(oidc::post_device_auth)
                            .service(oidc::post_device_verify)
                            .service(oidc::get_callback_html)
                            .service(oidc::get_certs)
                            .service(oidc::get_cert_by_kid)
                            .service(oidc::get_logout)
                            .service(oidc::post_logout)
                            .service(oidc::rotate_jwk)
                            .service(oidc::post_session)
                            .service(oidc::get_session_info)
                            .service(oidc::get_session_xsrf)
                            .service(clients::get_clients)
                            .service(clients::get_client_by_id)
                            .service(clients::get_client_logo)
                            .service(clients::put_client_logo)
                            .service(clients::delete_client_logo)
                            .service(clients::get_client_secret)
                            .service(clients::post_clients)
                            .service(clients::put_clients)
                            .service(clients::put_generate_client_secret)
                            .service(clients::delete_client)
                            .service(clients::post_clients_dyn)
                            .service(clients::get_clients_dyn)
                            .service(clients::put_clients_dyn)
                            .service(generic::get_login_time)
                            .service(fed_cm::get_fed_cm_accounts)
                            .service(fed_cm::get_fed_cm_config)
                            .service(fed_cm::get_fed_cm_client_meta)
                            .service(fed_cm::get_fed_cm_well_known)
                            .service(fed_cm::post_fed_cm_token)
                            .service(fed_cm::get_fed_client_config)
                            .service(fed_cm::get_fed_cm_status)
                            .service(users::get_users)
                            .service(users::get_users_register)
                            .service(users::post_users_register)
                            .service(users::get_cust_attr)
                            .service(users::post_cust_attr)
                            .service(users::put_cust_attr)
                            .service(users::delete_cust_attr)
                            .service(users::get_user_picture_config)
                            .service(users::get_user_by_id)
                            .service(users::get_user_attr)
                            .service(users::put_user_attr)
                            .service(users::put_user_picture)
                            .service(users::get_user_picture)
                            .service(users::delete_user_picture)
                            .service(users::get_user_devices)
                            .service(users::put_user_device_name)
                            .service(users::delete_user_device)
                            .service(users::get_user_webid_data)
                            .service(users::put_user_webid_data)
                            .service(users::get_user_email_confirm)
                            .service(users::post_user_self_convert_passkey)
                            .service(generic::post_password_hash_times)
                            .service(sessions::get_sessions)
                            .service(sessions::delete_sessions)
                            .service(sessions::delete_session_by_id)
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
                            .service(generic::post_pow)
                            .service(generic::get_search)
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
                            .service(oidc::post_token_introspect)
                            .service(oidc::get_userinfo)
                            .service(oidc::post_userinfo)
                            .service(oidc::get_forward_auth)
                            .service(generic::get_enc_keys)
                            .service(generic::post_migrate_enc_key)
                            .service(generic::ping)
                            .service(themes::get_theme)
                            .service(themes::post_theme)
                            .service(themes::put_theme)
                            .service(themes::delete_theme)
                            .service(oidc::post_validate_token)
                            .service(oidc::get_well_known)
                            .service(generic::get_health)
                            .service(generic::get_i18n_config)
                            .service(generic::get_ready)
                            .service(html::get_static_assets),
                    ),
            );

        if *SWAGGER_UI_EXTERNAL {
            app = app.service(swagger.clone());
        }

        #[cfg(not(target_os = "windows"))]
        if matches!(
            app_state.listen_scheme,
            ListenScheme::UnixHttp | ListenScheme::UnixHttps
        ) {
            app = app.app_data(UseDummyAddress);
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
                .bind_rustls_0_23(
                    format!("{}:{}", &listen_addr, get_https_port()),
                    tls::load_tls().await,
                )?
                .run()
                .await
        }

        ListenScheme::HttpHttps => {
            server
                .bind(format!("{}:{}", &listen_addr, get_http_port()))?
                .bind_rustls_0_23(
                    format!("{}:{}", &listen_addr, get_https_port()),
                    tls::load_tls().await,
                )?
                .run()
                .await
        }

        #[cfg(not(target_os = "windows"))]
        ListenScheme::UnixHttp | ListenScheme::UnixHttps => {
            server.bind_uds(listen_addr)?.run().await
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
