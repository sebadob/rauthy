use actix_web::http::Method;
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::LocalBoxFuture;
use rauthy_common::utils::real_ip_from_svc_req;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::future::{Ready, ready};
use std::rc::Rc;
use tracing::{debug, warn};

pub struct CsrfProtectionMiddleware;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for CsrfProtectionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CsrfMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CsrfMiddlewareInner {
            service: Rc::new(service),
        }))
    }
}

pub struct CsrfMiddlewareInner<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CsrfMiddlewareInner<S>
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
            if let Some(site) = req.headers().get("sec-fetch-site") {
                if req.method() == Method::OPTIONS {
                    return service.call(req).await;
                }

                let site = site.to_str().unwrap_or_default();

                // same origin is always allowed
                if site == "same-origin" {
                    return service.call(req).await;
                }

                if req.method() == Method::GET {
                    // user interactions will be 'none'
                    if site == "none" {
                        return service.call(req).await;
                    }

                    // allow links and redirects from external sites
                    let dest = req
                        .headers()
                        .get("sec-fetch-dest")
                        .map(|h| h.to_str().unwrap_or_default())
                        .unwrap_or_default();
                    let mode = req
                        .headers()
                        .get("sec-fetch-mode")
                        .map(|h| h.to_str().unwrap_or_default())
                        .unwrap_or_default();

                    debug!("sec-fetch-dest: {dest}, sec-fetch-mode: {mode}");

                    // allow images fetches like favicon
                    if dest == "image" && mode == "no-cors" {
                        return service.call(req).await;
                    }

                    // allow navigation to this site but no embedding
                    if mode == "navigate" && !["embed", "iframe", "object"].contains(&dest) {
                        return service.call(req).await;
                    }
                }

                // check exceptions last because it is the most expensive
                if is_path_csrf_exception(req.path()) {
                    return service.call(req).await;
                }

                let ip = real_ip_from_svc_req(&req)?;
                warn!(
                    "CSRF / Sec-Header violation from {ip} on path {}",
                    req.path()
                );
                if RauthyConfig::get().vars.access.sec_header_block {
                    Err(Error::from(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "cross-origin request forbidden for this resource",
                    )))
                } else {
                    warn!(
                        "SEC_HEADER_BLOCK is set to false - not blocking violation requests.\
                        This will not be possible in future releases."
                    );
                    service.call(req).await
                }
            } else {
                // allow requests that do not contain the header
                service.call(req).await
            }
        })
    }
}

#[inline(always)]
fn is_path_csrf_exception(path: &str) -> bool {
    match path.strip_prefix("/auth/v1") {
        None => path.starts_with("/.well-known/"),
        Some(path) => {
            // have their own custom validation
            path == "/oidc/authorize"
                || path == "/oidc/token"
                || path == "/oidc/logout"
                // no issue with CORS
                || path == "/clients_dyn"
                || path == "/oidc/userinfo"
                || path == "/users/register"
                || path == "/pow"
                // make it possible to fetch public keys from browsers / SPAs
                || path.starts_with("/oidc/certs")
                || path == "/atproto/client_metadata"
                // FedCM has its own validation mechanisms
                || path.starts_with("/fed_cm/")
                // Client Logos
                || path.ends_with("/logo")
                || path.starts_with("/.well-known/")
                // Webauthn has additional validation via Origin internally
                || path.contains("/webauthn/auth/")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_path_csrf_exception() {
        // allowed
        assert!(is_path_csrf_exception("/auth/v1/oidc/authorize"));
        assert!(is_path_csrf_exception("/auth/v1/oidc/token"));
        assert!(is_path_csrf_exception("/auth/v1/oidc/certs"));
        assert!(is_path_csrf_exception("/auth/v1/oidc/certs/123"));
        assert!(is_path_csrf_exception("/.well-known/openid-configuration"));
        assert!(is_path_csrf_exception(
            "/auth/v1/.well-known/openid-configuration"
        ));

        // denied
        assert!(!is_path_csrf_exception("/auth/v1/oidc/authorize/refresh"));
        assert!(!is_path_csrf_exception("/auth/v1/users"));
        assert!(!is_path_csrf_exception("/auth/v1/users/id123"));
        assert!(!is_path_csrf_exception("/docs/v1/whatever"));
        assert!(!is_path_csrf_exception("/"));
        assert!(!is_path_csrf_exception("/value123"));
        assert!(!is_path_csrf_exception(
            "what_ever_in_front/webauthn/auth/start"
        ));
        assert!(!is_path_csrf_exception(
            "what_ever_in_front/webauthn/auth/finish"
        ));
    }
}
