use crate::cookie_state::{OidcCookieState, OIDC_STATE_COOKIE};
use crate::principal::PrincipalOidc;
use crate::provider::OidcProvider;
use crate::token_set::{JwtAccessClaims, JwtIdClaims, OidcTokenSet};
use axum::body::Body;
use axum::extract::Query;
use axum::http::{header, StatusCode};
use axum::response::Response;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use tracing::{error, warn};

#[derive(Debug, Deserialize)]
pub struct OidcCallbackParams {
    code: String,
    state: String,
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

// Used instead of a normal bool to never confuse people about a meaning of multiple bool's
#[derive(Debug, PartialEq)]
pub enum OidcCookieInsecure {
    Yes,
    No,
}

// Used instead of a normal bool to never confuse people about a meaning of multiple bool's
#[derive(Debug, PartialEq)]
pub enum OidcSetRedirectStatus {
    Yes,
    No,
}

impl OidcCodeRequestParams {
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

        let value = cookie_state.to_cookie_value(enc_key);
        let mut builder = Cookie::build(OIDC_STATE_COOKIE, value)
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Lax)
            .max_age(time::Duration::seconds(300));
        builder = if insecure == OidcCookieInsecure::Yes {
            warn!("Building an INSECURE cookie - DO NOT USE IN PRODUCTION");
            builder.secure(false)
        } else {
            builder.secure(true)
        };
        let cookie = builder.finish().to_string();

        let code = if set_redirect_status == OidcSetRedirectStatus::Yes {
            302
        } else {
            200
        };
        Response::builder()
            .status(code)
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
    let cookie_state = OidcCookieState::from_cookie_value(jar, enc_key)?;
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
                let mut builder = Cookie::build(OIDC_STATE_COOKIE, "")
                    .path("/")
                    .http_only(true)
                    .same_site(SameSite::Lax)
                    .max_age(::time::Duration::seconds(1));
                builder = if insecure == OidcCookieInsecure::Yes {
                    warn!("Building an INSECURE cookie - DO NOT USE IN PRODUCTION");
                    builder.secure(false)
                } else {
                    builder.secure(true)
                };
                let cookie = builder.finish().to_string();

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
//
// #[cfg(test)]
// mod tests {
//     const KEY: &str = "bQMr84TTHCz2YGt57rjgBNu5PLs8A8fz";
//
//     fn setup_logging() {
//         dotenv::dotenv().ok();
//         let subscriber = tracing_subscriber::FmtSubscriber::builder()
//             .with_max_level(tracing::Level::DEBUG)
//             .finish();
//         tracing::subscriber::set_global_default(subscriber)
//             .expect("setting default subscriber failed");
//     }
//
//     #[tokio::test]
//     async fn test_validate_redirect_principal() -> anyhow::Result<()> {
//         setup_logging();
//
//         let redirect_uri = "http://localhost:8080/oidc/callback".to_string();
//         let config = OidcConfig::from_(redirect_uri).await?;
//         let _tx = init(config, 10).await?;
//
//         let resp = validate_redirect_principal(None, KEY.as_bytes(), false, true).await;
//         assert_eq!(resp.status().as_u16(), 302);
//
//         let headers = resp.headers();
//         let loc = headers.get(header::LOCATION).unwrap();
//         assert!(!loc.is_empty());
//         debug!("{:?}", loc);
//         debug!("{}", loc.to_str().unwrap());
//         assert!(loc
//             .to_str()
//             .unwrap()
//             .starts_with("https://auth.meteo.netitservices.com/auth/v1"));
//
//         Ok(())
//     }
// }
