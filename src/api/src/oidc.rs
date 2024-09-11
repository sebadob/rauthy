use crate::{map_auth_step, ReqPrincipal};
use actix_web::cookie::time::OffsetDateTime;
use actix_web::cookie::SameSite;
use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
use actix_web::http::{header, StatusCode};
use actix_web::{get, post, web, HttpRequest, HttpResponse, HttpResponseBuilder, ResponseError};
use chrono::Utc;
use rauthy_api_types::oidc::{
    AuthRequest, DeviceAcceptedRequest, DeviceCodeResponse, DeviceGrantRequest,
    DeviceVerifyRequest, DeviceVerifyResponse, JWKSCerts, JWKSPublicKeyCerts, LoginRefreshRequest,
    LoginRequest, LogoutRequest, OAuth2ErrorResponse, OAuth2ErrorTypeResponse, SessionInfoResponse,
    TokenRequest, TokenValidationRequest,
};
use rauthy_api_types::sessions::SessionState;
use rauthy_common::constants::{
    APPLICATION_JSON, AUTH_HEADERS_ENABLE, AUTH_HEADER_EMAIL, AUTH_HEADER_EMAIL_VERIFIED,
    AUTH_HEADER_FAMILY_NAME, AUTH_HEADER_GIVEN_NAME, AUTH_HEADER_GROUPS, AUTH_HEADER_MFA,
    AUTH_HEADER_ROLES, AUTH_HEADER_USER, COOKIE_MFA, COOKIE_SESSION_FED_CM,
    DEVICE_GRANT_CODE_LIFETIME, DEVICE_GRANT_POLL_INTERVAL, DEVICE_GRANT_RATE_LIMIT,
    EXPERIMENTAL_FED_CM_ENABLE, GRANT_TYPE_DEVICE_CODE, HEADER_HTML, HEADER_RETRY_NOT_BEFORE,
    OPEN_USER_REG, SESSION_LIFETIME,
};
use rauthy_common::utils::real_ip_from_req;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::api_cookie::ApiCookie;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::entity::auth_providers::AuthProviderTemplate;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::entity::devices::DeviceAuthCode;
use rauthy_models::entity::fed_cm::FedCMLoginStatus;
use rauthy_models::entity::ip_rate_limit::DeviceIpRateLimit;
use rauthy_models::entity::jwk::{JWKSPublicKey, JwkKeyPair, JWKS};
use rauthy_models::entity::pow::PowEntity;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::users::User;
use rauthy_models::entity::webauthn::WebauthnCookie;
use rauthy_models::entity::well_known::WellKnown;
use rauthy_models::language::Language;
use rauthy_models::templates::{
    AuthorizeHtml, CallbackHtml, Error1Html, ErrorHtml, FrontendAction,
};
use rauthy_models::JwtCommonClaims;
use rauthy_service::oidc::{authorize, logout, token_info, userinfo, validation};
use rauthy_service::{login_delay, oidc};
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

    let (client, origin_header) = match validation::validate_auth_req_param(
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
    if let Ok(mfa_cookie) = WebauthnCookie::parse_validate(&ApiCookie::from_req(&req, COOKIE_MFA)) {
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

    // check for `prompt=no-prompt`
    if !force_new_session
        && req_data
            .prompt
            .as_ref()
            .map(|p| p.as_str() == "none")
            .unwrap_or(false)
    {
        let status = StatusCode::UNAUTHORIZED;
        let body = Error1Html::build(&colors, &lang, status, Some("login_required"));
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
    // check if we can re-use a still valid session or need to create a new one
    let session = if let Some(session) = &principal.session {
        match principal.validate_session_auth_or_init() {
            Ok(_) => session.clone(),
            Err(_) => Session::new(*SESSION_LIFETIME, Some(real_ip_from_req(&req)?)),
        }
    } else {
        Session::new(*SESSION_LIFETIME, Some(real_ip_from_req(&req)?))
    };

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

    if *EXPERIMENTAL_FED_CM_ENABLE {
        Ok(HttpResponse::build(StatusCode::OK)
            .cookie(session.client_cookie_fed_cm())
            .cookie(cookie)
            .insert_header(HEADER_HTML)
            .body(body))
    } else {
        Ok(HttpResponse::build(StatusCode::OK)
            .cookie(cookie)
            .insert_header(HEADER_HTML)
            .body(body))
    }
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
    payload: actix_web_validator::Json<LoginRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    debug!("Validating session in auth or init state");
    principal.validate_session_auth_or_init()?;
    debug!("session is in auth or init state");

    // TODO refactor login delay to use Instant, which is a bit cleaner
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let session = principal.get_session()?;

    let mut has_password_been_hashed = false;
    let mut add_login_delay = true;
    let mut user_needs_mfa = false;

    let res = match authorize::post_authorize(
        &data,
        &req,
        payload.into_inner(),
        session.clone(),
        &mut has_password_been_hashed,
        &mut add_login_delay,
        &mut user_needs_mfa,
    )
    .await
    {
        Ok(auth_step) => map_auth_step(auth_step, &req).await,
        Err(err) => {
            debug!("{:?}", err);
            // We always must return the exact same error type, no matter what the actual error is,
            // to prevent information enumeration. The only exception is when the user needs to add
            // a passkey to the account while having given the correct credentials. In that case,
            // we return the original error to be able to display the info message in the UI.
            if user_needs_mfa {
                // in this case, we can return directly without any login delay
                return Err(err);
            }

            let err = Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid user credentials",
            ));
            if !add_login_delay {
                return err;
            }
            err
        }
    };

    let ip = real_ip_from_req(&req)?;
    login_delay::handle_login_delay(&data, ip, start, res, has_password_been_hashed).await
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

    let (client, header_origin) = validation::validate_auth_req_param(
        &data,
        &req,
        &req_data.client_id,
        &req_data.redirect_uri,
        &req_data.code_challenge,
        &req_data.code_challenge_method,
    )
    .await?;

    let auth_step = authorize::post_authorize_refresh(
        &data,
        session,
        client,
        header_origin,
        req_data.into_inner(),
    )
    .await?;
    map_auth_step(auth_step, &req).await
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
            Err(err) => {
                error!("{err}");
                return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::InvalidRequest,
                    error_description: Some(Cow::from(
                        "internal error - cannot extract IP from request",
                    )),
                });
            }
            Ok(ip) => {
                match DeviceIpRateLimit::is_limited(ip.to_string()).await {
                    Ok(dt) => {
                        if let Some(dt) = dt {
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
                    }
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                            error: OAuth2ErrorTypeResponse::InvalidRequest,
                            error_description: Some(Cow::from(
                                "Internal Server Error - Cache Lookup",
                            )),
                        });
                    }
                }

                if let Err(err) = DeviceIpRateLimit::insert(ip.to_string()).await {
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
            error_description: Some(err.message),
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
    let code = match DeviceAuthCode::new(scopes, client.id, payload.client_secret).await {
        Ok(code) => code,
        Err(err) => {
            return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                error: OAuth2ErrorTypeResponse::InvalidRequest,
                error_description: Some(err.message),
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
    payload: actix_web_validator::Json<DeviceVerifyRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let payload = payload.into_inner();
    debug!("{:?}", payload);

    let challenge = Pow::validate(&payload.pow)?;
    PowEntity::check_prevent_reuse(challenge.to_string()).await?;

    let mut device_code = DeviceAuthCode::find(payload.user_code)
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
            device_code.save().await?;
            Ok(HttpResponse::Accepted().finish())
        }
        DeviceAcceptedRequest::Decline => {
            device_code.delete().await?;
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
    let body = match logout::get_logout_html(req_data.into_inner(), session, &data, &lang).await {
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
    let cookie_fed_cm = ApiCookie::build_with_same_site(
        COOKIE_SESSION_FED_CM,
        Cow::from(&session.id),
        0,
        SameSite::None,
    );
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
            .cookie(cookie_fed_cm)
            .finish());
    }

    return Ok(HttpResponse::build(StatusCode::OK)
        .cookie(cookie)
        .cookie(cookie_fed_cm)
        .finish());
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
    path = "/oidc/rotate_jwk",
    tag = "oidc",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[post("/oidc/rotate_jwk")]
pub async fn rotate_jwk(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Secrets, AccessRights::Update)?;

    JWKS::rotate(&data)
        .await
        .map(|_| HttpResponse::Ok().finish())
}

/// Create a new session
///
/// You can use this endpoint to create a new session outside the `/authorize` page when logging
/// in. This endpoint is used inside Rauthy's integration tests internally and may be used, if you
/// want to create your own login or other user facing UI components.
/// This endpoint will return a session in `Init` state. You will need to log in and verify your
/// credentials, but it is a prerequisite for using the `/authorize` endpoint.
///
/// CAUTION: You will never see the returned CSRF token again - save it somewhere safe!
#[utoipa::path(
    post,
    path = "/oidc/session",
    tag = "oidc",
    responses(
        (status = 201, description = "Created"),
    ),
)]
#[post("/oidc/session")]
pub async fn post_session(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    let session = Session::new(*SESSION_LIFETIME, real_ip_from_req(&req).ok());
    session.save(&data).await?;
    let cookie = session.client_cookie();

    let timeout = OffsetDateTime::from_unix_timestamp(session.last_seen)
        .unwrap()
        .add(::time::Duration::seconds(data.session_timeout as i64));
    let info = SessionInfoResponse {
        id: session.id.as_str().into(),
        csrf_token: Some(session.csrf_token.as_str().into()),
        user_id: session.user_id.as_deref().map(|v| v.into()),
        roles: session.roles.as_deref().map(|v| v.into()),
        groups: session.groups.as_deref().map(|v| v.into()),
        exp: OffsetDateTime::from_unix_timestamp(session.exp).unwrap(),
        timeout,
        state: SessionState::from(session.state()?),
    };

    if *EXPERIMENTAL_FED_CM_ENABLE {
        Ok(HttpResponse::Created()
            .cookie(cookie)
            .cookie(session.client_cookie_fed_cm())
            .json(info))
    } else {
        Ok(HttpResponse::Created().cookie(cookie).json(info))
    }
}

