use crate::build_lax_cookie_300;
use crate::cookie_state::{OidcCookieState, OIDC_STATE_COOKIE};
use crate::provider::OidcProvider;
use crate::token_set::{JwtAccessClaims, JwtIdClaims, OidcTokenSet};
use serde::{Deserialize, Serialize};
use tracing::error;

#[cfg(feature = "axum")]
pub mod axum;

#[cfg(feature = "actix-web")]
pub mod actix_web;

/// Query params appended to the callback URL after a successful login from the user
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OidcCallbackParams {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize)]
struct OidcCodeRequestParams {
    client_id: String,
    client_secret: Option<String>,
    code: String,
    code_verifier: String,
    grant_type: &'static str,
    redirect_uri: String,
}

/// Used instead of a normal bool to never confuse people about a meaning of multiple bool's
#[derive(Debug, PartialEq)]
pub enum OidcCookieInsecure {
    Yes,
    No,
}

/// Used instead of a normal bool to never confuse people about a meaning of multiple bool's
#[derive(Debug, PartialEq)]
pub enum OidcSetRedirectStatus {
    Yes,
    No,
}

impl OidcCodeRequestParams {
    #[allow(dead_code)]
    pub async fn try_new(
        code: String,
        code_verifier: String,
        redirect_uri: String,
    ) -> anyhow::Result<Self> {
        let cfg = OidcProvider::config()?;
        let client_id = cfg.client_id.clone();
        let client_secret = cfg.secret.clone();
        Ok(Self {
            client_id,
            client_secret,
            code,
            code_verifier,
            grant_type: "authorization_code",
            redirect_uri,
        })
    }
}

/// Check the authentication
///
/// This will only exist without `actix-web` or `axum` features
///
/// # Returns
/// - Ok(()) if the user is logged in
/// - Err(None) if the user is not logged in and the OIDC provider is not correctly set up
/// - Err(Some(String, String)) if the user is not logged in.
///   In this case, the values in the tuple are header values you should return to the client:
///   (LocationHeaderString, EncryptedStateCookieValue)
#[cfg(not(any(feature = "axum", feature = "actix-web")))]
pub async fn validate_principal_generic(
    principal: Option<crate::principal::PrincipalOidc>,
    enc_key: &[u8],
    insecure: OidcCookieInsecure,
) -> Result<(), Option<(String, String)>> {
    if principal.is_some() {
        Ok(())
    } else {
        let (cookie_state, challenge) = OidcCookieState::generate();
        let loc = {
            let base = match OidcProvider::config() {
                Ok(c) => &c.auth_url_base,
                Err(_) => {
                    return Err(None);
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

        Err(Some((loc, cookie)))
    }
}

/// Handles the OIDC callback
///
/// If you use `actix-web` or `axum` features, you should use the more specific implementations.
///
/// # Panics
/// If the given `enc_key` is not exactly 32 bytes long
#[allow(dead_code)]
pub async fn oidc_callback(
    cookie_state: OidcCookieState,
    params: OidcCallbackParams,
    insecure: OidcCookieInsecure,
) -> anyhow::Result<(String, OidcTokenSet, JwtIdClaims)> {
    // validate the state to prevent xsrf attacks
    if params.state != cookie_state.state {
        return Err(anyhow::Error::msg("Bad state"));
    }

    let (token_uri, redirect_uri) = {
        let cfg = OidcProvider::config()?;
        let t = cfg.provider.token_endpoint.clone();
        let r = cfg.redirect_uri.clone();
        (t, r)
    };
    let req_data = OidcCodeRequestParams::try_new(
        params.code.clone(),
        cookie_state.pkce_verifier,
        redirect_uri,
    )
    .await?;

    let res = OidcProvider::client()
        .post(&token_uri)
        .form(&req_data)
        .send()
        .await?;
    if res.status().as_u16() >= 300 {
        error!("{:?}", res);
        let body = res.text().await;
        let msg = match body {
            Ok(value) => {
                error!("raw OIDC provider response: {:?}", value);
                value
            }
            Err(_) => "Internal Error - Bad response status".to_string(),
        };

        Err(anyhow::Error::msg(msg))
    } else {
        match res.json::<OidcTokenSet>().await {
            Ok(ts) => {
                // validate access token
                let _access_claims =
                    JwtAccessClaims::from_token_validated(&ts.access_token).await?;

                // validate id token
                if ts.id_token.is_none() {
                    return Err(anyhow::Error::msg("ID token is missing"));
                }
                let id_claims = JwtIdClaims::from_token_validated(
                    ts.id_token.as_deref().unwrap(),
                    &cookie_state.nonce,
                )
                .await?;

                // reset STATE_COOKIE
                let cookie = build_lax_cookie_300(
                    OIDC_STATE_COOKIE,
                    "",
                    insecure == OidcCookieInsecure::Yes,
                );

                Ok((cookie, ts, id_claims))
            }
            Err(err) => {
                error!("Deserializing OIDC response to OidcTokenSet: {}", err);
                Err(anyhow::Error::msg(
                    "Internal Error - Deserializing OIDC response",
                ))
            }
        }
    }
}
