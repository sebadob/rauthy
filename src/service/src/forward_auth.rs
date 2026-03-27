use actix_web::http::StatusCode;
use actix_web::http::header::LOCATION;
use actix_web::{HttpRequest, HttpResponse};
use chrono::Utc;
use rauthy_api_types::forward_auth::{ForwardAuthCallbackParams, ForwardAuthParams};
use rauthy_common::constants::TRUSTED_PROXIES;
use rauthy_common::utils::real_ip_from_req;
use rauthy_data::entity::auth_codes::AuthCode;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::forward_auth::{ForwardAuthCallbackState, ForwardAuthSession};
use rauthy_data::entity::sessions::{Session, SessionState};
use rauthy_data::entity::users::User;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::net::IpAddr;
use std::str::FromStr;
use tracing::{debug, error};

pub async fn get_forward_auth_client(
    client_id: String,
    req: HttpRequest,
    params: ForwardAuthParams,
) -> Result<HttpResponse, ErrorResponse> {
    validate_proxy(&req)?;

    let proto = get_header("X-Forwarded-Proto", &req)?;
    if !params.danger_cookie_insecure && proto.eq_ignore_ascii_case("http")
        || proto.eq_ignore_ascii_case("ws")
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Non-encrypted requests are forbidden",
        ));
    }

    let host = get_header("X-Forwarded-Host", &req)?;
    let origin = format!("{proto}://{host}");

    let forwarded_uri = get_header_alt("X-Forwarded-URI", "X-Original-URL", &req)?;
    let method = get_header("X-Forwarded-Method", &req)?;
    let check_csrf = !method.eq_ignore_ascii_case("GET") && !method.eq_ignore_ascii_case("HEAD");

    let client = get_client_validated(client_id, &origin).await?;
    client.validate_enabled()?;

    let session = match ForwardAuthSession::from_cookie(&req, params.danger_cookie_insecure).await {
        None => {
            debug!("No session found");
            None
        }
        Some(s) => {
            debug!(fwd_auth_session = ?s);
            if s.validate(&req).is_ok() {
                if check_csrf {
                    match s.validate_csrf(&req, params.danger_cookie_insecure) {
                        Ok(_) => Some(s),
                        Err(err) => {
                            error!(?err, "CSRF token check");
                            None
                        }
                    }
                } else {
                    Some(s)
                }
            } else {
                None
            }
        }
    };

    if session.is_none() {
        let redirect_uris = client.get_redirect_uris();
        let redirect_uri = match redirect_uris.first() {
            None => {
                debug!("The first configured `redirect_uri` must be absolute for `forward_auth`");
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "The first configured `redirect_uri` must be absolute for `forward_auth`",
                ));
            }
            Some(uri) => {
                if !uri.starts_with(&origin) {
                    debug!(
                        "The first configured `redirect_uri` must start with the `X-Forwarded-Host`"
                    );
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "The first configured `redirect_uri` must start with the `X-Forwarded-Host`",
                    ));
                }
                if uri.ends_with("*") {
                    debug!(
                        "The first configured `redirect_uri` must be absolute for `forward_auth`"
                    );
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "The first configured `redirect_uri` must be absolute for `forward_auth`",
                    ));
                }
                uri
            }
        };

        let iss = &RauthyConfig::get().issuer;
        let cid = client.id;
        let state = ForwardAuthCallbackState {
            client_id: cid.clone(),
            peer_ip: real_ip_from_req(&req)?,
            origin,
            forwarded_uri: forwarded_uri.to_string(),
            danger_cookie_insecure: params.danger_cookie_insecure,
        }
        .encrypted_str()?;
        let location = format!(
            "{iss}oidc/authorize?client_id={cid}&redirect_uri={redirect_uri}&response_type=code&state={state}"
        );

        let status =
            StatusCode::from_u16(params.redirect_state).unwrap_or(StatusCode::UNAUTHORIZED);

        debug!("HTTP {status} redirect to: {location}");
        return Ok(HttpResponse::build(status)
            .insert_header((LOCATION, location))
            .finish());
    };
    let mut session = session.unwrap().inner;

    // we don't have the middleware auto-updater for last_seen at this point
    let now = Utc::now().timestamp();
    if session.last_seen < now - 10 {
        session.last_seen = now;
        session.upsert().await?;
    }

    debug!("Checking user and client validity");

    let Some(user_id) = session.user_id.clone() else {
        return Err(ErrorResponse::new(
            ErrorResponseType::Internal,
            "missing user_id in valid session - this should never happen",
        ));
    };
    let user = User::find(user_id).await?;
    user.check_enabled()?;
    user.check_expired()?;
    client.validate_user_groups(&user)?;
    client.validate_mfa(&user, None)?;

    let headers = &RauthyConfig::get().vars.auth_headers;
    if headers.enable {
        let mfa_enabled = user.has_webauthn_enabled();
        Ok(HttpResponse::Ok()
            .insert_header((headers.user.as_ref(), user.id))
            .insert_header((headers.roles.as_ref(), user.roles))
            .insert_header((headers.groups.as_ref(), user.groups.unwrap_or_default()))
            .insert_header((headers.email.as_ref(), user.email))
            .insert_header((
                headers.email_verified.as_ref(),
                user.email_verified.to_string(),
            ))
            .insert_header((
                headers.family_name.as_ref(),
                user.family_name.unwrap_or_default(),
            ))
            .insert_header((headers.given_name.as_ref(), user.given_name))
            .insert_header((headers.mfa.as_ref(), mfa_enabled.to_string()))
            .finish())
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}

