use crate::build_lax_cookie_300;
use crate::cookie_state::{OidcCookieState, OIDC_STATE_COOKIE};
use crate::handler::{OidcCallbackParams, OidcCookieInsecure, OidcSetRedirectStatus};
use crate::principal::PrincipalOidc;
use crate::provider::OidcProvider;
use crate::token_set::{JwtIdClaims, OidcTokenSet};
use actix_web::{
    http::header::{LOCATION, SET_COOKIE},
    web, HttpRequest, HttpResponse, Responder,
};

/// Check the authentication
///
/// Extracts the `Bearer` token from the `Authorization` header. It redirects to the OIDC login if
/// the token is not valid.
///
/// # Returns
/// HTTP 202 if the user is logged in
/// HTTP 200 with a Location header for the login endpoint if set_redirect_status is set to No
/// HTTP 302 with a Location header for the login endpoint if set_redirect_status is set to Yes
///
/// # Panics
/// If the given `enc_key` is not exactly 32 bytes long
pub async fn validate_redirect_principal(
    principal: Option<PrincipalOidc>,
    enc_key: &[u8],
    insecure: OidcCookieInsecure,
    set_redirect_status: OidcSetRedirectStatus,
) -> impl Responder {
    if principal.is_some() {
        HttpResponse::Accepted().finish()
    } else {
        let (cookie_state, challenge) = OidcCookieState::generate();
        let loc = {
            let base = match OidcProvider::config() {
                Ok(c) => &c.auth_url_base,
                Err(_) => {
                    return HttpResponse::InternalServerError()
                        .body("OIDC Provider has not been set up")
                }
            };
            format!(
                "{base}&code_challenge={challenge}&nonce={}&state={}",
                cookie_state.nonce, cookie_state.state
            )
        };

        let value = cookie_state.to_encrypted_cookie_value(enc_key);
        let cookie = build_lax_cookie_300(
            OIDC_STATE_COOKIE,
            &value,
            insecure == OidcCookieInsecure::Yes,
        );

        if set_redirect_status == OidcSetRedirectStatus::Yes {
            HttpResponse::TemporaryRedirect()
        } else {
            HttpResponse::Ok()
        }
        .append_header((LOCATION, loc))
        .append_header((SET_COOKIE, cookie))
        .finish()
    }
}

/// Handles the OIDC callback for actix-web
///
/// # Panics
/// If the given `enc_key` is not exactly 32 bytes long
pub async fn oidc_callback(
    req: &HttpRequest,
    params: web::Query<OidcCallbackParams>,
    enc_key: &[u8],
    insecure: OidcCookieInsecure,
) -> anyhow::Result<(String, OidcTokenSet, JwtIdClaims)> {
    let cookie_state = OidcCookieState::from_req_cookie_value(req, enc_key)?;
    crate::handler::oidc_callback(cookie_state, params.into_inner(), insecure).await
}