/// OIDC sessioninfo
///
/// Returns information about the current session. This is currently only used in the Rauthy Admin UI
/// and does not make much sense anywhere else. Only works with a fully authenticated session.
#[utoipa::path(
    get,
    path = "/oidc/sessioninfo",
    tag = "oidc",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/oidc/sessioninfo")]
pub async fn get_session_info(data: web::Data<AppState>, principal: ReqPrincipal) -> HttpResponse {
    if let Err(err) = principal.validate_session_auth() {
        return HttpResponse::Unauthorized()
            .insert_header(FedCMLoginStatus::LoggedOut.as_header_pair())
            .json(err);
    }
    let session = match principal.get_session() {
        Ok(s) => s,
        Err(err) => {
            return HttpResponse::Unauthorized()
                .insert_header(FedCMLoginStatus::LoggedOut.as_header_pair())
                .json(err);
        }
    };

    let timeout = OffsetDateTime::from_unix_timestamp(session.last_seen)
        .unwrap()
        .add(::time::Duration::seconds(data.session_timeout as i64));
    let info = SessionInfoResponse {
        id: session.id.as_str().into(),
        csrf_token: None,
        user_id: session.user_id.as_deref().map(|v| v.into()),
        roles: session.roles.as_deref().map(|v| v.into()),
        groups: session.groups.as_deref().map(|v| v.into()),
        exp: OffsetDateTime::from_unix_timestamp(session.exp).unwrap(),
        timeout,
        state: SessionState::from(
            session
                .state()
                .unwrap_or(rauthy_models::entity::sessions::SessionState::Unknown),
        ),
    };

    HttpResponse::Ok()
        .insert_header(FedCMLoginStatus::LoggedIn.as_header_pair())
        .json(info)
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
        id: session.id.as_str().into(),
        csrf_token: Some(session.csrf_token.as_str().into()),
        user_id: session.user_id.as_deref().map(|v| v.into()),
        roles: session.roles.as_deref().map(|v| v.into()),
        groups: session.groups.as_deref().map(|v| v.into()),
        exp: OffsetDateTime::from_unix_timestamp(session.exp).unwrap(),
        timeout,
        state: SessionState::from(session.state()?),
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
    let ip = real_ip_from_req(&req)?;

    if payload.grant_type == GRANT_TYPE_DEVICE_CODE {
        // the `urn:ietf:params:oauth:grant-type:device_code` needs
        // a fully customized handling here with customized error response
        // to meet the oauth rfc
        return Ok(oidc::grant_type_device_code(&data, ip, payload.into_inner()).await);
    }

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let has_password_been_hashed = payload.grant_type == "password";

    let res = match oidc::get_token_set(payload.into_inner(), &data, req).await {
        Ok((token_set, headers)) => {
            let mut builder = HttpResponseBuilder::new(StatusCode::OK);
            for h in headers {
                builder.insert_header(h);
            }
            let resp = builder.json(token_set);
            Ok(resp)
        }
        Err(err) => {
            if !has_password_been_hashed {
                return Err(err);
            }
            // TODO return always the same error here as well, just like during authorize?
            Err(err)
        }
    };

    login_delay::handle_login_delay(&data, ip, start, res, has_password_been_hashed).await
}

