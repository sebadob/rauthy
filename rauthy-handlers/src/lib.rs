// Rauthy - OpenID Connect and Single Sign-On IdP
// Copyright (C) 2023 Sebastian Dobe <sebastiandobe@mailbox.org>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use actix_web::{web, HttpRequest, HttpResponse};
use rauthy_common::constants::COOKIE_MFA;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::webauthn::WebauthnCookie;
use rauthy_models::response::WebauthnLoginResponse;
use rauthy_models::AuthStep;
use tracing::error;

pub mod clients;
pub mod generic;
pub mod groups;
pub mod middleware;
pub mod oidc;
pub mod openapi;
pub mod roles;
pub mod scopes;
pub mod sessions;
pub mod users;

// TODO provide app state to create encrypted MFA cookies
pub fn map_auth_step(
    data: &web::Data<AppState>,
    auth_step: AuthStep,
    req: &HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    match auth_step {
        AuthStep::LoggedIn(res) => {
            let mut resp = HttpResponse::Accepted()
                .insert_header(res.header_loc)
                .insert_header(res.header_csrf)
                .finish();
            if let Some((name, value)) = res.header_origin {
                resp.headers_mut().insert(name, value);
            }
            Ok(resp)
        }

        // TODO clean up after websocket mfa app has been removed
        AuthStep::AwaitMfa(res) => Ok(HttpResponse::Ok()
            .insert_header(res.header_csrf)
            .json(res.await_mfa_response)),

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
            if let Ok(mfa_cookie) =
                WebauthnCookie::parse_validate(&req.cookie(COOKIE_MFA), &data.enc_keys)
            {
                if mfa_cookie.email != res.email {
                    add_req_mfa_cookie(data, &mut resp, res.email.clone())?;
                }
            } else {
                add_req_mfa_cookie(data, &mut resp, res.email.clone())?;
            }

            Ok(resp)
        }
    }
}

#[inline]
fn add_req_mfa_cookie(
    data: &web::Data<AppState>,
    resp: &mut HttpResponse,
    email: String,
) -> Result<(), ErrorResponse> {
    let mfa_cookie = WebauthnCookie::new(email);
    let secret = data.enc_keys.get(&data.enc_key_active).ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            "Internal Error with the ENC_KEYS config".to_string(),
        )
    })?;
    let cookie = mfa_cookie.build(&data.enc_key_active, secret.as_ref())?;

    if let Err(err) = resp.add_cookie(&cookie) {
        error!("Error adding mfa cookie in 'map_auth_step' : {}", err);
    }

    Ok(())
}

pub fn build_csp_header(nonce: &str) -> (&str, String) {
    // Note: The unsafe-inline for the style-src currently has an open issue on the svelte repo.
    // As soon as this is fixed, we can get rid of it.
    // let value = format!(
    //     "default-src 'self'; script-src 'self' 'nonce-{}'; style-src 'self' 'nonce-{}'; \
    //     frame-ancestors 'self'; object-src 'none'; img-src 'self' data:;",
    //     nonce, nonce
    // );
    let value = format!(
        "default-src 'self'; script-src 'self' 'nonce-{}'; style-src 'self' 'unsafe-inline'; \
        frame-ancestors 'self'; object-src 'none'; img-src 'self' data:;",
        nonce,
    );
    ("content-security-policy", value)
}
