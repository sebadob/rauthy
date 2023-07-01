use crate::{build_csp_header, map_auth_step};
use actix_web::cookie::time::OffsetDateTime;
use actix_web::http::{header, StatusCode};
use actix_web::{cookie, get, post, web, HttpRequest, HttpResponse};
use actix_web_grants::proc_macro::{has_any_permission, has_permissions, has_roles};
use rauthy_common::constants::{COOKIE_MFA, COOKIE_SESSION, HEADER_HTML, SESSION_LIFETIME};
use rauthy_common::error_response::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::entity::jwk::{JWKSPublicKey, JwkKeyPair, JWKS};
use rauthy_models::entity::principal::Principal;
use rauthy_models::entity::sessions::{Session, SessionState};
use rauthy_models::entity::users::User;
use rauthy_models::entity::webauthn::WebauthnCookie;
use rauthy_models::request::{
    AuthRequest, LoginRefreshRequest, LoginRequest, LogoutRequest, RefreshTokenRequest,
    TokenRequest, TokenValidationRequest,
};
use rauthy_models::response::{JWKSCerts, JWKSPublicKeyCerts, SessionInfoResponse};
use rauthy_models::templates::{AuthorizeHtml, CallbackHtml, FrontendAction};
use rauthy_models::JwtCommonClaims;
use rauthy_service::auth;
use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};

/// OIDC Authorization HTML
///
/// Starts the authorization_code flow. Log in with username / password.<br>
/// If one does not exist, a new session will be opened with the 'Init' state and set's a cookie.
#[utoipa::path(
    get,
    path = "/oidc/authorize",
    tag = "oidc",
    params(AuthRequest),
    responses(
        (status = 200, description = "If the params match the allowed settings, returns the pre-rendered HTML",),
        (status = 400, description = "If any params do not match the backend config", body = ErrorResponse),
    ),
)]
#[get("/oidc/authorize")]
#[has_permissions("all")]
pub async fn get_authorize(
    data: web::Data<AppState>,
    req: HttpRequest,
    req_data: actix_web_validator::Query<AuthRequest>,
    session: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let (client, origin_header) = auth::validate_auth_req_param(
        &data,
        &req,
        &req_data.client_id,
        &req_data.redirect_uri,
        &req_data.code_challenge,
        &req_data.code_challenge_method,
    )
    .await?;

    let colors = ColorEntity::find(&data, &req_data.client_id).await?;

    if session.is_some() && session.as_ref().unwrap().state == SessionState::Auth {
        let (body, nonce) = AuthorizeHtml::build(
            &client.name,
            &session.as_ref().unwrap().csrf_token,
            FrontendAction::Refresh,
            &colors,
        );

        // TODO make this prettier - append header conditionally easier?
        if let Some(o) = origin_header {
            return Ok(HttpResponse::Ok()
                .insert_header(o)
                .insert_header(HEADER_HTML)
                .insert_header(build_csp_header(&nonce))
                .body(body));
        }
        return Ok(HttpResponse::Ok()
            .append_header(HEADER_HTML)
            .append_header(build_csp_header(&nonce))
            .body(body));
    }

    let session = Session::new(None, *SESSION_LIFETIME);
    session.save(&data).await?;

    let mut action = FrontendAction::None;

    if let Ok(mfa_cookie) = WebauthnCookie::parse_validate(&req.cookie(COOKIE_MFA), &data.enc_keys)
    {
        if let Ok(user) = User::find_by_email(&data, mfa_cookie.email.clone()).await {
            // we need to check this, because a user could deactivate MFA in another browser or
            // be deleted while still having existing mfa cookies somewhere else
            if user.has_webauthn_enabled() {
                action = FrontendAction::MfaLogin(mfa_cookie.email);
            }
        }
    }

    // if let Some(mfa_cookie) = req.cookie(COOKIE_MFA) {
    //     action = FrontendAction::MfaLogin(mfa_cookie.value().to_string())
    // }

    let (body, nonce) = AuthorizeHtml::build(&client.name, &session.csrf_token, action, &colors);

    let cookie = session.client_cookie();
    if let Some(o) = origin_header {
        // TODO is 'Access-Control-Allow-Credentials: true' needed as well?
        return Ok(HttpResponse::Ok()
            .cookie(cookie)
            .insert_header(o)
            .insert_header(HEADER_HTML)
            .insert_header(build_csp_header(&nonce))
            .body(body));
    }
    Ok(HttpResponse::build(StatusCode::OK)
        .cookie(cookie)
        .insert_header(HEADER_HTML)
        .insert_header(build_csp_header(&nonce))
        .body(body))
}

