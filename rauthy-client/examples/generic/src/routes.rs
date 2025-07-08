use crate::config::Config;
use crate::{extractors, templates, DEV_MODE};
use actix_web::http::header::{CONTENT_TYPE, LOCATION, SET_COOKIE};
use actix_web::web::Query;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use rauthy_client::cookie_state::OidcCookieState;
use rauthy_client::handler::{OidcCallbackParams, OidcCookieInsecure};

type ConfigExt = web::Data<Config>;

/// Index HTML
#[get("/")]
pub async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .append_header((CONTENT_TYPE, "text/html"))
        .body(templates::HTML_INDEX)
}

/// OIDC Auth check and login
///
/// Endpoint with no redirect on purpose to use the result inside Javascript from the frontend.
/// HTTP 202 means logged in Principal
/// HTTP 406 will have a location header and a manual redirect must be done
#[get("/auth_check")]
pub async fn get_auth_check(config: ConfigExt, req: HttpRequest) -> HttpResponse {
    let enc_key = config.enc_key.as_slice();

    // We need to get the principal manually.
    // We use our own custom helper function to extract the principal.
    let principal = extractors::principal_opt_from_req(&req).await;

    // if we are in dev mode, we allow insecure cookies
    let insecure = if DEV_MODE {
        OidcCookieInsecure::Yes
    } else {
        OidcCookieInsecure::No
    };

    match rauthy_client::handler::validate_principal_generic(principal, enc_key, insecure).await {
        Ok(()) => HttpResponse::Accepted().finish(),
        Err(header_values) => match header_values {
            // we return HTTP 406 to not trigger default browser behavior from 401
            Some((loc, state)) => HttpResponse::NotAcceptable()
                .append_header((LOCATION, loc))
                .append_header((SET_COOKIE, state))
                .finish(),
            None => HttpResponse::InternalServerError().body("OIDC Provider is not set up"),
        },
    }
}

/// OIDC Callback endpoint - must match the `redirect_uri` for the login flow
#[get("/callback")]
pub async fn get_callback(
    req: HttpRequest,
    config: ConfigExt,
    // How you extract these OidcCallbackParams depends on your setup.
    // These are query parameters that are being appended to the callback URL.
    // You just need to extract these in whatever way makes sense to you.
    // We need `code` and `state` params here to proceed.
    params: Query<OidcCallbackParams>,
) -> HttpResponse {
    let enc_key = config.enc_key.as_slice();

    // We need to manually extract the state cookie
    let cookie_state = match req.cookie(rauthy_client::cookie_state::OIDC_STATE_COOKIE) {
        None => {
            tracing::warn!("STATE_COOKIE is missing - Request may have expired");
            return HttpResponse::BadRequest()
                .body("STATE_COOKIE is missing - Request may have expired");
        }
        Some(cookie) => match OidcCookieState::from_cookie_value(cookie.value(), enc_key) {
            Ok(cookie_state) => cookie_state,
            Err(err) => {
                return HttpResponse::BadRequest().body(err.to_string());
            }
        },
    };

    // The `DEV_MODE` again here to just have a nicer DX when developing -> we allow insecure cookies
    let insecure = if DEV_MODE {
        OidcCookieInsecure::Yes
    } else {
        OidcCookieInsecure::No
    };

    let callback_res =
        rauthy_client::handler::oidc_callback(cookie_state, params.into_inner(), insecure).await;
    let (cookie_str, token_set, _id_claims) = match callback_res {
        Ok(res) => res,
        Err(err) => {
            return HttpResponse::BadRequest().body(format!("Invalid OIDC Callback: {err}"))
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

    HttpResponse::Ok()
        .append_header((SET_COOKIE, cookie_str))
        .append_header((CONTENT_TYPE, "text/html"))
        .body(body)
}

#[get("/protected")]
pub async fn get_protected(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let principal = extractors::principal_from_req(&req).await?;

    Ok(HttpResponse::Ok().body(format!("Hello from Protected Resource:<br/>{principal:?}")))
}
