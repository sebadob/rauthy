use crate::real_ip_from_svc_req;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use rauthy_common::constants::{COOKIE_SESSION, SESSION_VALIDATE_IP};
use rauthy_common::error_response::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::sessions::Session;
use std::future::{ready, Ready};
use std::rc::Rc;
use time::OffsetDateTime;
use tracing::debug;

pub struct RauthySessionMiddleware;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for RauthySessionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SessionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SessionMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct SessionMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for SessionMiddleware<S>
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
            let session = get_session_from_cookie(&req).await?;
            if let Some(s) = session {
                req.extensions_mut().insert(Some(s));
            } else {
                let n: Option<Session> = None;
                req.extensions_mut().insert(n);
            }

            service.call(req).await
        })
    }
}

async fn get_session_from_cookie(req: &ServiceRequest) -> Result<Option<Session>, ErrorResponse> {
    let cookie = req.cookie(COOKIE_SESSION);
    if cookie.is_none() {
        return Ok(None);
    }
    let cookie = cookie.unwrap();

    let session_id = cookie.value();
    let data = req
        .app_data::<web::Data<AppState>>()
        .expect("Error getting AppData inside session middleware");

    match Session::find(data, session_id.to_string()).await {
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

                Ok(Some(session))
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