/// POST login credentials to proceed with the authorization_code flow
///
/// This is the 2nd step of the authorization_code flow.<br>
/// The already validated params from the GET /auth/v1/oidc/authorize + Username + Password + CSRF Token
/// + *rauthy-session* Cookie will be validated. On success, it will return an empty body but with
/// **Location** and possibly **Allowed-Origins** header set.<br>
/// If the user has MFA configured, this will be requested after a successful login via credentials
/// first.
///
/// **Permissions**
/// - `session-init`
#[utoipa::path(
    post,
    path = "/oidc/authorize",
    tag = "oidc",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Correct credentials, but needs to continue with Webauthn MFA Login", body = MfaLoginRequestAwait),
        (status = 202, description = "Correct credentials and not MFA Login required, adds Location header"),
        (status = 400, description = "Missing / bad input data", body = ErrorResponse),
        (status = 401, description = "Bad input or CSRF Token error", body = ErrorResponse),
    ),
)]
#[post("/oidc/authorize")]
#[has_any_permission("session-auth", "session-init")]
pub async fn post_authorize(
    data: web::Data<AppState>,
    req: HttpRequest,
    req_data: web::Json<LoginRequest>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    // TODO refactor login delay to use Instant, which is a bit cleaner
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let session = Session::extract_validate_csrf(session_req, &req)?;
    let res = auth::authorize(&data, &req, req_data.into_inner(), session)
        .await
        .map(|auth_step| map_auth_step(&data, auth_step, &req))?;

    auth::handle_login_delay(start, &data.caches.ha_cache_config, res, true).await
}

#[get("/oidc/callback")]
#[has_permissions("all")]
pub async fn get_callback_html(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy(&data).await?;
    let (body, nonce) = CallbackHtml::build(&colors);

    Ok(HttpResponse::Ok()
        .insert_header(HEADER_HTML)
        .insert_header(build_csp_header(&nonce))
        .body(body))
}

// TODO clean up?
/// DEPRECATED
///
/// This is an older refresh token endpoint, which is not being used anymore in favor of the
/// *refresh_token* grant on the [POST /token](post_token) endpoint.
#[utoipa::path(
    post,
    path = "/oidc/authorize/refresh",
    tag = "deprecated",
    request_body = LoginRefreshRequest,
    responses(
        (status = 202, description = "Accepted"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[post("/oidc/authorize/refresh")]
#[has_permissions("session-auth")]
pub async fn post_authorize_refresh(
    data: web::Data<AppState>,
    req: HttpRequest,
    req_data: web::Json<LoginRefreshRequest>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let (client, header_origin) = auth::validate_auth_req_param(
        &data,
        &req,
        &req_data.client_id,
        &req_data.redirect_uri,
        &req_data.code_challenge,
        &req_data.code_challenge_method,
    )
    .await?;

    let session = Session::extract_validate_csrf(session_req, &req)?;
    auth::authorize_refresh(&data, session, client, header_origin, req_data.into_inner())
        .await
        .map(|auth_step| map_auth_step(&data, auth_step, &req))?
}

/// JWT Token public JWKS
///
/// Returns the Json Web Key Set (JWKS) for independent validation of the signed JWT Tokens.
#[utoipa::path(
    get,
    path = "/oidc/certs",
    tag = "oidc",
    responses((status = 200, description = "Ok")),
)]
#[get("/oidc/certs")]
#[has_permissions("all")]
pub async fn get_certs(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let jwks = JWKS::find_pk(&data).await?;
    let res = JWKSCerts::from(jwks);
    Ok(HttpResponse::Ok().json(res))
}

