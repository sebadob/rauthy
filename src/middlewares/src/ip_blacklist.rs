use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use chrono::Utc;
use futures::future::LocalBoxFuture;
use rauthy_common::utils::real_ip_from_svc_req;
use rauthy_data::entity::ip_blacklist::IpBlacklist;
use rauthy_data::html::templates::{BlockedHtml, TooManyRequestsHtml};
use rauthy_data::ipgeo;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::future::{Ready, ready};
use std::rc::Rc;
use tracing::info;

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
            let config = &RauthyConfig::get().vars.geo;
            let ip = real_ip_from_svc_req(&req)?;

            // If the `block_is_whitelist` is `None`, Geoblocking is not configured at all
            if let Some(is_whitelist) = config.block_is_whitelist {
                // TODO does it make sense to also blacklist such an IP for faster lookups in case
                //  of following, concurrent requests?
                //  - measure time taken for the lookup vs memory overhead
                //  - maybe caching a looked up IP is the better approach with less overhead
                if let Some(iso_code) = ipgeo::get_location_alpha2(req.headers(), ip)? {
                    if is_whitelist && !config.country_list.contains(&iso_code) {
                        info!("Denying access from non-whitelisted country '{iso_code}'");
                        blocked_err()?;
                    } else if !is_whitelist && config.country_list.contains(&iso_code) {
                        info!("Denying access from blacklisted country '{iso_code}'");
                        blocked_err()?;
                    }
                } else if config.block_unknown {
                    info!("Denying access from unknown country");
                    blocked_err()?;
                }
            }

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

#[inline]
fn blocked_err() -> Result<(), Error> {
    Err(Error::from(ErrorResponse::new(
        ErrorResponseType::Blocked,
        BlockedHtml::build(),
    )))
}
