use crate::build_lax_cookie_300;
use crate::cookie_state::{OidcCookieState, OIDC_STATE_COOKIE};
use crate::handler::{OidcCallbackParams, OidcCookieInsecure, OidcSetRedirectStatus};
use crate::principal::PrincipalOidc;
use crate::provider::OidcProvider;
use crate::token_set::{JwtIdClaims, OidcTokenSet};
use axum::{
    body::Body,
    extract::Query,
    http::{header, StatusCode},
    response::Response,
};
use axum_extra::extract::CookieJar;

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
) -> Response<Body> {
    if principal.is_some() {
        Response::builder()
            .status(StatusCode::ACCEPTED)
            .body(Body::empty())
            .unwrap()
    } else {
        let (cookie_state, challenge) = OidcCookieState::generate();
        let loc = {
            let base = match OidcProvider::config() {
                Ok(c) => &c.auth_url_base,
                Err(_) => {
                    return Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("OIDC Provider has not been set up"))
                        .unwrap();
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
            Response::builder().status(302)
        } else {
            Response::builder().status(200)
        }
        .header(header::LOCATION, loc)
        .header(header::SET_COOKIE, cookie)
        .body(Body::empty())
        .unwrap()
    }
}

/// Handles the OIDC callback
///
/// # Panics
/// If the given `enc_key` is not exactly 32 bytes long
pub async fn oidc_callback(
    jar: &CookieJar,
    params: Query<OidcCallbackParams>,
    enc_key: &[u8],
    insecure: OidcCookieInsecure,
) -> anyhow::Result<(String, OidcTokenSet, JwtIdClaims)> {
    let cookie_state = OidcCookieState::from_jar_cookie_value(jar, enc_key)?;
    crate::handler::oidc_callback(cookie_state, params.0, insecure).await
}
