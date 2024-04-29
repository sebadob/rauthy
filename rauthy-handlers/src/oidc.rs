use crate::{map_auth_step, ReqPrincipal};
use actix_web::cookie::time::OffsetDateTime;
use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
use actix_web::http::{header, StatusCode};
use actix_web::{get, post, web, HttpRequest, HttpResponse, HttpResponseBuilder, ResponseError};
use chrono::Utc;
use rauthy_common::constants::{
    APPLICATION_JSON, AUTH_HEADERS_ENABLE, AUTH_HEADER_EMAIL, AUTH_HEADER_EMAIL_VERIFIED,
    AUTH_HEADER_FAMILY_NAME, AUTH_HEADER_GIVEN_NAME, AUTH_HEADER_GROUPS, AUTH_HEADER_MFA,
    AUTH_HEADER_ROLES, AUTH_HEADER_USER, COOKIE_MFA, DEVICE_GRANT_CODE_LIFETIME,
    DEVICE_GRANT_POLL_INTERVAL, DEVICE_GRANT_RATE_LIMIT, GRANT_TYPE_DEVICE_CODE, HEADER_HTML,
    HEADER_RETRY_NOT_BEFORE, OPEN_USER_REG, SESSION_LIFETIME,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::real_ip_from_req;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::entity::auth_providers::AuthProviderTemplate;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::entity::devices::DeviceAuthCode;
use rauthy_models::entity::ip_rate_limit::DeviceIpRateLimit;
use rauthy_models::entity::jwk::{JWKSPublicKey, JwkKeyPair, JWKS};
use rauthy_models::entity::pow::PowEntity;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::users::User;
use rauthy_models::entity::webauthn::WebauthnCookie;
use rauthy_models::entity::well_known::WellKnown;
use rauthy_models::language::Language;
use rauthy_models::request::{
    AuthRequest, DeviceAcceptedRequest, DeviceGrantRequest, DeviceVerifyRequest,
    LoginRefreshRequest, LoginRequest, LogoutRequest, TokenRequest, TokenValidationRequest,
};
use rauthy_models::response::{
    DeviceCodeResponse, DeviceVerifyResponse, JWKSCerts, JWKSPublicKeyCerts, OAuth2ErrorResponse,
    OAuth2ErrorTypeResponse, SessionInfoResponse,
};
use rauthy_models::templates::{
    AuthorizeHtml, CallbackHtml, Error1Html, ErrorHtml, FrontendAction,
};
use rauthy_models::JwtCommonClaims;
use rauthy_service::auth;
use spow::pow::Pow;
use std::borrow::Cow;
use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error};

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
pub async fn get_authorize(
    data: web::Data<AppState>,
    req: HttpRequest,
    req_data: actix_web_validator::Query<AuthRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find(&data, &req_data.client_id)
        .await
        .unwrap_or_default();
    let lang = Language::try_from(&req).unwrap_or_default();

    let (client, origin_header) = match auth::validate_auth_req_param(
        &data,
        &req,
        &req_data.client_id,
        &req_data.redirect_uri,
        &req_data.code_challenge,
        &req_data.code_challenge_method,
    )
    .await
    {
        Ok(res) => res,
        Err(err) => {
            let status = err.status_code();
            let body = Error1Html::build(&colors, &lang, status, Some(err.message));
            return Ok(ErrorHtml::response(body, status));
        }
    };

    // check prompt and max_age to possibly force a new session
    let mut force_new_session = if req_data
        .prompt
        .as_ref()
        .map(|p| p.as_str() == "login")
        .unwrap_or(false)
    {
        true
    } else if let Some(max_age) = req_data.max_age {
        if let Some(session) = &principal.session {
            let session_created = session.exp - *SESSION_LIFETIME as i64;
            Utc::now().timestamp() > session_created + max_age
        } else {
            true
        }
    } else {
        false
    };

    // check if the user needs to do the Webauthn login each time
    let mut action = FrontendAction::None;
    if let Ok(mfa_cookie) = WebauthnCookie::parse_validate(&req.cookie(COOKIE_MFA)) {
        if let Ok(user) = User::find_by_email(&data, mfa_cookie.email.clone()).await {
            // we need to check this, because a user could deactivate MFA in another browser or
            // be deleted while still having existing mfa cookies somewhere else
            if user.has_webauthn_enabled() {
                action = FrontendAction::MfaLogin(mfa_cookie.email);

                // if the user must do another MFA login anyway, we do never force a new session creation,
                // because the authentication happens each time anyway
                force_new_session = false;
            }
        }
    }

    // check for no-prompt
    if !force_new_session
        && req_data
            .prompt
            .as_ref()
            .map(|p| p.as_str() == "none")
            .unwrap_or(false)
    {
        let status = StatusCode::UNAUTHORIZED;
        let body = Error1Html::build(&colors, &lang, status, Some("login_required".to_string()));
        return Ok(ErrorHtml::response(body, status));
    }

    let auth_providers_json = AuthProviderTemplate::get_all_json_template(&data).await?;
    let tpl_data = Some(format!(
        "{}\n{}\n{}",
        client.name.unwrap_or_default(),
        client.client_uri.unwrap_or_default(),
        *OPEN_USER_REG,
    ));

    // if the user is still authenticated and everything is valid -> immediate refresh
    if !force_new_session && principal.validate_session_auth().is_ok() {
        let csrf = principal.get_session_csrf_token()?;
        let body = AuthorizeHtml::build(
            &tpl_data,
            csrf,
            FrontendAction::Refresh,
            &colors,
            &lang,
            auth_providers_json,
        );

        if let Some(o) = origin_header {
            return Ok(HttpResponse::Ok()
                .insert_header(o)
                .insert_header(HEADER_HTML)
                .body(body));
        }
        return Ok(HttpResponse::Ok().append_header(HEADER_HTML).body(body));
    }

    let session = Session::new(*SESSION_LIFETIME, real_ip_from_req(&req));
    if let Err(err) = session.save(&data).await {
        let status = err.status_code();
        let body = Error1Html::build(&colors, &lang, status, Some(err.message));
        return Ok(ErrorHtml::response(body, status));
    }

    let body = AuthorizeHtml::build(
        &tpl_data,
        &session.csrf_token,
        action,
        &colors,
        &lang,
        auth_providers_json,
    );

    let cookie = session.client_cookie();
    if let Some(o) = origin_header {
        // TODO is 'Access-Control-Allow-Credentials: true' needed as well?
        return Ok(HttpResponse::Ok()
            .cookie(cookie)
            .insert_header(o)
            .insert_header(HEADER_HTML)
            .body(body));
    }
    Ok(HttpResponse::build(StatusCode::OK)
        .cookie(cookie)
        .insert_header(HEADER_HTML)
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
/// - `session-auth`
#[utoipa::path(
    post,
    path = "/oidc/authorize",
    tag = "oidc",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Correct credentials, but needs to continue with Webauthn MFA Login", body = WebauthnLoginResponse),
        (status = 202, description = "Correct credentials and not MFA Login required, adds Location header"),
        (status = 400, description = "Missing / bad input data", body = ErrorResponse),
        (status = 401, description = "Bad input or CSRF Token error", body = ErrorResponse),
    ),
)]
#[post("/oidc/authorize")]
pub async fn post_authorize(
    data: web::Data<AppState>,
    req: HttpRequest,
    req_data: actix_web_validator::Json<LoginRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;

    // TODO refactor login delay to use Instant, which is a bit cleaner
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let session = principal.get_session()?;
    let res = match auth::authorize(&data, &req, req_data.into_inner(), session.clone()).await {
        Ok(auth_step) => map_auth_step(auth_step, &req).await,
        Err(err) => Err(err),
    };

    let ip = real_ip_from_req(&req);
    auth::handle_login_delay(&data, ip, start, &data.caches.ha_cache_config, res).await
}

