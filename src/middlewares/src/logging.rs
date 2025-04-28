use actix_web::http::Method;
use actix_web::http::header::HeaderValue;
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::LocalBoxFuture;
use rauthy_common::utils::real_ip_from_svc_req;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::env;
use std::future::{Ready, ready};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::LazyLock;
use tracing::{debug, info};

static LOG_LEVEL_ACCESS: LazyLock<LogLevelAccess> = LazyLock::new(|| {
    env::var("LOG_LEVEL_ACCESS")
        .as_deref()
        .unwrap_or("Modifying")
        .parse::<LogLevelAccess>()
        .expect("Cannot parse LOG_LEVEL_ACCESS")
});

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
            log_access(&req).await?;
            service.call(req).await
        })
    }
}

async fn log_access(req: &ServiceRequest) -> Result<(), ErrorResponse> {
    let path = req.uri().path();
    let ip = real_ip_from_svc_req(req)?;

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
