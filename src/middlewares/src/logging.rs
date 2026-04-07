use actix_web::http::Method;
use actix_web::http::header::HeaderValue;
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::LocalBoxFuture;
use rauthy_common::logging::LogLevelAccess;
use rauthy_common::utils::real_ip_from_svc_req;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use std::future::{Ready, ready};
use std::rc::Rc;
use tokio::time::Instant;
use tracing::info;

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
        let time_taken = Instant::now();
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let log = build_log(&req)?;
            let resp = service.call(req).await;
            emit_log(log, time_taken);
            resp
        })
    }
}

#[inline(always)]
fn emit_log(log: Option<String>, inst: Instant) {
    if let Some(log) = log {
        // Technically, this downcast is unsafe. In reality,
        // it will be impossible to ever exceed u64::MAX.
        let elapsed = inst.elapsed().as_micros() as u64;
        if elapsed > 1000 {
            info!("{} {}ms", log, elapsed / 1000);
        } else {
            info!("{} {}μs", log, elapsed);
        }
    }
}

#[inline(always)]
fn build_log(req: &ServiceRequest) -> Result<Option<String>, ErrorResponse> {
    let path = req.uri().path();
    let ip = real_ip_from_svc_req(req)?;

    let log = match RauthyConfig::get().log_level_access {
        LogLevelAccess::Debug => {
            format!("{:?}", req)
        }
        LogLevelAccess::Verbose => {
            format!(
                "{ip} {:?} {} {path} {:?}",
                req.version(),
                req.method(),
                req.headers()
                    .get(actix_web::http::header::USER_AGENT)
                    .unwrap_or(&HeaderValue::from_static(""))
            )
        }
        LogLevelAccess::Basic => {
            if let Some(p) = path.strip_prefix("/auth/v1")
                && (p.starts_with("/_app/")
                    || p.starts_with("/docs/")
                    || p == "/ping"
                    || p == "/health")
            {
                return Ok(None);
            }
            format!(
                "{ip} {:?} {} {path} {:?}",
                req.version(),
                req.method(),
                req.headers()
                    .get(actix_web::http::header::USER_AGENT)
                    .unwrap_or(&HeaderValue::from_static(""))
            )
        }
        LogLevelAccess::Modifying => {
            if req.method() == Method::GET || req.method() == Method::OPTIONS {
                return Ok(None);
            }
            format!(
                "{ip} {:?} {} {path} {:?}",
                req.version(),
                req.method(),
                req.headers()
                    .get(actix_web::http::header::USER_AGENT)
                    .unwrap_or(&HeaderValue::from_static(""))
            )
        }
        LogLevelAccess::Off => return Ok(None),
    };

    Ok(Some(log))
}
