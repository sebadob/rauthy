use actix_web::http::Method;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::LocalBoxFuture;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use std::future::{ready, Ready};
use std::rc::Rc;

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
                let site = site.to_str().unwrap_or_default();

                // same origin is always allowed
                if site == "same-origin" {
                    return service.call(req).await;
                }

                if req.method() == &Method::GET {
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

                    if mode == "navigate" && !["object", "embed"].contains(&dest) {
                        return service.call(req).await;
                    }
                }

                // check exceptions last because it is the most expensive
                if is_path_csrf_exception(req.path()) {
                    return service.call(req).await;
                }

                Err(Error::from(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "cross-origin request forbidden for this resource".to_string(),
                )))
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
        None => true,
        Some(path) => {
            path == "/oidc/authorize"
                || path == "/oidc/logout"
                // has its own custom validation
                || path == "/oidc/token"
                || path == "/oidc/userinfo"
                // no issue with CORS
                || path == "/users/register"
                || path == "/pow"
                // make it possible to fetch public keys from browsers / SPAs
                || path.starts_with("/oidc/certs")
                || path.ends_with("/logo")
                || path.starts_with("/.well-known/")
                || path.starts_with("/auth/v1/.well-known/")
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
        assert!(is_path_csrf_exception("/"));
        assert!(is_path_csrf_exception("/value123"));
        assert!(is_path_csrf_exception("/auth/v1/oidc/authorize"));
        assert!(is_path_csrf_exception("/auth/v1/oidc/token"));
        assert!(is_path_csrf_exception("/auth/v1/oidc/certs"));
        assert!(is_path_csrf_exception("/auth/v1/oidc/certs/123"));
        assert!(is_path_csrf_exception(
            "what_ever_in_front/authauthn/auth/start"
        ));
        assert!(is_path_csrf_exception(
            "what_ever_in_front/authauthn/auth/finish"
        ));
        assert!(is_path_csrf_exception("/.well-known/openid-configuration"));
        assert!(is_path_csrf_exception(
            "/auth/v1/.well-known/openid-configuration"
        ));

        // denied
        assert!(!is_path_csrf_exception("/auth/v1/oidc/authorize/refresh"));
        assert!(!is_path_csrf_exception("/auth/v1/users"));
        assert!(!is_path_csrf_exception("/auth/v1/users/id123"));
    }
}