/// Single JWK by kid
///
/// Returns a single Json Web Key (JWK) by given kid
#[utoipa::path(
    get,
    path = "/oidc/certs/{kid}",
    tag = "oidc",
    responses((status = 200, description = "Ok")),
)]
#[get("/oidc/certs/{kid}")]
#[has_permissions("all")]
pub async fn get_cert_by_kid(
    data: web::Data<AppState>,
    kid: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    let kp = JwkKeyPair::find(&data, kid.into_inner()).await?;
    let pub_key = JWKSPublicKey::from_key_pair(&kp);
    Ok(HttpResponse::Ok().json(JWKSPublicKeyCerts::from(pub_key)))
}

/// Logout HTML page
///
/// Returns an HTML page which can be used for logging the user out. Invalidates the session and deletes
/// all possibly existing refresh tokens from the database. Does an automatic logout if the
/// `id_token_hint` is given.
#[utoipa::path(
    get,
    path = "/oidc/logout",
    tag = "oidc",
    params(LogoutRequest),
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
    ),
)]
#[get("/oidc/logout")]
#[has_permissions("all")]
pub async fn get_logout(
    data: web::Data<AppState>,
    req_data: web::Query<LogoutRequest>,
    session_req: web::ReqData<Option<Session>>,
) -> HttpResponse {
    // If we get any logout errors, maybe because there is no session anymore or whatever happens,
    // just redirect to rauthy's root page, since the user is not logged in anyway anymore.
    let session = match Session::extract_from_req(session_req) {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::build(StatusCode::from_u16(302).unwrap())
                .insert_header(("location", "/auth/v1/"))
                .finish()
        }
    };

    let (body, nonce) = match auth::logout(req_data.into_inner(), session, &data).await {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::build(StatusCode::from_u16(302).unwrap())
                .insert_header(("location", "/auth/v1/"))
                .finish()
        }
    };

    return HttpResponse::build(StatusCode::OK)
        .append_header(HEADER_HTML)
        .append_header(build_csp_header(&nonce))
        .body(body);
}

/// Send the logout confirmation
///
/// This is the corresponding endpoint for the `GET /auth/v1/oidc/logout`
#[utoipa::path(
    post,
    path = "/oidc/logout",
    tag = "oidc",
    params(LogoutRequest),
    responses(
        (status = 200, description = "Ok without `post_logout_redirect_uri`"),
        (status = 301, description = "if a `post_logout_redirect_uri` was given"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
    ),
)]
#[post("/oidc/logout")]
#[has_permissions("all")]
pub async fn post_logout(
    data: web::Data<AppState>,
    req_data: web::Query<LogoutRequest>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let mut session = Session::extract_from_req(session_req)?;
    session.invalidate(&data).await?;

    let cookie = cookie::Cookie::build(COOKIE_SESSION, "")
        .max_age(cookie::time::Duration::ZERO)
        .finish();

    if req_data.post_logout_redirect_uri.is_some() {
        let state = if req_data.state.is_some() {
            req_data.state.as_ref().unwrap().as_str()
        } else {
            ""
        };
        let loc = format!(
            "{}?state={}",
            req_data.post_logout_redirect_uri.as_ref().unwrap(),
            state
        );
        return Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
            .append_header((header::LOCATION, loc))
            .cookie(cookie)
            .finish());
    }

    return Ok(HttpResponse::build(StatusCode::OK).cookie(cookie).finish());
}

