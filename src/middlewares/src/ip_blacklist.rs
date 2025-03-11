use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use chrono::Utc;
use futures::future::LocalBoxFuture;
use rauthy_common::utils::real_ip_from_svc_req;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::events::ip_blacklist_handler::{IpBlacklistCheck, IpBlacklistReq};
use rauthy_models::html::templates::TooManyRequestsHtml;
use std::future::{Ready, ready};
use std::rc::Rc;
use tokio::sync::oneshot;
use tracing::error;

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
            let app_state = req
                .app_data::<web::Data<AppState>>()
                .expect("AppState to be in the Actix context");

            let ip = real_ip_from_svc_req(&req)?;
            // if let Some(ip) = real_ip_from_svc_req(&req) {
            let (tx, rx) = oneshot::channel();
            app_state
                .tx_ip_blacklist
                .send_async(IpBlacklistReq::BlacklistCheck(IpBlacklistCheck {
                    ip: ip.to_string(),
                    tx,
                }))
                .await
                .unwrap();
            match rx.await {
                Ok(exp) => {
                    if let Some(exp) = exp {
                        if exp > Utc::now() {
                            let ts = exp.timestamp();
                            return Err(Error::from(ErrorResponse::new(
                                ErrorResponseType::TooManyRequests(ts),
                                TooManyRequestsHtml::build(ip.to_string(), ts),
                            )));
                        }
                    }
                }
                Err(err) => {
                    error!(
                        "Checking IP Blacklist status in middleware - this should never happen: {:?}",
                        err
                    );
                }
            }

            service.call(req).await
        })
    }
}