pub async fn get_forward_auth_client_callback(
    client_id: String,
    req: HttpRequest,
    params: ForwardAuthCallbackParams,
) -> Result<HttpResponse, ErrorResponse> {
    validate_proxy(&req)?;

    let ip = real_ip_from_req(&req)?;
    let proto = get_header("X-Forwarded-Proto", &req)?;
    let host = get_header("X-Forwarded-Host", &req)?;
    let origin = format!("{proto}://{host}");
    debug!(origin);

    let state = ForwardAuthCallbackState::try_from(params.state.as_str())?;
    debug!(?state);

    if !state.danger_cookie_insecure && proto.eq_ignore_ascii_case("http")
        || proto.eq_ignore_ascii_case("ws")
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Non-encrypted requests are forbidden",
        ));
    }

    if state.client_id != client_id {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Mismatch in client_id",
        ));
    }
    if state.peer_ip != ip {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "State created from different IP",
        ));
    }
    if state.origin != origin {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "State created from different Origin",
        ));
    }

    let client = get_client_validated(client_id, &origin).await?;
    client.validate_enabled()?;

    let Some(auth_code) = AuthCode::find(params.code).await? else {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Invalid auth code or code expired",
        ));
    };
    auth_code.delete().await?;
    if auth_code.client_id != client.id {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Mismatch in AuthCode.client_id",
        ));
    }
    let now = Utc::now().timestamp();
    if auth_code.exp < now {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "AuthCode has expired",
        ));
    }
    let Some(sid) = auth_code.session_id else {
        return Err(ErrorResponse::new(
            ErrorResponseType::Internal,
            "AuthCode does not contain a SessionID when it should",
        ));
    };

    let user = User::find(auth_code.user_id).await?;
    user.check_enabled()?;
    user.check_expired()?;
    client.validate_user_groups(&user)?;
    client.validate_mfa(&user, None)?;

    // all good

    // update session metadata
    let mut session = Session::find(sid).await?;
    if session.state != SessionState::Auth {
        // A Session is only set to `SessionState::Auth` AFTER a successful and complete
        // auth code flow. Because of this, it is possible to get here with an `init` session.
        session.state = SessionState::Auth;
        session.validate_user_expiry(&user)?;
        session.user_id = Some(user.id.clone());
        session.roles = Some(user.roles);
        session.groups = user.groups;
    }
    session.last_seen = Utc::now().timestamp();
    session.upsert().await?;

    let fwd_session = ForwardAuthSession { inner: session };
    let (cookie_session, cookie_csrf) = fwd_session.build_cookies(state.danger_cookie_insecure);

    Ok(HttpResponse::build(StatusCode::from_u16(302).unwrap())
        .insert_header((LOCATION, state.forwarded_uri))
        .cookie(cookie_session)
        .cookie(cookie_csrf)
        .finish())
}

#[inline]
async fn get_client_validated(client_id: String, origin: &str) -> Result<Client, ErrorResponse> {
    let client = Client::find(client_id).await?;
    client.validate_enabled()?;

    match client.get_allowed_origins() {
        None => {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "No `allowed_origin`s configured for this client",
            ));
        }
        Some(origins) => {
            if !origins.iter().any(|o| o == origin) {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!(
                        "The origin '{origin}' is not an `allowed_origin` for this \
                        client"
                    ),
                ));
            }
        }
    };

    if client.challenge.is_some() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "A client used for `forward_auth` must not have PKCE enabled",
        ));
    }

    Ok(client)
}

#[inline]
fn get_header<'a>(key: &str, req: &'a HttpRequest) -> Result<&'a str, ErrorResponse> {
    match req.headers().get(key) {
        None => {
            debug!("Missing header {key}");
            Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Missing header {key}"),
            ))
        }
        Some(v) => Ok(v.to_str().unwrap_or_default()),
    }
}

#[inline]
fn get_header_alt<'a>(
    key_first: &str,
    key_second: &str,
    req: &'a HttpRequest,
) -> Result<&'a str, ErrorResponse> {
    match get_header(key_first, req) {
        Ok(v) => Ok(v),
        Err(_) => match get_header(key_second, req) {
            Ok(v) => Ok(v),
            Err(_) => {
                debug!("Missing at least one of headers {key_first} / {key_second}");
                Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("Missing at least one of headers {key_first} / {key_second}"),
                ))
            }
        },
    }
}

#[inline]
fn validate_proxy(req: &HttpRequest) -> Result<(), ErrorResponse> {
    let mut is_match = false;

    let proxy_ip = match req.connection_info().peer_addr() {
        None => {
            debug!("Cannot extract peer IP from HTTP request");
            return Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                "Cannot extract peer IP from HTTP request",
            ));
        }
        Some(s) => match IpAddr::from_str(s) {
            Ok(ip) => ip,
            Err(err) => {
                debug!(?err, "Cannot parse peer IP from HTTP request");
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Cannot parse peer IP from HTTP request",
                ));
            }
        },
    };

    for cidr in TRUSTED_PROXIES.get().unwrap() {
        if cidr.contains(&proxy_ip) {
            is_match = true;
            break;
        }
    }

    if is_match {
        Ok(())
    } else {
        debug!("Invalid proxy peer IP");
        Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Invalid proxy peer IP",
        ))
    }
}
