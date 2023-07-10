// Rauthy - OpenID Connect and Single Sign-On IdP
// Copyright (C) 2023 Sebastian Dobe <sebastiandobe@mailbox.org>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::cache_notify::handle_notify;
use crate::logging::setup_logging;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_grants::GrantsMiddleware;
use rauthy_common::constants::{
    CACHE_NAME_12HR, CACHE_NAME_AUTH_CODES, CACHE_NAME_LOGIN_DELAY, CACHE_NAME_POW,
    CACHE_NAME_SESSIONS, CACHE_NAME_WEBAUTHN, CACHE_NAME_WEBAUTHN_DATA, MFA_REQ_LIFETIME, POW_EXP,
    RAUTHY_VERSION, WEBAUTHN_DATA_EXP, WEBAUTHN_REQ_EXP,
};
use rauthy_common::password_hasher;
use rauthy_handlers::middleware::logging::RauthyLoggingMiddleware;
use rauthy_handlers::middleware::session::RauthySessionMiddleware;
use rauthy_handlers::openapi::ApiDoc;
use rauthy_handlers::{clients, generic, groups, oidc, roles, scopes, sessions, users};
use rauthy_models::app_state::{AppState, Caches};
use rauthy_models::email::EMail;
use rauthy_models::{email, ListenScheme};
use rauthy_service::auth;
use std::error::Error;
use std::time::Duration;
use std::{env, thread};
use tokio::sync::mpsc;
use tokio::time;
use tracing::{debug, error, info};
use utoipa_swagger_ui::SwaggerUi;

mod cache_notify;
mod logging;
mod schedulers;
mod tls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let banner = r#"
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
    .to_string();

    // banner
    println!("{}", banner);
    // just a gimmick, drop it immediately and free up the memory
    drop(banner);
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
        redhac::TimedCache::with_lifespan(300 + *MFA_REQ_LIFETIME),
        Some(64),
    );

    // sessions
    let session_lifetime = env::var("SESSION_LIFETIME")
        .unwrap_or_else(|_| String::from("14400"))
        .trim()
        .parse::<u32>()
        .expect("SESSION_LIFETIME cannot be parsed to u32 - bad format");
    cache_config.spawn_cache(
        CACHE_NAME_SESSIONS.to_string(),
        redhac::TimedCache::with_lifespan(session_lifetime as u64),
        Some(64),
    );

    // PoWs
    cache_config.spawn_cache(
        CACHE_NAME_POW.to_string(),
        redhac::TimedCache::with_lifespan(*POW_EXP),
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
        // tx_ws_listen,
    };
    let app_state = web::Data::new(AppState::new(tx_email, caches).await?);

    // spawn password hash limiter
    tokio::spawn(password_hasher::run());

    // spawn remote cache notification service
    tokio::spawn(handle_notify(app_state.clone(), rx_notify));

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

    let swagger = SwaggerUi::new("/docs/v1/swagger-ui/{_:.*}")
        .url("/docs/v1/api-doc/openapi.json", ApiDoc::build(&app_state))
        .config(
            utoipa_swagger_ui::Config::from("../api-doc/openapi.json").try_it_out_enabled(false),
        );

    // Note: all .wrap's are executed in reverse order -> the last .wrap is executed as the first
    // one for any new request
    let server = HttpServer::new(move || {
        App::new()
            // .data shares application state for all workers
            .app_data(app_state.clone())
            .wrap(GrantsMiddleware::with_extractor(
                auth::permission_extractor,
            ))
            .wrap(RauthySessionMiddleware)
            .wrap(RauthyLoggingMiddleware)
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("x-frame-options", "SAMEORIGIN"))
                    .add(("x-xss-protection", "1;mode=block"))
                    .add(("x-content-type-options", "nosniff"))
                    .add((
                        "strict-transport-security",
                        "max-age=31536000;includeSubDomains",
                    ))
                    .add(("referrer-policy", "no-referrer"))
                    .add(("x-robots-tag", "none"))
                    .add((
                        "content-security-policy",
                        // unsafe-inline is currently needed, since svelte does currently need this
                        // for the initial static hydration script. An issue is open about this and
                        // this will most probably solved soon.
                        "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; frame-ancestors 'self'; object-src 'none'; img-src 'self' data:;",
                    ))
                    .add(("cache-control", "no-store"))
                    .add(("pragma", "no-cache")),
            )
            .service(generic::redirect)
            .service(
                    web::scope("/auth")
                    .service(generic::redirect_v1)
                    .service(
                    web::scope("/v1")
                        .service(generic::get_index)
                        .service(generic::get_account_html)
                        .service(generic::get_admin_html)
                        .service(generic::get_auth_check)
                        .service(generic::get_auth_check_admin)
                        .service(oidc::get_authorize)
                        .service(oidc::post_authorize)
                        .service(oidc::get_callback_html)
                        .service(oidc::post_authorize_refresh)
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
                        .service(users::post_webauthn_reg_start)
                        .service(users::post_webauthn_reg_finish)
                        .service(users::post_webauthn_auth_start)
                        .service(users::post_webauthn_auth_finish)
                        .service(users::post_webauthn_delete)
                        .service(generic::get_password_policy)
                        .service(generic::put_password_policy)
                        .service(generic::get_pow)
                        .service(oidc::post_refresh_token)
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
                        .service(generic::get_static_assets)
                    )
            )
            .service(swagger.clone())
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
                .bind_rustls(
                    format!("{}:{}", &listen_addr, get_https_port()),
                    tls::load_tls().await,
                )?
                .run()
                .await
        }

        ListenScheme::HttpHttps => {
            server
                .bind(format!("{}:{}", &listen_addr, get_http_port()))?
                .bind_rustls(
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