/// Immediate login refresh with valid session
///
/// This endpoint is used from the login form if an authenticated and valid session still exists
/// and the user wants to log in to another client. Since the session is still valid, an auth code
/// can be issued without further authentication checks to provide a better UX.
#[utoipa::path(
    post,
    path = "/oidc/authorize/refresh",
    tag = "oidc",
    request_body = LoginRefreshRequest,
    responses(
        (status = 202, description = "Accepted"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[post("/oidc/authorize/refresh")]
pub async fn post_authorize_refresh(
    data: web::Data<AppState>,
    req: HttpRequest,
    req_data: actix_web_validator::Json<LoginRefreshRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    let session = principal.validate_session_auth()?;

    let (client, header_origin) = auth::validate_auth_req_param(
        &data,
        &req,
        &req_data.client_id,
        &req_data.redirect_uri,
        &req_data.code_challenge,
        &req_data.code_challenge_method,
    )
    .await?;

    let auth_step =
        auth::authorize_refresh(&data, session, client, header_origin, req_data.into_inner())
            .await?;
    map_auth_step(auth_step, &req)
        .await
        .map(|res| res.0)
        .map_err(|err| err.0)
}

#[get("/oidc/callback")]
pub async fn get_callback_html(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    // TODO can we even be more strict and request session auth here?
    principal.validate_session_auth_or_init()?;
    let colors = ColorEntity::find_rauthy(&data).await?;
    let body = CallbackHtml::build(&colors);
    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
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
pub async fn get_certs(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let jwks = JWKS::find_pk(&data).await?;
    let res = JWKSCerts::from(jwks);
    Ok(HttpResponse::Ok()
        .insert_header((
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_str("*").unwrap(),
        ))
        .json(res))
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
pub async fn get_cert_by_kid(
    data: web::Data<AppState>,
    kid: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    let kp = JwkKeyPair::find(&data, kid.into_inner()).await?;
    let pub_key = JWKSPublicKey::from_key_pair(&kp);
    Ok(HttpResponse::Ok().json(JWKSPublicKeyCerts::from(pub_key)))
}

/// POST for starting an OAuth 2.0 Device Authorization Grant flow
#[utoipa::path(
    post,
    path = "/oidc/device",
    tag = "oidc",
    request_body = DeviceGrantRequest,
    responses(
        (status = 200, description = "Ok", body = DeviceCodeResponse),
        (status = 400, description = "BadRequest", body = OAuth2ErrorResponse),
    ),
)]
#[post("/oidc/device")]
pub async fn post_device_auth(
    data: web::Data<AppState>,
    req: HttpRequest,
    payload: actix_web_validator::Form<DeviceGrantRequest>,
) -> HttpResponse {
    // handle ip rate-limiting
    if DEVICE_GRANT_RATE_LIMIT.is_some() {
        match real_ip_from_req(&req) {
            None => {
                let err = "Cannot extract client IP for rate-limiting - denying request";
                error!("{err}");
                return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::InvalidRequest,
                    error_description: Some(Cow::from(
                        "internal error - cannot extract IP from request",
                    )),
                });
            }
            Some(ip) => {
                if let Some(dt) = DeviceIpRateLimit::is_limited(&data, ip.clone()).await {
                    return HttpResponse::TooManyRequests()
                        .insert_header((HEADER_RETRY_NOT_BEFORE, dt.timestamp()))
                        .json(OAuth2ErrorResponse {
                            error: OAuth2ErrorTypeResponse::InvalidRequest,
                            error_description: Some(Cow::from(format!(
                                "no further requests allowed before: {}",
                                dt
                            ))),
                        });
                }

                if let Err(err) = DeviceIpRateLimit::insert(&data, ip).await {
                    error!(
                        "Error inserting IP into the cache for rate-limiting: {:?}",
                        err
                    );
                }
            }
        };
    }

    // find and validate the client
    let payload = payload.into_inner();
    let client = match Client::find(&data, payload.client_id).await {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::NotFound().json(OAuth2ErrorResponse {
                error: OAuth2ErrorTypeResponse::InvalidClient,
                error_description: Some(Cow::from("`client_id` does not exist")),
            });
        }
    };

    if !client.enabled {
        return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
            error: OAuth2ErrorTypeResponse::UnauthorizedClient,
            error_description: Some(Cow::from("Client has been disabled")),
        });
    }

    if let Err(err) = client.validate_flow(GRANT_TYPE_DEVICE_CODE) {
        return HttpResponse::Forbidden().json(OAuth2ErrorResponse {
            error: OAuth2ErrorTypeResponse::UnauthorizedClient,
            error_description: Some(Cow::from(err.message)),
        });
    }

    let scopes = if let Some(scopes) = payload.scope {
        let iter = scopes.split(' ').collect::<Vec<&str>>();
        for scope in iter {
            if !client.scopes.contains(scope) {
                return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::InvalidScope,
                    error_description: Some(Cow::from(format!(
                        "Allowed scopes: {}",
                        client.scopes
                    ))),
                });
            }
        }
        Some(scopes)
    } else {
        None
    };

    if let Ok(secret) = client.get_secret_cleartext() {
        if secret != payload.client_secret {
            return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                error: OAuth2ErrorTypeResponse::UnauthorizedClient,
                error_description: Some(Cow::from("Invalid `client_secret`")),
            });
        }
    }

    // we are good - create the code
    let code = match DeviceAuthCode::new(&data, scopes, client.id, payload.client_secret).await {
        Ok(code) => code,
        Err(err) => {
            return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                error: OAuth2ErrorTypeResponse::InvalidRequest,
                error_description: Some(Cow::from(err.message)),
            });
        }
    };

    let user_code = code.user_code();
    let verification_uri = code.verification_uri();
    let verification_uri_complete = Some(code.verification_uri_complete());
    let resp = DeviceCodeResponse {
        device_code: &code.device_code,
        user_code,
        verification_uri,
        verification_uri_complete,
        expires_in: *DEVICE_GRANT_CODE_LIFETIME,
        interval: Some(*DEVICE_GRANT_POLL_INTERVAL),
    };

    HttpResponse::Ok().json(resp)
}

