use actix_web::http::header::HeaderValue;
use actix_web::http::{Method, Uri};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::LocalBoxFuture;
use lazy_static::lazy_static;
use rauthy_common::constants::PROXY_MODE;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use std::env;
use std::future::{ready, Ready};
use std::rc::Rc;
use std::str::FromStr;
use tracing::{debug, info};

lazy_static! {
    static ref LOG_LEVEL_ACCESS: LogLevelAccess = env::var("LOG_LEVEL_ACCESS")
        .unwrap_or_else(|_| "Modifying".to_string())
        .parse::<LogLevelAccess>()
        .expect("Cannot parse LOG_LEVEL_ACCESS");
}

#[derive(Debug, PartialEq, Eq)]
enum LogLevelAccess {
    Debug,
    Verbose,
    Basic,
    Modifying,
    Off,
}

impl FromStr for LogLevelAccess {
    type Err = ErrorResponse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Debug" => Ok(Self::Debug),
            "Verbose" => Ok(Self::Verbose),
            "Basic" => Ok(Self::Basic),
            "Modifying" => Ok(Self::Modifying),
            "Off" => Ok(Self::Off),
            _ => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Cannot parse to LogLevelAccess".to_string(),
            )),
        }
    }
}

pub struct RauthyLoggingMiddleware;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for RauthyLoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct LoggingMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            handle_req(&req).await?;
            service.call(req).await
        })
    }
}

async fn handle_req(req: &ServiceRequest) -> Result<(), ErrorResponse> {
    // let data = req
    //     .app_data::<web::Data<AppState>>()
    //     .expect("Error getting AppData inside logging middleware");
    let uri = req.uri();

    // validate_host(uri, data).await?;
    log_access(uri, req).await?;

    Ok(())
}

async fn log_access(uri: &Uri, req: &ServiceRequest) -> Result<(), ErrorResponse> {
    let path = uri.path();
    let ip = {
        let conn_info = req.connection_info();
        let ip = if *PROXY_MODE {
            conn_info.realip_remote_addr()
        } else {
            conn_info.peer_addr()
        };
        ip.unwrap_or("<UNKNOWN>").to_string()
    };

    match *LOG_LEVEL_ACCESS {
        LogLevelAccess::Debug => {
            debug!("{:?}", req)
        }
        LogLevelAccess::Verbose => {
            info!(
                "{} {:?} {} {} {:?}",
                ip,
                req.version(),
                req.method(),
                path,
                req.headers()
                    .get(actix_web::http::header::USER_AGENT)
                    .unwrap_or(&HeaderValue::from_bytes(b"").unwrap())
            );
        }
        LogLevelAccess::Basic => {
            if path.starts_with("/auth/v1/_app")
                || path.starts_with("/docs/")
                || path.starts_with("/auth/v1/ping")
                || path.starts_with("/auth/v1/health")
            {
                return Ok(());
            }
            info!(
                "{} {:?} {} {} {:?}",
                ip,
                req.version(),
                req.method(),
                path,
                req.headers()
                    .get(actix_web::http::header::USER_AGENT)
                    .unwrap_or(&HeaderValue::from_bytes(b"").unwrap())
            );
        }
        LogLevelAccess::Modifying => {
            if path.starts_with("/auth/v1/_app")
                || path.starts_with("/docs/")
                || req.method() == Method::GET
            {
                return Ok(());
            }
            info!(
                "{} {:?} {} {} {:?}",
                ip,
                req.version(),
                req.method(),
                path,
                req.headers()
                    .get(actix_web::http::header::USER_AGENT)
                    .unwrap_or(&HeaderValue::from_bytes(b"").unwrap())
            );
        }
        LogLevelAccess::Off => return Ok(()),
    }

    Ok(())
}

// async fn validate_host(uri: &Uri, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
//     // host can be none if we are on localhost or is none, if connected via HTTP/2
//     let host = uri.host();
//     if let Some(host) = host {
//         let scheme = uri
//             .scheme()
//             .expect("Scheme is None when Host was Some")
//             .as_str();
//         if !match data.listen_scheme {
//             ListenScheme::Http => scheme == "http",
//             ListenScheme::Https => scheme == "https",
//             ListenScheme::HttpHttps => scheme == "https" || scheme == "http",
//         } {
//             return Err(ErrorResponse::new(
//                 ErrorResponseType::NotFound,
//                 "".to_string(),
//             ));
//         }
//
//         if !data.public_url.contains(host) {
//             return Err(ErrorResponse::new(
//                 ErrorResponseType::NotFound,
//                 "".to_string(),
//             ));
//         }
//     } else {
//         // TODO do something in that case? Check Origin in case of HTTP/2 ?
//         // can this only happen on localhost for 100%? -> investigate
//         debug!("Host is None in logging middleware");
//     }
//
//     Ok(())
// }
