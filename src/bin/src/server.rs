use crate::{api_services, default_headers, get_http_port, get_https_port, tls};
use actix_web::rt::System;
use actix_web::{App, HttpServer, web};
use actix_web_prom::PrometheusMetricsBuilder;
use prometheus::Registry;
use rauthy_common::constants::{SWAGGER_UI_EXTERNAL, SWAGGER_UI_INTERNAL};
use rauthy_common::utils::UseDummyAddress;
use rauthy_handlers::openapi::ApiDoc;
use rauthy_handlers::{fed_cm, generic, oidc};
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
// There is most probably a way to do this when we wrap it inside something like `Box<dyn ...>`,
// but I have not figured out the correct type for that yet.

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
