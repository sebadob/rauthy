// Copyright 2025 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, HeaderMap, HeaderValue,
};
use actix_web::{HttpRequest, HttpResponse, web};
use rauthy_api_types::users::WebauthnLoginResponse;
use rauthy_common::constants::COOKIE_MFA;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::AuthStep;
use rauthy_models::api_cookie::ApiCookie;
use rauthy_models::entity::api_keys::ApiKey;
use rauthy_models::entity::fed_cm::FedCMLoginStatus;
use rauthy_models::entity::principal::Principal;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::webauthn::WebauthnCookie;
use rust_embed::RustEmbed;
use tracing::error;

pub mod api_keys;
pub mod auth_providers;
pub mod blacklist;
pub mod clients;
pub mod dev_only;
pub mod events;
pub mod fed_cm;
pub mod generic;
pub mod groups;
pub mod html;
pub mod oidc;
pub mod openapi;
pub mod roles;
pub mod scopes;
pub mod sessions;
pub mod themes;
pub mod users;

pub type ReqApiKey = web::ReqData<Option<ApiKey>>;
pub type ReqPrincipal = web::ReqData<Principal>;
pub type ReqSession = web::ReqData<Option<Session>>;

#[derive(Debug, PartialEq)]
enum AcceptEncoding {
    Br,
    Gzip,
    None,
}

impl From<&HeaderMap> for AcceptEncoding {
    #[inline]
    fn from(headers: &HeaderMap) -> Self {
        if let Some(accept) = headers
            .get("accept-encoding")
            .map(|v| v.to_str().unwrap_or_default())
        {
            if accept.contains("br") {
                Self::Br
            } else if accept.contains("gzip") {
                Self::Gzip
            } else {
                Self::None
            }
        } else {
            Self::None
        }
    }
}

impl AcceptEncoding {
    #[inline]
    pub fn value(&self) -> HeaderValue {
        match self {
            AcceptEncoding::Br => HeaderValue::from_static("br"),
            AcceptEncoding::Gzip => HeaderValue::from_static("gzip"),
            AcceptEncoding::None => HeaderValue::from_static("none"),
        }
    }
}

#[derive(RustEmbed)]
#[folder = "../../static/v1/"]
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
                resp.headers_mut().insert(
                    ACCESS_CONTROL_ALLOW_METHODS,
                    HeaderValue::from_static("POST"),
                );
                resp.headers_mut().insert(
                    ACCESS_CONTROL_ALLOW_CREDENTIALS,
                    HeaderValue::from_static("true"),
                );
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

/// Throws an error if the `content-length` exceeds the given size in MB.
fn content_len_limit(req: &HttpRequest, limit_mb: u16) -> Result<(), ErrorResponse> {
    match req.headers().get("Content-Length") {
        None => Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Content-Length header missing",
        )),
        Some(value) => match value.to_str().unwrap_or("0").parse::<u32>() {
            Ok(len) => {
                if len > limit_mb as u32 * 1024 * 1024 {
                    Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        format!("Max size {limit_mb}MB"),
                    ))
                } else {
                    Ok(())
                }
            }
            Err(err) => {
                error!("Error decoding Content-Length header {:?}: {}", value, err);
                Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "invalid Content-Length header",
                ))
            }
        },
    }
}
