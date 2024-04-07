use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http, web, Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use rauthy_common::constants::{COOKIE_SESSION, SESSION_VALIDATE_IP, TOKEN_API_KEY};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::real_ip_from_svc_req;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::{ApiKey, ApiKeyEntity};
use rauthy_models::entity::principal::Principal;
use rauthy_models::entity::sessions::Session;
use std::future::{ready, Ready};
use std::rc::Rc;
use time::OffsetDateTime;
use tracing::debug;

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
            // let mut session = None;
            let mut principal = Principal::default();

            let data = req
                .app_data::<web::Data<AppState>>()
                .expect("Error getting AppData inside session middleware");

            principal.api_key = get_api_key_from_headers(&req, data).await?;
            if let Some(s) = get_session_from_cookie(&req, data).await? {
                principal.roles = s.roles_as_vec().unwrap_or_default();
                principal.session = Some(s);
            }

            req.extensions_mut().insert(principal);

            service.call(req).await
        })
    }
}

#[inline(always)]
async fn get_api_key_from_headers(
    req: &ServiceRequest,
    data: &web::Data<AppState>,
) -> Result<Option<ApiKey>, ErrorResponse> {
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
            "Malformed 'Authorization' header".to_string(),
        ));
    };
    let api_key_value = if k.ne(TOKEN_API_KEY) || k.is_empty() {
        None
    } else {
        Some(v)
    };

    if let Some(api_key_value) = api_key_value {
        ApiKeyEntity::api_key_from_token_validated(data, api_key_value)
            .await
            .map(Some)
    } else {
        Ok(None)
    }
}

#[inline(always)]
async fn get_session_from_cookie(
    req: &ServiceRequest,
    data: &web::Data<AppState>,
) -> Result<Option<Session>, ErrorResponse> {
    let session_id = match req.cookie(COOKIE_SESSION) {
        None => {
            return Ok(None);
        }
        Some(cookie) => cookie.value().to_string(),
    };

    match Session::find(data, session_id).await {
        Ok(mut session) => {
            let remote_ip = if *SESSION_VALIDATE_IP {
                real_ip_from_svc_req(req)
            } else {
                None
            };
            if session.is_valid(data.session_timeout, remote_ip) {
                let now = OffsetDateTime::now_utc().unix_timestamp();
                // only update the last_seen, if it is older than 10 seconds
                if session.last_seen < now - 10 {
                    session.last_seen = now;
                    session.save(data).await?;
                }

                if req.method() != http::Method::GET && !is_path_csrf_exception(req.path()) {
                    // any request other than GET needs to validate the CSRF token in the header
                    if session.validate_csrf(req.request()).is_ok() {
                        Ok(Some(session))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(Some(session))
                }
            } else {
                Ok(None)
            }
        }
        Err(err) => {
            debug!(
                "ERROR: Could not lookup the session from the cookie: {:?}",
                err
            );
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
