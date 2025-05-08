use crate::config::Config;
use crate::{templates, DEV_MODE};
use axum::body::Body;
use axum::extract::Query;
use axum::http::header::{CONTENT_TYPE, SET_COOKIE};
use axum::response::{IntoResponse, Response};
use axum_extra::extract::CookieJar;
use rauthy_client::backchannel_logout::logout_token::LogoutToken;
use rauthy_client::handler::{OidcCallbackParams, OidcCookieInsecure, OidcSetRedirectStatus};
use rauthy_client::principal::PrincipalOidc;
use rauthy_client::secure_random;
use std::sync::Arc;
use tracing::{error, info};

type ConfigExt = axum::extract::State<Arc<Config>>;

static SESSION_COOKIE_KEY: &str = "ExampleSession";

/// Index HTML
pub async fn get_index(config: ConfigExt) -> Response<Body> {
    let logout_uri = format!("{}/oidc/logout", config.iss);
    let body = templates::HTML_INDEX.replace("{{ LOGOUT_URI }}", &logout_uri);

    Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "text/html")
        .body(Body::from(body))
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
    jar: CookieJar,
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
    let (cookie_str, token_set, id_claims) = match callback_res {
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

    // This is a very naive approach to HTML templating and only for simplicity in this example.
    // Please don't do this in production and use a proper templating engine.
    // This HTML template will save the access token in local storage (be careful with that in prod).
    // This will make it possible to use the `get_protected()` route below via a token.
    let body = templates::HTML_CALLBACK
        .replace("{{ ACCESS_TOKEN }}", &token_set.access_token)
        .replace("{{ ID_TOKEN }}", token_set.id_token.as_ref().unwrap())
        .replace("{{ URI }}", "/");

    // Build our very simple Session
    let sid = secure_random(48);
    // CAUTION: NEVER build an insecure cookie like this. ALWAYS set the `Secure` flag.
    // This is only for the example.
    let session_cookie =
        format!("{SESSION_COOKIE_KEY}={sid}; Path=/; HttpOnly; SameSite=Lax; Max-Age=3600");
    {
        let mut lock = config.sessions.write().await;
        lock.push((sid, id_claims.sub.unwrap()));
    }

    Response::builder()
        .status(200)
        // we should append the returned cookie jar here to
        // delete the state cookie from the login flow
        .header(SET_COOKIE, cookie_str)
        // To be able to show Backchannel Logout as well,
        // we will do a (very simplified) session approach.
        .header(SET_COOKIE, session_cookie)
        .header(CONTENT_TYPE, "text/html")
        .body(Body::from(body))
        .unwrap()
}

/// This is the endpoint for backchannel logouts.
pub async fn post_logout(config: ConfigExt, logout_token: LogoutToken) -> Response {
    // If you actually get here, the `LogoutToken` has already been validated and you can be sure
    // it actually came from your configured Rauthy instance and it valid.
    //
    // You now only need to log out the user it refers to.
    // - If the `sub` is `Some(_)`, you should log out the whole user with all possibliy
    // existing sessions.
    // - If the `sid` is `Some(_)`, only the mentioned Session should be logged out.

    // You may optionally cache the `jti` to prevent token replays, but in reality, this cannot
    // happen anyway as long as you are using TLS, which you should always do anyway.

    let mut lock = config.sessions.write().await;
    if let Some(sub) = logout_token.sub {
        info!("Received a Logout Token for user ID {sub}");
        lock.retain(|(_, uid)| uid != &sub);
    } else if let Some(sid) = logout_token.sid {
        info!("Received a Logout Token for Session ID {sid}");
        lock.retain(|(id, _)| id != &sid);
    }

    Response::default()
}

/// As soon as you request the `principal: PrincipalOidc` as a parameter, this route can only be
/// accessed with a valid Token. Otherwise, the Principal cannot be build and would return a 401
/// from the extractor function.
pub async fn get_protected(principal: PrincipalOidc) -> Response {
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
    //
    // let userinfo = principal.fetch_userinfo().await?;

    Response::new(format!(
        "Hello from Token-Protected Resource:<br/>{:?}",
        principal
    ))
    .into_response()
}

/// This endpoitn will only allow access with our very simple Session. JWT tokens are good for
/// initializing a session, but not that good for ongoing authn/authz, since they cannot be revoked.
/// However, a token-based authn/authz scales better (when done right) and is way easier of course.
/// Both approaches have their up's and downs. We show the session as well to be able to demostrate
/// OIDC Backchannmel Logut, since tokens cannot be revoked.
pub async fn get_session(config: ConfigExt, jar: CookieJar) -> Response {
    let sid = match jar.get(SESSION_COOKIE_KEY) {
        None => {
            error!("Session Cookie does not exist in Jar");
            return Response::builder()
                .status(401)
                .body(Body::empty())
                .unwrap()
                .into_response();
        }
        Some(cookie) => {
            let sid = cookie.value();
            info!("Session ID from Cookie: {sid}");

            let lock = config.sessions.read().await;
            if !lock.iter().any(|(id, _)| id == sid) {
                return Response::builder()
                    .status(401)
                    .body(Body::empty())
                    .unwrap()
                    .into_response();
            }

            sid.to_string()
        }
    };

    Response::new(format!(
        "Hello from Session-Protected Resource:<br/>Session ID: {}",
        sid
    ))
    .into_response()
}