/// POST for vertifying an OAuth 2.0 Device Authorization Grant flow
#[utoipa::path(
    post,
    path = "/oidc/device/verify",
    tag = "oidc",
    request_body = DeviceVerifyRequest,
    responses(
        (status = 200, description = "Ok", body = DeviceVerifyResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
    ),
)]
#[post("/oidc/device/verify")]
#[tracing::instrument(level = "debug", skip_all, fields(user_code = payload.user_code))]
pub async fn post_device_verify(
    data: web::Data<AppState>,
    payload: actix_web_validator::Json<DeviceVerifyRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let payload = payload.into_inner();
    debug!("{:?}", payload);

    let challenge = Pow::validate(&payload.pow)?;
    PowEntity::check_prevent_reuse(&data, challenge.to_string()).await?;

    let mut device_code = DeviceAuthCode::find(&data, payload.user_code)
        .await?
        .ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "DeviceAuthCode does not exist".to_string(),
            )
        })?;

    match payload.device_accepted {
        DeviceAcceptedRequest::Accept => {
            device_code.verified_by = Some(principal.user_id()?.to_string());
            device_code.save(&data).await?;
            Ok(HttpResponse::Accepted().finish())
        }
        DeviceAcceptedRequest::Decline => {
            device_code.delete(&data).await?;
            Ok(HttpResponse::NoContent().finish())
        }
        DeviceAcceptedRequest::Pending => Ok(HttpResponse::Ok().json(DeviceVerifyResponse {
            scopes: device_code.scopes,
        })),
    }
}

