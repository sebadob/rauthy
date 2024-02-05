// Copyright 2024 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

use actix_web::dev::ServiceRequest;
use actix_web::{web, HttpRequest, HttpResponse};
use rauthy_common::constants::{COOKIE_MFA, PROXY_MODE};
use rauthy_common::error_response::ErrorResponse;
use rauthy_models::entity::api_keys::ApiKey;
use rauthy_models::entity::principal::Principal;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::webauthn::WebauthnCookie;
use rauthy_models::response::WebauthnLoginResponse;
use rauthy_models::AuthStep;
use rust_embed::RustEmbed;
use tracing::error;

pub mod api_keys;
pub mod blacklist;
pub mod clients;
pub mod events;
pub mod generic;
pub mod groups;
pub mod middleware;
pub mod oidc;
pub mod openapi;
pub mod roles;
pub mod scopes;
pub mod sessions;
pub mod users;

pub type ReqApiKey = web::ReqData<Option<ApiKey>>;
pub type ReqPrincipal = web::ReqData<Principal>;
pub type ReqSession = web::ReqData<Option<Session>>;

#[derive(RustEmbed)]
#[folder = "../static/v1/"]
struct Assets;

pub async fn map_auth_step(
    auth_step: AuthStep,
    req: &HttpRequest,
    // the bool for Ok() is true is the password has been hashed
    // the bool for Err() means if we need to add a login delay (and none otherwise for better UX)
) -> Result<(HttpResponse, bool), (ErrorResponse, bool)> {
    match auth_step {
        AuthStep::LoggedIn(res) => {
            let mut resp = HttpResponse::Accepted()
                .insert_header(res.header_loc)
                .insert_header(res.header_csrf)
                .finish();
            if let Some((name, value)) = res.header_origin {
                resp.headers_mut().insert(name, value);
            }
            Ok((resp, res.has_password_been_hashed))
        }

        AuthStep::AwaitWebauthn(res) => {
            let body = WebauthnLoginResponse {
                code: res.code,
                user_id: res.user_id,
                exp: res.exp,
            };
            let mut resp = HttpResponse::Ok()
                .insert_header(res.header_csrf)
                .json(&body);

            if let Some((name, value)) = res.header_origin {
                resp.headers_mut().insert(name, value);
            }

            // if there is no mfa_cookie present, set a new one
            if let Ok(mfa_cookie) = WebauthnCookie::parse_validate(&req.cookie(COOKIE_MFA)) {
                if mfa_cookie.email != res.email {
                    add_req_mfa_cookie(&mut resp, res.email.clone()).map_err(|err| (err, true))?;
                }
            } else {
                add_req_mfa_cookie(&mut resp, res.email.clone()).map_err(|err| (err, true))?;
            }

            Ok((resp, res.has_password_been_hashed))
        }
    }
}

#[inline]
fn add_req_mfa_cookie(resp: &mut HttpResponse, email: String) -> Result<(), ErrorResponse> {
    let binding = WebauthnCookie::new(email);
    let cookie = binding.build()?;

    if let Err(err) = resp.add_cookie(&cookie) {
        error!("Error adding mfa cookie in 'map_auth_step' : {}", err);
    }

    Ok(())
}

pub fn real_ip_from_req(req: &HttpRequest) -> Option<String> {
    if *PROXY_MODE {
        // TODO maybe make this configurable and extract headers in user-configured order?
        req.connection_info()
            .realip_remote_addr()
            .map(|ip| ip.to_string())
    } else {
        req.connection_info().peer_addr().map(|ip| ip.to_string())
    }
}

pub fn real_ip_from_svc_req(req: &ServiceRequest) -> Option<String> {
    if *PROXY_MODE {
        // TODO maybe make this configurable and extract headers in user-configured order?
        req.connection_info()
            .realip_remote_addr()
            .map(|ip| ip.to_string())
    } else {
        req.connection_info().peer_addr().map(|ip| ip.to_string())
    }
}
