use crate::config::Config;
use crate::{templates, DEV_MODE};
use axum::body::Body;
use axum::extract::Query;
use axum::http::header::{CONTENT_TYPE, SET_COOKIE};
use axum::response::{IntoResponse, Response};
use rauthy_client::handler::{OidcCallbackParams, OidcCookieInsecure, OidcSetRedirectStatus};
use rauthy_client::principal::PrincipalOidc;
use rauthy_client::rauthy_error::RauthyError;
use std::sync::Arc;

type ConfigExt = axum::extract::State<Arc<Config>>;

/// Index HTML
pub async fn get_index() -> Response<Body> {
    Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "text/html")
        .body(Body::from(templates::HTML_INDEX))
        .unwrap()
}

/// OIDC Auth check and login
///
/// Endpoint with no redirect on purpose to use the result inside Javascript from the frontend.
/// HTTP 200 will have a location header and a manual redirect must be done
/// HTTP 202 means logged in Principal
pub async fn get_auth_check(config: ConfigExt, principal: Option<PrincipalOidc>) -> Response<Body> {
    let enc_key = config.enc_key.as_slice();

    // if we are in dev mode, we allow insecure cookies
    let insecure = if DEV_MODE {
        OidcCookieInsecure::Yes
    } else {
        OidcCookieInsecure::No
    };

    rauthy_client::handler::axum::validate_redirect_principal(
        principal,
        // this enc_key must be exactly 32 bytes long
        enc_key,
        insecure,
        // if you want the browser to automatically redirect to the login, set to yes
        // we set this to no to actually show a button for logging in beforehand
        OidcSetRedirectStatus::No,
    )
    .await
}

/// OIDC Callback endpoint - must match the `redirect_uri` for the login flow
pub async fn get_callback(
    jar: axum_extra::extract::CookieJar,
    config: ConfigExt,
    params: Query<OidcCallbackParams>,
) -> Response<Body> {
    let enc_key = config.enc_key.as_slice();

    // The `DEV_MODE` again here to just have a nicer DX when developing -> we allow insecure cookies
    let insecure = if DEV_MODE {
        OidcCookieInsecure::Yes
    } else {
        OidcCookieInsecure::No
    };
    let callback_res =
        rauthy_client::handler::axum::oidc_callback(&jar, params, enc_key, insecure).await;
    let (cookie_str, token_set, _id_claims) = match callback_res {
        Ok(res) => res,
        Err(err) => {
            return Response::builder()
                .status(400)
                .body(Body::from(format!("Invalid OIDC Callback: {}", err)))
                .unwrap()
        }
    };

    // At this point, the redirect was valid and everything was fine.
    // Depending on how you like to proceed, you could create an independant session for the user,
    // or maybe create just another factor of authentication like a CSRF token.
    // Otherwise, you could just go on and using the existing access token for further authentication.
    //
    // For the sake of this example, we will return the raw access token to the user via the HTML
    // so we can use it for future authentication from the frontend, but this is really up to you
    // and the security needs of your application.

    // This is a very naive approach to HTML templating and only for simplicity in this example.
    // Please don't do this in production and use a proper templating engine.
    let body = templates::HTML_CALLBACK
        .replace("{{ TOKEN }}", &token_set.access_token)
        .replace("{{ URI }}", "/");

    Response::builder()
        .status(200)
        // we should append the returned cookie jar here to
        // delete the state cookie from the login flow
        .header(SET_COOKIE, cookie_str)
        .header(CONTENT_TYPE, "text/html")
        .body(Body::from(body))
        .unwrap()
}

/// As soon as you request the `principal: PrincipalOidc` as a parameter, this route can only be
/// accessed with a valid Token. Otherwise, the Principal cannot be build and would return a 401
/// from the extractor function.
pub async fn get_protected(principal: PrincipalOidc) -> Result<Response, RauthyError> {
    // As soon as we get here, the principal is actually valid already.
    // The Principal provides some handy base functions for further easy validation, like:
    //
    // principal.is_admin()?;
    // principal.has_any_group(vec!["group123", "group456"])?;
    // principal.has_any_role(vec!["admin", "root"])?;

    // For very important routes, you could do an additional userinfo fetch, if this feature
    // has been enabled. This should not be done on each request, but may make sense depending
    // on you setup. The userinfo fetch will do an additional online validation of the token
    // and trigger validation + database lookups on Rauthy.
    // The other reason you might want to do this, if you need more information about the user
    // and you only have an access_token and not done the initial login flow to get the id_token.

    let userinfo = principal.fetch_userinfo().await?;
    println!("{:?}", userinfo);

    Ok(Response::new(format!(
        "Hello from Protected Resource:<br/>{:?}",
        principal
    ))
    .into_response())
}