/// Rotate JWKs
///
/// Rotates all currently exiting JWKs (Json Web Keys) for signing new tokens. This is a manual
/// operation currently, but this may be handled by a scheduler in the future.<br>
/// When the JWKs are rotated, all newly signed tokens from that point on will use the completely random
/// secure new JWKs.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/oidc/rotateJwk",
    tag = "oidc",
    params(LogoutRequest),
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[post("/oidc/rotateJwk")]
#[has_roles("rauthy_admin")]
pub async fn rotate_jwk(
    data: web::Data<AppState>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::get_from_req(principal.into_inner())?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

    auth::rotate_jwks(&data)
        .await
        .map(|_| HttpResponse::Ok().finish())
}

/// OIDC sessioninfo
///
/// Returns information about the current session. This is currently only used in the Rauthy Admin UI
/// and does not make much sense anywhere else. Only works with a fully authenticated session.
///
/// **Permissions**
/// - session-auth
#[utoipa::path(
    post,
    path = "/oidc/sessioninfo",
    tag = "oidc",
    params(LogoutRequest),
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/oidc/sessioninfo")]
#[has_permissions("session-auth")]
pub async fn get_session_info(
    data: web::Data<AppState>,
    session_req: web::ReqData<Option<Session>>,
) -> HttpResponse {
    // direct unwrap is safe: 'session-auth' permission means there can only be an active session
    let session = session_req.into_inner().unwrap();
    // let timeout_secs = session.last_seen.timestamp() + data.session_timeout as i64;
    let timeout = OffsetDateTime::from_unix_timestamp(session.last_seen)
        .unwrap()
        .add(::time::Duration::seconds(data.session_timeout as i64));
    let info = SessionInfoResponse {
        id: session.id,
        csrf_token: None,
        user_id: session.user_id,
        roles: session.roles,
        groups: session.groups,
        exp: OffsetDateTime::from_unix_timestamp(session.exp).unwrap(),
        timeout,
    };
    HttpResponse::Ok().json(info)
}

// TODO maybe generate a new csrf token each time this endpoint is used. This would boost the security
// but at the same time make it impossible to have 2 windows of rauthy open in 2 browsers at the
// same time, since they would invalidate each others XSRF tokens. Additionally, external clients
// could just use this endpoint (which they usually should not by specification) and generate a new
// token without the user knowing about it. --> Think about it
/// Session CSRF Token
///
/// Returns the CSRF token for the current session in exchange for a valid `access_token`.
/// Returning the CSRF token this way is safe, since it needs the Session Cookie + JWT Token, which
/// is never set as a cookie at all.
///
/// **Permissions**
/// - token-auth && session-auth
#[utoipa::path(
    get,
    path = "/oidc/sessioninfo/xsrf",
    tag = "oidc",
    params(LogoutRequest),
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/oidc/sessioninfo/xsrf")]
#[has_permissions("token-auth", "session-auth")]
pub async fn get_session_xsrf(
    data: web::Data<AppState>,
    session_req: web::ReqData<Option<Session>>,
) -> HttpResponse {
    // direct unwrap is safe: 'session-auth' permission means there can only be an active session
    let session = session_req.into_inner().unwrap();
    let timeout = OffsetDateTime::from_unix_timestamp(session.last_seen)
        .unwrap()
        .add(::time::Duration::seconds(data.session_timeout as i64));
    let info = SessionInfoResponse {
        id: session.id,
        csrf_token: Some(session.csrf_token),
        user_id: session.user_id,
        roles: session.roles,
        groups: session.groups,
        exp: OffsetDateTime::from_unix_timestamp(session.exp).unwrap(),
        timeout,
    };
    HttpResponse::Ok().json(info)
}

