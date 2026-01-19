// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

use actix_web::http::StatusCode;
use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, HeaderMap, HeaderValue,
};
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, web};
use rauthy_api_types::tos::ToSAwaitLoginResponse;
use rauthy_api_types::users::WebauthnLoginResponse;
use rauthy_common::constants::COOKIE_MFA;
use rauthy_data::AuthStep;
use rauthy_data::api_cookie::ApiCookie;
use rauthy_data::entity::api_keys::ApiKey;
use rauthy_data::entity::auth_providers::NewFederatedUserCreated;
use rauthy_data::entity::fed_cm::FedCMLoginStatus;
use rauthy_data::entity::principal::Principal;
use rauthy_data::entity::sessions::Session;
use rauthy_data::entity::webauthn::WebauthnCookie;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rust_embed::Embed;
use tracing::error;

pub mod api_keys;
pub mod atproto;
pub mod auth_providers;
pub mod backup;
pub mod blacklist;
pub mod clients;
pub mod cors_preflight;
pub mod dev_only;
pub mod email;
pub mod events;
pub mod fed_cm;
pub mod generic;
pub mod groups;
pub mod html;
pub mod oidc;
pub mod openapi;
pub mod pam;
pub mod roles;
pub mod scopes;
pub mod sessions;
pub mod swagger_ui;
pub mod themes;
pub mod tos;
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

#[derive(Embed)]
#[folder = "../../static/v1/"]
struct Assets;

pub async fn map_auth_step(
    auth_step: AuthStep,
    req: &HttpRequest,
    new_user_created: NewFederatedUserCreated,
    // the bool for Ok() is true is the password has been hashed
    // the bool for Err() means if we need to add a login delay (and none otherwise for better UX)
) -> Result<HttpResponse, ErrorResponse> {
    // we will only get here after a successful login -> always return logged-in header
    let fed_cm_header = FedCMLoginStatus::LoggedIn.as_header_pair();

    match auth_step {
        AuthStep::LoggedIn(res) => {
            let mut builder = HttpResponse::Accepted();
            builder
                .insert_header(fed_cm_header)
                .insert_header(res.header_loc)
                .insert_header(res.header_csrf);

            if let Some(origin) = res.header_origin {
                builder
                    .insert_header(origin)
                    .insert_header((
                        ACCESS_CONTROL_ALLOW_METHODS,
                        HeaderValue::from_static("POST"),
                    ))
                    .insert_header((
                        ACCESS_CONTROL_ALLOW_CREDENTIALS,
                        HeaderValue::from_static("true"),
                    ));
            }

            Ok(builder.finish())
        }

        AuthStep::AwaitToSAccept(res) => {
            // Partial Content as return code is technically probably not "correct", as you
            // would expect it during downloads, but it makes the checks in the UI a lot more
            // resilient, when we have a dedicated code for each possible situation we need to
            // handle.
            let mut builder = HttpResponseBuilder::new(StatusCode::from_u16(206).unwrap());
            builder
                .insert_header(fed_cm_header)
                .insert_header(res.header_csrf);

            if let Some(origin) = res.header_origin {
                builder
                    .insert_header(origin)
                    .insert_header((
                        ACCESS_CONTROL_ALLOW_METHODS,
                        HeaderValue::from_static("POST"),
                    ))
                    .insert_header((
                        ACCESS_CONTROL_ALLOW_CREDENTIALS,
                        HeaderValue::from_static("true"),
                    ));
            }

            Ok(builder.json(&ToSAwaitLoginResponse {
                tos_await_code: res.code,
                user_id: Some(res.user_id),
                force_accept: (new_user_created == NewFederatedUserCreated::Yes).then_some(true),
            }))
        }

        AuthStep::AwaitWebauthn(res) => {
            let mut builder = HttpResponse::Ok();
            builder
                .insert_header(fed_cm_header)
                .insert_header(res.header_csrf);

            if let Some(origin) = res.header_origin {
                builder.insert_header(origin);
            }

            // if there is no mfa_cookie present, set a new one
            if let Ok(mfa_cookie) =
                WebauthnCookie::parse_validate(&ApiCookie::from_req(req, COOKIE_MFA))
            {
                if mfa_cookie.email != res.email {
                    builder.cookie(WebauthnCookie::new(res.email.clone()).build()?);
                }
            } else {
                builder.cookie(WebauthnCookie::new(res.email.clone()).build()?);
            }

            Ok(builder.json(&WebauthnLoginResponse {
                code: res.code,
                user_id: res.user_id,
                exp: res.exp,
            }))
        }

        AuthStep::ProviderLink => Ok(HttpResponse::NoContent()
            .insert_header(fed_cm_header)
            .finish()),
    }
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
