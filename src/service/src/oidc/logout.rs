use crate::oidc::bcl_logout_token::LogoutToken;
use crate::oidc::validation;
use actix_web::cookie::SameSite;
use actix_web::http::{StatusCode, header};
use actix_web::{HttpRequest, HttpResponse, web};
use rauthy_api_types::oidc::LogoutRequest;
use rauthy_common::constants::{COOKIE_SESSION, COOKIE_SESSION_FED_CM};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::api_cookie::ApiCookie;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::theme::ThemeCssFull;
use rauthy_models::entity::users::User;
use rauthy_models::html::HtmlCached;
use rauthy_models::{JwtIdClaims, JwtTokenType};
use std::borrow::Cow;

// We will allow more clock skew here for the token expiration validation to not be too
// strict, as long as the signature of the token and all other things are valid.
// This value could be made configurable, but it is probably not really worth it.
static LOGOUT_TOKEN_CLOCK_SKEW: u64 = 600;

/// Returns the Logout HTML Page for [GET /oidc/logout](crate::handlers::get_logout)
pub async fn get_logout_html(
    req: HttpRequest,
    logout_request: LogoutRequest,
    session: Session,
    data: &web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    let theme_ts = ThemeCssFull::find_theme_ts_rauthy().await?;
    if logout_request.id_token_hint.is_none() {
        return HtmlCached::Logout(session.csrf_token)
            .handle(req, theme_ts, false)
            .await;
    }

    // check if the provided token hint is valid
    let token_raw = logout_request.id_token_hint.unwrap();
    let claims =
        validation::validate_token::<JwtIdClaims>(data, &token_raw, Some(LOGOUT_TOKEN_CLOCK_SKEW))
            .await?;

    // check if it is an ID token
    if JwtTokenType::Id != claims.custom.typ {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "The provided token is not an ID token",
        ));
    }

    // from here on, the token_hint contains a valid ID token -> skip the logout confirmation
    if logout_request.post_logout_redirect_uri.is_some() {
        // unwrap is safe since the token is valid already
        let client_id = claims.custom.azp;
        let client = Client::find(client_id).await?;
        if client.post_logout_redirect_uris.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Given 'post_logout_redirect_uri' is not allowed",
            ));
        }

        let target = logout_request.post_logout_redirect_uri.unwrap();
        let uri_vec = client.get_post_logout_uris();
        let valid_redirect = uri_vec.as_ref().unwrap().iter().filter(|uri| {
            if uri.ends_with('*') && target.starts_with(uri.split_once('*').unwrap().0) {
                return true;
            }
            if target.eq(*uri) {
                return true;
            }
            false
        });
        if valid_redirect.count() == 0 {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Given 'post_logout_redirect_uri' is not allowed",
            ));
        }
        // redirect uri is valid at this point
    }

    HtmlCached::Logout(session.csrf_token)
        .handle(req, theme_ts, false)
        .await
}

#[inline]
pub async fn post_logout_handle(
    data: web::Data<AppState>,
    params: LogoutRequest,
    session: Option<Session>,
) -> Result<HttpResponse, ErrorResponse> {
    // TODO better detection for backchannel logout - could also be rp initiated only
    // without a logout token
    let is_backchannel = params.logout_token.is_some();

    let (session, user, post_logout_redirect_uri) =
        if let Some(id_token_hint) = params.id_token_hint {
            let claims = validation::validate_token::<JwtIdClaims>(
                &data,
                &id_token_hint,
                Some(LOGOUT_TOKEN_CLOCK_SKEW),
            )
            .await?;

            let post_logout_redirect_uri = if let Some(uri) = params.post_logout_redirect_uri {
                let client = Client::find(claims.custom.azp).await?;
                client.validate_post_logout_redirect_uri(&uri)?;
                Some(uri)
            } else {
                None
            };

            if let Some(sid) = claims.custom.sid {
                (
                    Some(Session::find(sid).await?),
                    None,
                    post_logout_redirect_uri,
                )
            } else if let Some(uid) = claims.subject {
                (None, Some(User::find(uid).await?), post_logout_redirect_uri)
            } else {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "invalid `id_token`",
                ));
            }
        } else if let Some(s) = session {
            (Some(s), None, None)
        } else if let Some(token) = params.logout_token {
            let lt = LogoutToken::from_str_validated(&token).await?;
            if let Some(sid) = lt.sid {
                (Some(Session::find(sid).await?), None, None)
            } else if let Some(uid) = lt.sub {
                (None, Some(User::find(uid).await?), None)
            } else {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "invalid `logout_token`",
                ));
            }
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "need at least one of `id_token_hint`, `logout_token`, or a valid session",
            ));
        };

    let loc = if let Some(uri) = post_logout_redirect_uri {
        let state = if let Some(st) = params.state {
            Cow::from(format!("?state={}", st))
        } else {
            Cow::from("")
        };
        Some(format!("{}{}", uri, state))
    } else if is_backchannel {
        None
    } else {
        Some(data.issuer.to_string())
    };

    if let Some(user) = user {
        // TODO backchannel logout for whole user
    }

    if let Some(session) = session {
        let cookie_fed_cm = ApiCookie::build_with_same_site(
            COOKIE_SESSION_FED_CM,
            Cow::from(&session.id),
            0,
            SameSite::None,
        );
        let sid = session.id.clone();
        let cookie_session = ApiCookie::build(COOKIE_SESSION, &sid, 0);
        session.invalidate().await?;

        // TODO backchannel logout for session only

        if let Some(loc) = loc {
            Ok(HttpResponse::build(StatusCode::OK)
                .append_header((header::LOCATION, loc))
                .cookie(cookie_session)
                .cookie(cookie_fed_cm)
                .finish())
        } else {
            Ok(HttpResponse::build(StatusCode::OK)
                .cookie(cookie_session)
                .cookie(cookie_fed_cm)
                .finish())
        }
    } else if let Some(loc) = loc {
        Ok(HttpResponse::build(StatusCode::OK)
            .append_header((header::LOCATION, loc))
            .finish())
    } else {
        Ok(HttpResponse::build(StatusCode::OK).finish())
    }
}
