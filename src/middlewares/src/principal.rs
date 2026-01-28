use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    http,
};
use chrono::Utc;
use futures::future::LocalBoxFuture;
use rauthy_common::constants::{COOKIE_SESSION, TOKEN_API_KEY};
use rauthy_common::utils::real_ip_from_svc_req;
use rauthy_data::api_cookie::ApiCookie;
use rauthy_data::entity::api_keys::{ApiKey, ApiKeyEntity};
use rauthy_data::entity::principal::Principal;
use rauthy_data::entity::sessions::Session;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::future::{Ready, ready};
use std::rc::Rc;
use tracing::{debug, trace};

pub struct RauthyPrincipalMiddleware;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for RauthyPrincipalMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = PrincipalMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PrincipalMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct PrincipalMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for PrincipalMiddleware<S>
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
            if req.path().starts_with("/auth/v1/_app/") || req.path() == "/auth/v1/" {
                return service.call(req).await;
            }

            let mut principal = Principal {
                session: None,
                api_key: get_api_key_from_headers(&req).await?,
                roles: vec![],
            };

            if let Some(s) = get_session_from_cookie(&req).await? {
                principal.roles = s.roles_as_vec().unwrap_or_default();
                principal.session = Some(s);
            }

            req.extensions_mut().insert(principal);

            service.call(req).await
        })
    }
}

#[inline(always)]
async fn get_api_key_from_headers(req: &ServiceRequest) -> Result<Option<ApiKey>, ErrorResponse> {
    let headers = req.headers();
    let auth_header = if let Some(Ok(header)) = headers.get("Authorization").map(|h| h.to_str()) {
        header
    } else {
        return Ok(None);
    };
    let (k, v) = if let Some(res) = auth_header.split_once(' ') {
        res
    } else {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Malformed 'Authorization' header",
        ));
    };
    let api_key_value = if k.ne(TOKEN_API_KEY) || k.is_empty() {
        None
    } else {
        Some(v)
    };

    if let Some(api_key_value) = api_key_value {
        ApiKeyEntity::api_key_from_token_validated(api_key_value)
            .await
            .map(Some)
    } else {
        Ok(None)
    }
}

#[inline(always)]
async fn get_session_from_cookie(req: &ServiceRequest) -> Result<Option<Session>, ErrorResponse> {
    let session_id = match ApiCookie::from_svc_req(req, COOKIE_SESSION) {
        None => {
            return Ok(None);
        }
        Some(session_id) => session_id,
    };

    match Session::find(session_id).await {
        Ok(mut session) => {
            let remote_ip = if RauthyConfig::get().vars.access.session_validate_ip {
                real_ip_from_svc_req(req).ok()
            } else {
                None
            };

            if session.is_valid(
                RauthyConfig::get().vars.lifetimes.session_timeout,
                remote_ip,
                req.path(),
            ) {
                let now = Utc::now().timestamp();
                // only update the last_seen, if it is older than 10 seconds
                if session.last_seen < now - 10 {
                    session.last_seen = now;
                    session.upsert().await?;
                }

                if req.method() == http::Method::GET
                    // || req.method() == http::Method::HEAD
                    || is_path_csrf_exception(req.path())
                    || session.validate_csrf(req.request()).is_ok()
                {
                    Ok(Some(session))
                } else {
                    Ok(None)
                }
            } else {
                trace!("Access to {} with invalid Session", req.path());
                Ok(None)
            }
        }
        Err(err) => {
            debug!(?err, "ERROR: Could not lookup the session from the cookie",);
            Ok(None)
        }
    }
}

// !!! CAUTION !!!
// CSRF MUST BE CHECKED FOR THESE EXCEPTIONS MANUALLY !
#[inline(always)]
fn is_path_csrf_exception(path: &str) -> bool {
    path.ends_with("/webauthn/register/start") || path.ends_with("/webauthn/register/finish")
}