/// The token introspection endpoint for OAuth2
///
/// By default, this endpoint requires authorization.
/// You can authorize in 2 different ways:
/// 1. `Basic` auth with `client_id:client_secret`
/// 2. `Bearer` JWT token
///
/// If your client application can't provide any, you can disable authorization for this endpoint
/// by setting `DANGER_DISABLE_INTROSPECT_AUTH=true` in the Rauthy config.
/// Only do this, if you know what you are doing and have other ways to prevent public access to
/// this endpoint.
#[utoipa::path(
    post,
    path = "/oidc/introspect",
    tag = "oidc",
    request_body = TokenValidationRequest,
    responses(
        (status = 200, description = "Ok", body = TokenInfo),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/oidc/introspect")]
pub async fn post_token_introspect(
    data: web::Data<AppState>,
    req: HttpRequest,
    req_data: actix_web_validator::Form<TokenValidationRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    match token_info::get_token_info(&data, &req, &req_data.token).await {
        Ok(info) => Ok(HttpResponse::Ok().json(info)),
        Err(err) => {
            error!("{:?}", err);
            Err(err)
        }
    }
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
//     oidc::validate_refresh_token(None, &req_data.refresh_token, &data)
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
    validation::validate_token::<JwtCommonClaims>(&data, &req_data.token)
        .await
        .map(|_| HttpResponse::Accepted().finish())
}

/// The userinfo endpoint for the OIDC standard.
///
/// Depending on the JWT token from the *Authorization* header, it will return information about
/// the requesting user / token.
#[utoipa::path(
    get,
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
    userinfo::get_userinfo(&data, req)
        .await
        .map(|u| HttpResponse::Ok().json(u))
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
#[post("/oidc/userinfo")]
pub async fn post_userinfo(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    userinfo::get_userinfo(&data, req)
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
    let info = userinfo::get_userinfo(&data, req).await?;

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