// Logout HTML page
//
// Returns an HTML page which can be used for logging the user out. Invalidates the session and deletes
// all possibly existing refresh tokens from the database. Does an automatic logout if the
// `id_token_hint` is given.
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
pub async fn get_logout(
    data: web::Data<AppState>,
    req: HttpRequest,
    req_data: actix_web_validator::Query<LogoutRequest>,
    principal: ReqPrincipal,
) -> HttpResponse {
    // If we get any logout errors, maybe because there is no session anymore or whatever happens,
    // just redirect to rauthy's root page, since the user is not logged in anyway anymore.
    let session = match principal.get_session() {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::build(StatusCode::from_u16(302).unwrap())
                .insert_header(("location", "/auth/v1/"))
                .finish()
        }
    };

    let lang = Language::try_from(&req).unwrap_or_default();
    let body = match auth::logout(req_data.into_inner(), session, &data, &lang).await {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::build(StatusCode::from_u16(302).unwrap())
                .insert_header(("location", "/auth/v1/"))
                .finish()
        }
    };

    return HttpResponse::build(StatusCode::OK)
        .append_header(HEADER_HTML)
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
pub async fn post_logout(
    data: web::Data<AppState>,
    req_data: actix_web_validator::Query<LogoutRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    let mut session = principal.get_session()?.clone();
    let cookie = session.invalidate(&data).await?;

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
pub async fn rotate_jwk(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Secrets, AccessRights::Update)?;

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
pub async fn get_session_info(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;
    let session = principal.get_session()?;

    // let timeout_secs = session.last_seen.timestamp() + data.session_timeout as i64;
    let timeout = OffsetDateTime::from_unix_timestamp(session.last_seen)
        .unwrap()
        .add(::time::Duration::seconds(data.session_timeout as i64));
    let info = SessionInfoResponse {
        id: &session.id,
        csrf_token: None,
        user_id: session.user_id.as_ref(),
        roles: session.roles.as_ref(),
        groups: session.groups.as_ref(),
        exp: OffsetDateTime::from_unix_timestamp(session.exp).unwrap(),
        timeout,
    };

    Ok(HttpResponse::Ok().json(info))
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
pub async fn get_session_xsrf(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;
    let session = principal.get_session()?;

    let timeout = OffsetDateTime::from_unix_timestamp(session.last_seen)
        .unwrap()
        .add(::time::Duration::seconds(data.session_timeout as i64));
    let info = SessionInfoResponse {
        id: &session.id,
        csrf_token: Some(&session.csrf_token),
        user_id: session.user_id.as_ref(),
        roles: session.roles.as_ref(),
        groups: session.groups.as_ref(),
        exp: OffsetDateTime::from_unix_timestamp(session.exp).unwrap(),
        timeout,
    };
    Ok(HttpResponse::Ok().json(info))
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
    request_body(content = TokenRequest, content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 200, description = "Ok", body = TokenSet),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/oidc/token")]
#[tracing::instrument(level = "debug", skip_all, fields(grant_type = payload.grant_type))]
pub async fn post_token(
    req: HttpRequest,
    data: web::Data<AppState>,
    payload: actix_web_validator::Form<TokenRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let ip = real_ip_from_req(&req);

    if payload.grant_type == GRANT_TYPE_DEVICE_CODE {
        // TODO the `urn:ietf:params:oauth:grant-type:device_code` needs
        // a fully customized handling here with customized error response
        // to meet the oauth rfc
        return Ok(auth::grant_type_device_code(&data, ip, payload.into_inner()).await);
    }

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let add_login_delay = payload.grant_type == "password";

    let res = match auth::get_token_set(payload.into_inner(), &data, req).await {
        Ok((token_set, headers)) => {
            let mut builder = HttpResponseBuilder::new(StatusCode::OK);
            for h in headers {
                builder.insert_header(h);
            }
            let resp = builder.json(token_set);
            Ok((resp, add_login_delay))
        }
        Err(err) => Err((err, add_login_delay)),
    };

    auth::handle_login_delay(&data, ip, start, &data.caches.ha_cache_config, res).await
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
pub async fn post_token_info(
    data: web::Data<AppState>,
    req_data: actix_web_validator::Json<TokenValidationRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    auth::get_token_info(&data, &req_data.token)
        .await
        .map(|i| HttpResponse::Ok().json(i))
}

// // TODO remove?
// /// DEPRECATED
// ///
// /// This is an older endpoint for refreshing tokens manually. This is not being used anymore an will
// /// be removed soon in favor of the `refresh_token` flow on the [token](post_token) endpoint.
// #[utoipa::path(
//     post,
//     path = "/oidc/token/refresh",
//     tag = "deprecated",
//     request_body = RefreshTokenRequest,
//     responses(
//         (status = 200, description = "Ok", body = TokenSet),
//         (status = 401, description = "Unauthorized", body = ErrorResponse),
//         (status = 404, description = "NotFound", body = ErrorResponse),
//     ),
// )]
// #[post("/oidc/token/refresh")]
// pub async fn post_refresh_token(
//     req_data: actix_web_validator::Json<RefreshTokenRequest>,
//     data: web::Data<AppState>,
// ) -> Result<HttpResponse, ErrorResponse> {
//     auth::validate_refresh_token(None, &req_data.refresh_token, &data)
//         .await
//         .map(|token_set| HttpResponse::Ok().json(token_set))
// }

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
pub async fn get_userinfo(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    auth::get_userinfo(&data, req)
        .await
        .map(|u| HttpResponse::Ok().json(u))
}

/// GET forward authentication
///
/// This endpoint is very similar to the `/userinfo`, but instead of returning information about
/// the user inside the body, information is added to the headers with an empty body.
/// This makes it possible to use this endpoint in combination with a reverse proxy and can be used
/// for authn/authz on downstream applications via Headers.
///
/// However, header based authentication as a lot of pitfalls outside the scope of Rauthy and
/// you should only use it, if your downstream apps do not support OIDC natively, and you have no
/// other choice.
///
/// Even though forward auth can be used to check general authentication / access to an application,
/// it can never implement a really secure, proper way to mitigate potential CSRF Attacks. This is
/// something, that the downstream application would have to manage.
#[utoipa::path(
    get,
    path = "/oidc/forward_auth",
    tag = "oidc",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[get("/oidc/forward_auth")]
pub async fn get_forward_auth(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    let info = auth::get_userinfo(&data, req).await?;

    if *AUTH_HEADERS_ENABLE {
        Ok(HttpResponse::Ok()
            .insert_header((AUTH_HEADER_USER.as_str(), info.id))
            .insert_header((AUTH_HEADER_ROLES.as_str(), info.roles.join(",")))
            .insert_header((
                AUTH_HEADER_GROUPS.as_str(),
                info.groups.map(|g| g.join(",")).unwrap_or_default(),
            ))
            .insert_header((AUTH_HEADER_EMAIL.as_str(), info.email.unwrap_or_default()))
            .insert_header((
                AUTH_HEADER_EMAIL_VERIFIED.as_str(),
                info.email_verified.unwrap_or(false).to_string(),
            ))
            .insert_header((
                AUTH_HEADER_FAMILY_NAME.as_str(),
                info.family_name.unwrap_or_default(),
            ))
            .insert_header((
                AUTH_HEADER_GIVEN_NAME.as_str(),
                info.given_name.unwrap_or_default(),
            ))
            .insert_header((AUTH_HEADER_MFA.as_str(), info.mfa_enabled.to_string()))
            .finish())
    } else {
        Ok(HttpResponse::Ok().finish())
    }
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
pub async fn get_well_known(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let wk = WellKnown::json(&data).await?;
    Ok(HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, APPLICATION_JSON))
        .insert_header((
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_str("*").unwrap(),
        ))
        .body(wk))
}