/// The token endpoint for the OAuth2 / OIDC workflow.
///
/// The accepted options and values depend on the clients config.<br>
/// Username enumeration will be prevented and no matter if the request was successful or not, it will
/// always take about the same amount of time, except for the first 5-7 (successful) request after
/// a fresh restart of the application.
#[utoipa::path(
    post,
    path = "/oidc/token",
    tag = "oidc",
    request_body = TokenRequest,
    responses(
        (status = 200, description = "Ok", body = TokenSet),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/oidc/token")]
#[has_permissions("all")]
pub async fn post_token(
    req_data: actix_web_validator::Form<TokenRequest>,
    req: HttpRequest,
    session_req: web::ReqData<Option<Session>>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let save_timer = req_data.grant_type == "password";

    let csrf_header = if let Some(s) = session_req.into_inner() {
        // TODO validate CSRF if a session is present? What about the RFC? -> Check
        Session::get_csrf_header(&s.csrf_token)
    } else {
        Session::get_csrf_header("none")
    };

    let res = auth::get_token_set(req_data.into_inner(), &data, req)
        .await
        .map(|(token_set, header_origin)| {
            if let Some(o) = header_origin {
                return HttpResponse::Ok()
                    .insert_header(o)
                    .insert_header(csrf_header)
                    .json(token_set);
            }
            HttpResponse::Ok()
                .insert_header(csrf_header)
                .json(token_set)
        });

    auth::handle_login_delay(start, &data.caches.ha_cache_config, res, save_timer).await
}

/// The tokenInfo endpoint for the OIDC standard.
#[utoipa::path(
    post,
    path = "/oidc/tokenInfo",
    tag = "oidc",
    request_body = TokenValidationRequest,
    responses(
        (status = 200, description = "Ok", body = TokenInfo),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/oidc/tokenInfo")]
#[has_permissions("all")]
pub async fn post_token_info(
    data: web::Data<AppState>,
    req_data: actix_web_validator::Json<TokenValidationRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    auth::get_token_info(&data, &req_data.token)
        .await
        .map(|i| HttpResponse::Ok().json(i))
}

// TODO remove?
/// DEPRECATED
///
/// This is an older endpoint for refreshing tokens manually. This is not being used anymore an will
/// be removed soon in favor of the `refresh_token` flow on the [token](post_token) endpoint.
#[utoipa::path(
    post,
    path = "/oidc/token/refresh",
    tag = "deprecated",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Ok", body = TokenSet),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/oidc/token/refresh")]
#[has_permissions("all")]
pub async fn post_refresh_token(
    req_data: actix_web_validator::Json<RefreshTokenRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    auth::validate_refresh_token(None, &req_data.refresh_token, &data)
        .await
        .map(|token_set| HttpResponse::Ok().json(token_set))
}

/// DEPRECATED
///
/// This is an older endpoint for validating tokens manually. This is not being used anymore an will
/// be removed soon in favor of the [userinfo](get_userinfo) endpoint.
#[utoipa::path(
    post,
    path = "/oidc/token/validate",
    tag = "deprecated",
    request_body = TokenValidationRequest,
    responses(
        (status = 202, description = "Accepted"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[post("/oidc/token/validate")]
#[has_permissions("all")]
pub async fn post_validate_token(
    data: web::Data<AppState>,
    req_data: actix_web_validator::Json<TokenValidationRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    auth::validate_token::<JwtCommonClaims>(&data, &req_data.token)
        .await
        .map(|_| HttpResponse::Accepted().finish())
}

/// The userinfo endpoint for the OIDC standard.
///
/// Depending on the JWT token from the *Authorization* header, it will return information about
/// the requesting user / token.
#[utoipa::path(
    post,
    path = "/oidc/userinfo",
    tag = "oidc",
    responses(
        (status = 200, description = "Ok", body = Userinfo),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/oidc/userinfo")]
#[has_permissions("token-auth")]
pub async fn get_userinfo(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    auth::get_userinfo(&data, req)
        .await
        .map(|u| HttpResponse::Ok().json(u))
}

/// The `.well-known` endpoint for OIDC Client auto discovery.
///
/// Capable OIDC clients can use this endpoint to auto-discover all necessary OIDC information and
/// endpoints that are provided by rauthy to automatically choose the best / safest options.
#[utoipa::path(
    get,
    path = "/.well-known/openid-configuration",
    tag = "oidc",
    responses(
        (status = 200, description = "Ok", body = WellKnown),
    ),
)]
#[get("/.well-known/openid-configuration")]
#[has_permissions("all")]
pub async fn get_well_known(data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(&data.well_known)
}
