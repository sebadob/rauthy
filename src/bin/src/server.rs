use crate::tls;
use actix_web::rt::System;
use actix_web::{App, HttpServer, middleware, web};
use actix_web_prom::PrometheusMetricsBuilder;
use prometheus::Registry;
use rauthy_common::constants::{SWAGGER_UI_EXTERNAL, SWAGGER_UI_INTERNAL};
use rauthy_common::utils::UseDummyAddress;
use rauthy_handlers::openapi::ApiDoc;
use rauthy_handlers::{
    api_keys, auth_providers, blacklist, clients, dev_only, events, fed_cm, generic, groups, html,
    oidc, roles, scopes, sessions, themes, users,
};
use rauthy_middlewares::csrf_protection::CsrfProtectionMiddleware;
use rauthy_middlewares::ip_blacklist::RauthyIpBlacklistMiddleware;
use rauthy_middlewares::logging::RauthyLoggingMiddleware;
use rauthy_middlewares::principal::RauthyPrincipalMiddleware;
use rauthy_models::ListenScheme;
use rauthy_models::app_state::AppState;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::{env, thread};
use tracing::{error, info};
use utoipa_swagger_ui::SwaggerUi;

// TODO Currently, we have some duplicated code in here for building the HttpServer.
// This is due to the strict typing from actix_web. We want to be able to conditionally `.wrap`
// the server with an optional prometheus metrics collector.
// The issue here is, that any additional `.wrap()` changes the auto-derived type for
// `actix_web::App` and we can't make it "pretty" easily, as we can do it for additional services.
//
// There is most probably a way to do this when we wrap it inside something like
// `Box<dyn ServiceFactory<_>>`, but I have not figured out the correct type for that yet.

pub async fn server_with_metrics(app_state: web::Data<AppState>) -> std::io::Result<()> {
    let listen_scheme = app_state.listen_scheme.clone();
    let listen_addr = app_state.listen_addr.clone();

    let shared_registry = Registry::new();
    let metrics = PrometheusMetricsBuilder::new("api")
        .registry(shared_registry.clone())
        .endpoint("/metrics")
        .exclude("/favicon.ico")
        .exclude("/metrics")
        .build()
        .unwrap();

    let swagger_internal = if *SWAGGER_UI_INTERNAL {
        Some(swagger_ui(&app_state))
    } else {
        None
    };

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
        let srv = if let Some(swagger_ui) = swagger_internal {
            info!(
                "Serving Swagger UI internally on: http://{}/docs/v1/swagger-ui/",
                addr_full
            );
            HttpServer::new(move || App::new().wrap(metrics.clone()).service(swagger_ui.clone()))
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

    let metrics_collector = PrometheusMetricsBuilder::new("rauthy")
        .registry(shared_registry)
        // no endpoint means it will not expose one, only collect data
        .exclude("/favicon.ico")
        .exclude("/metrics")
        .build()
        .unwrap();

    let server = HttpServer::new(move || {
        let mut app = App::new()
            .wrap(RauthyLoggingMiddleware)
            .app_data(app_state.clone())
            .wrap(RauthyPrincipalMiddleware)
            .wrap(CsrfProtectionMiddleware)
            .wrap(default_headers())
            // TODO currently, there is no nice way to conditionally add the public metrics
            // collecting, apart from specifying the whole server builder twice at this point.
            // This is due to the very strict typing. Maybe we can extract most builders into
            // separate functions somehow to get rid of it with disabled metrics.
            //
            // It is most probably possible to define `app` with some form of `Box<dyn ...>` to be
            // able to build 2 different versions of `app` at this point, but I have not figured
            // out the correct type yet.
            .wrap(metrics_collector.clone())
            .service(oidc::get_well_known)
            .service(fed_cm::get_fed_cm_well_known)
            .service(generic::catch_all)
            // Important: Do not move this middleware do need the least amount of computing
            // for blacklisted IPs -> middlewares are executed in reverse order -> this one first
            .wrap(RauthyIpBlacklistMiddleware)
            .service(api_services());

        if *SWAGGER_UI_EXTERNAL {
            app = app.service(swagger_ui(&app_state));
        }

        #[cfg(not(target_os = "windows"))]
        if matches!(
            &app_state.listen_scheme,
            ListenScheme::UnixHttp | ListenScheme::UnixHttps
        ) {
            app = app.app_data(UseDummyAddress);
        }

        app
    })
    // overwrites the number of worker threads -> default == available cpu cores
    .workers(workers())
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

pub async fn server_without_metrics(app_state: web::Data<AppState>) -> std::io::Result<()> {
    let listen_scheme = app_state.listen_scheme.clone();
    let listen_addr = app_state.listen_addr.clone();

    let server = HttpServer::new(move || {
        let mut app = App::new()
            .wrap(RauthyLoggingMiddleware)
            .app_data(app_state.clone())
            .wrap(RauthyPrincipalMiddleware)
            .wrap(CsrfProtectionMiddleware)
            .wrap(default_headers())
            .service(oidc::get_well_known)
            .service(fed_cm::get_fed_cm_well_known)
            .service(generic::catch_all)
            // Important: Do not move this middleware do need the least amount of computing
            // for blacklisted IPs -> middlewares are executed in reverse order -> this one first
            .wrap(RauthyIpBlacklistMiddleware)
            .service(api_services());

        if *SWAGGER_UI_EXTERNAL {
            app = app.service(swagger_ui(&app_state));
        }

        #[cfg(not(target_os = "windows"))]
        if matches!(
            &app_state.listen_scheme,
            ListenScheme::UnixHttp | ListenScheme::UnixHttps
        ) {
            app = app.app_data(UseDummyAddress);
        }

        app
    })
    // overwrites the number of worker threads -> default == available cpu cores
    .workers(workers())
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

fn default_headers() -> middleware::DefaultHeaders {
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
        .add(("cache-control", "no-store"))
}

fn api_services() -> actix_web::Scope {
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
                .service(oidc::get_well_known)
                .service(generic::get_health)
                .service(generic::get_i18n_config)
                .service(generic::get_ready)
                .service(html::get_static_assets),
        )
}

fn swagger_ui(app_state: &web::Data<AppState>) -> SwaggerUi {
    SwaggerUi::new("/docs/v1/swagger-ui/{_:.*}")
        .url("/docs/v1/api-doc/openapi.json", ApiDoc::build(app_state))
        .config(
            utoipa_swagger_ui::Config::from("../api-doc/openapi.json")
                .try_it_out_enabled(false)
                .supported_submit_methods(["get"])
                .filter(true),
        )
}

fn workers() -> usize {
    let mut workers = env::var("HTTP_WORKERS")
        .as_deref()
        .unwrap_or("0")
        .parse::<usize>()
        .expect("Unable to parse HTTP_WORKERS");
    if workers == 0 {
        workers = num_cpus::get();
    }
    workers
}
