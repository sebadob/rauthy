use crate::config::Config;
use crate::{templates, DEV_MODE};
use actix_web::http::header::{CONTENT_TYPE, SET_COOKIE};
use actix_web::web::Query;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use rauthy_client::handler::{OidcCallbackParams, OidcCookieInsecure, OidcSetRedirectStatus};
use rauthy_client::principal::PrincipalOidc;

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
/// HTTP 200 will have a location header and a manual redirect must be done
/// HTTP 202 means logged in Principal
#[get("/auth_check")]
pub async fn get_auth_check(config: ConfigExt, principal: Option<PrincipalOidc>) -> impl Responder {
    let enc_key = config.enc_key.as_slice();

    // if we are in dev mode, we allow insecure cookies
    let insecure = if DEV_MODE {
        OidcCookieInsecure::Yes
    } else {
        OidcCookieInsecure::No
    };

    rauthy_client::handler::actix_web::validate_redirect_principal(
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
#[get("/callback")]
pub async fn get_callback(
    req: HttpRequest,
    config: ConfigExt,
    params: Query<OidcCallbackParams>,
) -> HttpResponse {
    let enc_key = config.enc_key.as_slice();

    // The `DEV_MODE` again here to just have a nicer DX when developing -> we allow insecure cookies
    let insecure = if DEV_MODE {
        OidcCookieInsecure::Yes
    } else {
        OidcCookieInsecure::No
    };
    let callback_res =
        rauthy_client::handler::actix_web::oidc_callback(&req, params, enc_key, insecure).await;
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

/// As soon as you request the `principal: PrincipalOidc` as a parameter, this route can only be
/// accessed with a valid Token. Otherwise, the Principal cannot be build and would return a 401
/// from the extractor function.
#[get("/protected")]
pub async fn get_protected(principal: PrincipalOidc) -> Result<HttpResponse, actix_web::Error> {
    // As soon as we get here, the principal is actually valid already.
    // The Principal provides some handy base functions for further easy validation, like:
    //
    // principal.is_admin()?;
    // principal.has_any_group(vec!["group123", "group456"])?;
    // principal.has_any_role(vec!["admin", "root"])?;

    Ok(HttpResponse::Ok().body(format!("Hello from Protected Resource:<br/>{principal:?}")))
}
