use crate::tls;
use actix_web::rt::System;
use actix_web::{App, HttpServer, middleware, web};
use actix_web_prom::PrometheusMetricsBuilder;
use prometheus::Registry;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::UseDummyAddress;
use rauthy_data::ListenScheme;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_handlers::{
    api_keys, atproto, auth_providers, backup, blacklist, clients, cors_preflight, dev_only, email,
    events, fed_cm, generic, groups, html, oidc, pam, roles, scopes, sessions, swagger_ui, themes,
    tos, users,
};
use rauthy_middlewares::csrf_protection::CsrfProtectionMiddleware;
use rauthy_middlewares::ip_blacklist::RauthyIpBlacklistMiddleware;
use rauthy_middlewares::logging::RauthyLoggingMiddleware;
use rauthy_middlewares::principal::RauthyPrincipalMiddleware;
use std::cmp::max;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::thread;
use tracing::{error, info};
// TODO Currently, we have some duplicated code in here for building the HttpServer.
// This is due to the strict typing from actix_web. We want to be able to conditionally `.wrap`
// the server with an optional prometheus metrics collector.
// The issue here is, that any additional `.wrap()` changes the auto-derived type for
// `actix_web::App` and we can't make it "pretty" easily, as we can do it for additional services.
//
// There is most probably a way to do this when we wrap it inside something like
// `Box<dyn ServiceFactory<_>>`, but I have not figured out the correct type for that yet.

