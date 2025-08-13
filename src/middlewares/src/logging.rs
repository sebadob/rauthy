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
use tracing::{debug, info};

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

    match RauthyConfig::get().log_level_access {
        LogLevelAccess::Debug => {
            debug!("{:?}", req)
        }
        LogLevelAccess::Verbose => {
            info!(
                "{ip} {:?} {} {path} {:?}",
                req.version(),
                req.method(),
                req.headers()
                    .get(actix_web::http::header::USER_AGENT)
                    .unwrap_or(&HeaderValue::from_static(""))
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
                "{ip} {:?} {} {path} {:?}",
                req.version(),
                req.method(),
                req.headers()
                    .get(actix_web::http::header::USER_AGENT)
                    .unwrap_or(&HeaderValue::from_static(""))
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
                "{ip} {:?} {} {path} {:?}",
                req.version(),
                req.method(),
                req.headers()
                    .get(actix_web::http::header::USER_AGENT)
                    .unwrap_or(&HeaderValue::from_static(""))
            );
        }
        LogLevelAccess::Off => return Ok(()),
    }

    Ok(())
}
