use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use chrono::Utc;
use futures::future::LocalBoxFuture;
use rauthy_common::utils::real_ip_from_svc_req;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::entity::ip_blacklist::IpBlacklist;
use rauthy_models::html::templates::TooManyRequestsHtml;
use std::future::{Ready, ready};
use std::rc::Rc;

pub struct RauthyIpBlacklistMiddleware;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for RauthyIpBlacklistMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = IpBlacklistMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(IpBlacklistMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct IpBlacklistMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for IpBlacklistMiddleware<S>
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
            let ip = real_ip_from_svc_req(&req)?;
            if let Some(blacklisted) = IpBlacklist::get(ip.to_string()).await? {
                debug_assert_eq!(blacklisted.ip, ip.to_string());
                debug_assert!(blacklisted.exp >= Utc::now());

                let ts = blacklisted.exp.timestamp();
                return Err(Error::from(ErrorResponse::new(
                    ErrorResponseType::TooManyRequests(ts),
                    TooManyRequestsHtml::build(ip.to_string(), ts),
                )));
            }

            service.call(req).await
        })
    }
}