pub async fn server_with_metrics() -> std::io::Result<()> {
    let listen_scheme = RauthyConfig::get().listen_scheme.clone();
    let listen_addr = RauthyConfig::get().vars.server.listen_address.to_string();

    let shared_registry = Registry::new();
    let metrics = PrometheusMetricsBuilder::new("api")
        .registry(shared_registry.clone())
        .endpoint("/metrics")
        .exclude("/favicon.ico")
        .exclude("/metrics")
        .build()
        .unwrap();

    thread::spawn(move || {
        let vars = &RauthyConfig::get().vars.server;
        if let Err(err) = Ipv4Addr::from_str(&vars.metrics_addr) {
            let msg = format!("Error parsing METRICS_ADDR: {err}");
            error!(msg);
            panic!("{}", msg);
        }
        let addr_full = format!("{}:{}", vars.metrics_addr, vars.metrics_port);

        info!("Metrics available on: http://{addr_full}/metrics");
        // TODO create single threaded runtime specifically -> probably use tokio
        System::new()
            .block_on(
                HttpServer::new(move || App::new().wrap(metrics.clone()))
                    .workers(1)
                    .bind(addr_full)
                    .unwrap()
                    .run(),
            )
            .unwrap();
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
            // Important: Do not move this middleware do need the least amount of computing
            // for blacklisted IPs -> middlewares are executed in reverse order -> this one first
            .wrap(RauthyIpBlacklistMiddleware)
            .service(api_services())
            .service(generic::catch_all);

        #[cfg(not(target_os = "windows"))]
        if matches!(
            &RauthyConfig::get().listen_scheme,
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
                .bind(format!(
                    "{listen_addr}:{}",
                    RauthyConfig::get().vars.server.port_http
                ))?
                .run()
                .await
        }

        ListenScheme::Https => {
            server
                .bind_rustls_0_23(
                    format!(
                        "{listen_addr}:{}",
                        RauthyConfig::get().vars.server.port_https
                    ),
                    tls::load_tls().await,
                )?
                .run()
                .await
        }

        ListenScheme::HttpHttps => {
            server
                .bind(format!(
                    "{listen_addr}:{}",
                    RauthyConfig::get().vars.server.port_http
                ))?
                .bind_rustls_0_23(
                    format!(
                        "{listen_addr}:{}",
                        RauthyConfig::get().vars.server.port_https
                    ),
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

pub async fn server_without_metrics() -> std::io::Result<()> {
    let listen_scheme = RauthyConfig::get().listen_scheme.clone();
    let listen_addr = RauthyConfig::get().vars.server.listen_address.to_string();

    let server = HttpServer::new(move || {
        let mut app = App::new()
            .wrap(RauthyLoggingMiddleware)
            .wrap(RauthyPrincipalMiddleware)
            .wrap(CsrfProtectionMiddleware)
            .wrap(default_headers())
            .service(oidc::get_well_known)
            .service(fed_cm::get_fed_cm_well_known)
            // Important: Do not move this middleware do need the least amount of computing
            // for blacklisted IPs -> middlewares are executed in reverse order -> this one first
            .wrap(RauthyIpBlacklistMiddleware)
            .service(api_services())
            .service(generic::catch_all);

        #[cfg(not(target_os = "windows"))]
        if matches!(
            &RauthyConfig::get().listen_scheme,
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
                .bind(format!(
                    "{listen_addr}:{}",
                    RauthyConfig::get().vars.server.port_http
                ))?
                .run()
                .await
        }

        ListenScheme::Https => {
            server
                .bind_rustls_0_23(
                    format!(
                        "{listen_addr}:{}",
                        RauthyConfig::get().vars.server.port_https
                    ),
                    tls::load_tls().await,
                )?
                .run()
                .await
        }

        ListenScheme::HttpHttps => {
            server
                .bind(format!(
                    "{listen_addr}:{}",
                    RauthyConfig::get().vars.server.port_http
                ))?
                .bind_rustls_0_23(
                    format!(
                        "{listen_addr}:{}",
                        RauthyConfig::get().vars.server.port_https
                    ),
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
                .service(atproto::get_atproto_client_metadata)
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
                .service(auth_providers::delete_provider_img)
                .service(auth_providers::post_provider_link)
                .service(backup::get_backups)
                .service(backup::post_backup)
                .service(backup::get_backup_local)
                .service(backup::get_backup_s3)
                .service(blacklist::get_blacklist)
                .service(blacklist::post_blacklist)
                .service(blacklist::delete_blacklist)
                .service(cors_preflight::options_clients_dyn)
                .service(cors_preflight::options_authorize)
                .service(cors_preflight::options_certs)
                .service(cors_preflight::options_certs_by_kid)
                .service(cors_preflight::options_logout)
                .service(cors_preflight::options_token)
                .service(cors_preflight::options_token_revoke)
                .service(cors_preflight::options_introspect)
                .service(cors_preflight::options_userinfo)
                .service(cors_preflight::options_users_picture)
                .service(cors_preflight::options_users_register)
                .service(cors_preflight::options_atproto_metadata)
                .service(cors_preflight::options_openid_configuration)
                .service(email::get_email_jobs)
                .service(email::post_send_email)
                .service(email::post_email_job_cancel)
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
                .service(html::get_admin_config_backups_html)
                .service(html::get_admin_config_encryption_html)
                .service(html::get_admin_config_jwks_html)
                .service(html::get_admin_config_policy_html)
                .service(html::get_admin_docs_html)
                .service(html::get_admin_events_html)
                .service(html::get_admin_groups_html)
                .service(html::get_admin_roles_html)
                .service(html::get_admin_scopes_html)
                .service(html::get_admin_pam_html)
                .service(html::get_admin_providers_html)
                .service(html::get_admin_sessions_html)
                .service(html::get_admin_users_html)
                .service(html::get_device_html)
                .service(html::get_fed_cm_html)
                .service(generic::get_auth_check)
                .service(generic::get_auth_check_admin)
                .service(generic::get_timezones)
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
                .service(clients::get_forward_auth_oidc)
                .service(clients::get_forward_auth_callback)
                .service(generic::get_login_time)
                .service(fed_cm::get_fed_cm_accounts)
                .service(fed_cm::get_fed_cm_config)
                .service(fed_cm::get_fed_cm_client_meta)
                .service(fed_cm::get_fed_cm_well_known)
                .service(fed_cm::post_fed_cm_token)
                .service(fed_cm::get_fed_client_config)
                .service(fed_cm::get_fed_cm_status)
                .service(pam::get_pam_emails_unlinked)
                .service(pam::get_pam_groups)
                .service(pam::post_pam_groups)
                .service(pam::delete_pam_group)
                .service(pam::get_pam_group_hosts_count)
                .service(pam::get_hosts)
                .service(pam::post_hosts)
                .service(pam::get_hosts_user_access)
                .service(pam::get_host_details)
                .service(pam::put_host)
                .service(pam::post_host_secret)
                .service(pam::put_host_secret)
                .service(pam::delete_host)
                .service(pam::post_host_whoami)
                .service(pam::post_getent)
                .service(pam::post_login)
                .service(pam::post_mfa_start)
                .service(pam::post_mfa_finish)
                .service(pam::post_preflight)
                .service(pam::get_pam_users)
                .service(pam::post_pam_users)
                .service(pam::post_pam_password)
                .service(pam::get_pam_user_self)
                .service(pam::post_pam_user_self_authorized_keys)
                .service(pam::delete_pam_user_self_authorized_keys)
                .service(pam::get_pam_users_authorized_keys)
                .service(pam::get_pam_user)
                .service(pam::put_pam_user)
                .service(pam::delete_pam_user_authorized_key)
                .service(pam::get_validate_user)
                .service(users::get_users)
                .service(users::get_users_register)
                .service(users::post_users_register)
                .service(html::get_user_password_reset_fixed)
                .service(users::get_user_values_config)
                .service(users::get_cust_attr)
                .service(users::post_cust_attr)
                .service(users::put_cust_attr)
                .service(users::delete_cust_attr)
                .service(users::get_user_picture_config)
                .service(users::get_user_by_id)
                .service(users::get_user_attr)
                .service(users::get_user_attr_editable)
                .service(users::put_user_attr)
                .service(users::post_user_mfa_token)
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
                .service(users::put_user_self_preferred_username)
                .service(generic::post_password_hash_times)
                .service(sessions::get_sessions)
                .service(sessions::delete_sessions)
                .service(sessions::delete_session_by_id)
                .service(sessions::delete_sessions_for_user)
                .service(tos::get_tos)
                .service(tos::post_tos)
                .service(tos::get_tos_latest)
                .service(tos::get_tos_user_status)
                .service(tos::post_tos_accept)
                .service(tos::post_tos_deny)
                .service(users::get_user_password_reset)
                .service(users::put_user_password_reset)
                .service(users::get_user_by_email)
                .service(users::post_users)
                .service(users::put_user_by_id)
                .service(users::patch_user)
                .service(users::put_user_self)
                .service(users::get_user_self_delete_config)
                .service(users::delete_user_self)
                .service(users::delete_user_by_id)
                .service(users::post_user_password_request_reset)
                .service(users::get_user_revoke)
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
                .service(oidc::post_token_revoke)
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
                .service(swagger_ui::get_openapi_doc)
                .service(swagger_ui::get_swagger_ui)
                .service(html::get_static_assets),
        )
}

fn workers() -> usize {
    let vars = &RauthyConfig::get().vars;
    let mut workers = vars.server.http_workers as usize;
    if workers == 0 {
        let cores = num_cpus::get();
        if cores < 4 {
            workers = 1;
        } else {
            let reserve = if is_hiqlite() { 2 } else { 1 };
            workers = max(
                2,
                cores as i64 - vars.hashing.max_hash_threads as i64 - reserve,
            ) as usize;
        }
    }
    workers
}
