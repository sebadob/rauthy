// Copyright 2024 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

use actix_web::{web, HttpRequest, HttpResponse};
use rauthy_common::constants::COOKIE_MFA;
use rauthy_common::error_response::ErrorResponse;
use rauthy_models::api_cookie::ApiCookie;
use rauthy_models::entity::api_keys::ApiKey;
use rauthy_models::entity::fed_cm::FedCMLoginStatus;
use rauthy_models::entity::principal::Principal;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::webauthn::WebauthnCookie;
use rauthy_models::response::WebauthnLoginResponse;
use rauthy_models::AuthStep;
use rust_embed::RustEmbed;
use tracing::error;

pub mod api_keys;
pub mod auth_providers;
pub mod blacklist;
pub mod clients;
pub mod events;
pub mod fed_cm;
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
) -> Result<HttpResponse, ErrorResponse> {
    // we will only get here after a successful login -> always return logged-in header
    let fed_cm_header = FedCMLoginStatus::LoggedIn.as_header_pair();

    match auth_step {
        AuthStep::LoggedIn(res) => {
            let mut resp = HttpResponse::Accepted()
                .insert_header(fed_cm_header)
                .insert_header(res.header_loc)
                .insert_header(res.header_csrf)
                .finish();
            if let Some((name, value)) = res.header_origin {
                resp.headers_mut().insert(name, value);
            }
            Ok(resp)
        }

        AuthStep::AwaitWebauthn(res) => {
            let body = WebauthnLoginResponse {
                code: res.code,
                user_id: res.user_id,
                exp: res.exp,
            };
            let mut resp = HttpResponse::Ok()
                .insert_header(fed_cm_header)
                .insert_header(res.header_csrf)
                .json(&body);

            if let Some((name, value)) = res.header_origin {
                resp.headers_mut().insert(name, value);
            }

            // if there is no mfa_cookie present, set a new one
            if let Ok(mfa_cookie) =
                WebauthnCookie::parse_validate(&ApiCookie::from_req(req, COOKIE_MFA))
            {
                if mfa_cookie.email != res.email {
                    add_req_mfa_cookie(&mut resp, res.email.clone())?;
                }
            } else {
                add_req_mfa_cookie(&mut resp, res.email.clone())?;
            }

            Ok(resp)
        }

        AuthStep::ProviderLink => {
            // TODO generate a new event type in this case?
            Ok(HttpResponse::NoContent()
                .insert_header(fed_cm_header)
                .finish())
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
