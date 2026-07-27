use crate::oidc::bcl_logout_token::LogoutToken;
use actix_web::cookie::SameSite;
use actix_web::http::header::{ACCESS_CONTROL_ALLOW_METHODS, HeaderValue};
use actix_web::http::{StatusCode, header};
use actix_web::{HttpRequest, HttpResponse};
use rauthy_api_types::oidc::{BackchannelLogoutRequest, LogoutRequest};
use rauthy_common::constants::{COOKIE_SESSION, COOKIE_SESSION_FED_CM};
use rauthy_common::http_client;
use rauthy_data::api_cookie::ApiCookie;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::failed_backchannel_logout::FailedBackchannelLogout;
use rauthy_data::entity::issued_tokens::IssuedToken;
use rauthy_data::entity::jwk::{JwkKeyPair, JwkKeyPairAlg};
use rauthy_data::entity::refresh_tokens::RefreshToken;
use rauthy_data::entity::sessions::Session;
use rauthy_data::entity::theme::ThemeCssFull;
use rauthy_data::entity::user_login_states::UserLoginState;
use rauthy_data::entity::users::User;
use rauthy_data::html::HtmlCached;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_jwt::claims::{JwtIdClaims, JwtTokenType};
use rauthy_jwt::token::JwtToken;
use std::borrow::Cow;
use std::str::FromStr;
use std::string::ToString;
use tokio::task::JoinSet;
use tracing::{debug, error, info};

// We will allow more clock skew here for the token expiration validation to not be too
// strict, as long as the signature of the token and all other things are valid.
// This value could be made configurable, but it is probably not really worth it.
static LOGOUT_TOKEN_CLOCK_SKEW: u16 = 600;

/// Returns the Logout HTML Page for [GET /oidc/logout](crate::handlers::get_logout)
pub async fn get_logout_html(
    req: HttpRequest,
    logout_request: LogoutRequest,
    session: Session,
) -> Result<HttpResponse, ErrorResponse> {
    let theme_ts = ThemeCssFull::find_theme_ts_rauthy().await?;
    if logout_request.id_token_hint.is_none() {
        return HtmlCached::Logout(session.csrf_token)
            .handle(req, theme_ts, false)
            .await;
    }

    // check if the provided token hint is valid
    let token_raw = logout_request.id_token_hint.unwrap();
    let mut buf = Vec::with_capacity(512);
    JwtToken::validate_claims_into(
        &token_raw,
        Some(JwtTokenType::Id),
        LOGOUT_TOKEN_CLOCK_SKEW,
        &mut buf,
    )
    .await?;
    let claims = serde_json::from_slice::<JwtIdClaims>(&buf)?;

    // from here on, the token_hint contains a valid ID token -> skip the logout confirmation
    if let Some(target) = logout_request.post_logout_redirect_uri {
        // unwrap is safe since the token is valid already
        let client = Client::find(claims.common.azp.to_string()).await?;
        if client.post_logout_redirect_uris.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Given 'post_logout_redirect_uri' is not allowed",
            ));
        }

        let uri_vec = client.get_post_logout_uris();

        let valid_redirect = uri_vec.as_ref().unwrap().iter().any(|uri| {
            if uri.ends_with('*') && target.starts_with(uri.split_once('*').unwrap().0) {
                return true;
            }
            if target.eq(uri) {
                return true;
            }
            false
        });
        if valid_redirect {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Given 'post_logout_redirect_uri' is not allowed",
            ));
        }
    }

    HtmlCached::Logout(session.csrf_token)
        .handle(req, theme_ts, false)
        .await
}

#[inline]
pub async fn post_logout_handle(
    req: HttpRequest,
    params: LogoutRequest,
    session: Option<Session>,
) -> Result<HttpResponse, ErrorResponse> {
    let is_backchannel = params.logout_token.is_some()
        // should always exist in even barely modern browsers
        || req.headers().get("sec-fetch-site").is_none();

    let mut buf = Vec::with_capacity(512);
    let (session, user, cors_header, post_logout_redirect_uri) =
        if let Some(id_token_hint) = &params.id_token_hint {
            JwtToken::validate_claims_into(
                id_token_hint,
                Some(JwtTokenType::Id),
                LOGOUT_TOKEN_CLOCK_SKEW,
                &mut buf,
            )
            .await?;
            let claims = serde_json::from_slice::<JwtIdClaims>(&buf)?;
            let client = Client::find(claims.common.azp.to_string()).await?;
            let cors_header = client.get_validated_origin_header(&req)?;

            let post_logout_redirect_uri = if let Some(uri) = params.post_logout_redirect_uri {
                client.validate_post_logout_redirect_uri(&uri)?;
                Some(uri)
            } else {
                None
            };

            let (session, user) = find_session_with_user_fallback(
                claims.sid.map(String::from),
                claims.common.sub.map(String::from),
            )
            .await?;
            (session, user, cors_header, post_logout_redirect_uri)
        } else if let Some(s) = session {
            let (session, user) = find_session_with_user_fallback(Some(s.id), s.user_id).await?;
            (session, user, None, None)
        } else if let Some(token) = params.logout_token {
            let lt = LogoutToken::from_str_validated(&token, &mut buf).await?;
            let (session, user) =
                find_session_with_user_fallback(lt.sid.map(String::from), lt.sub.map(String::from))
                    .await?;
            (session, user, None, None)
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "need at least one of `id_token_hint`, `logout_token`, or a valid session",
            ));
        };

    let token_revoke = RauthyConfig::get().vars.access.token_revoke_on_logout;

    let sid = session.as_ref().map(|s| s.id.clone());
    if let Some(session) = session {
        let uid = session.user_id.clone();
        if token_revoke {
            RefreshToken::delete_by_sid(session.id.clone()).await?;
            IssuedToken::revoke_for_session(
                &session.id,
                RauthyConfig::get().vars.access.token_revoke_device_tokens,
            )
            .await?;
        }
        session.delete().await?;
        execute_backchannel_logout(sid.clone(), uid).await?;
    }

    if let Some(user) = user {
        if token_revoke {
            RefreshToken::invalidate_for_user(&user.id).await?;
            IssuedToken::revoke_for_user(
                &user.id,
                RauthyConfig::get().vars.access.token_revoke_device_tokens,
            )
            .await?;
        }
        Session::invalidate_for_user(&user.id).await?;
        execute_backchannel_logout(None, Some(user.id)).await?;
    }

    if is_backchannel {
        Ok(HttpResponse::build(StatusCode::OK).finish())
    } else {
        let uri = post_logout_redirect_uri
            .as_ref()
            .unwrap_or(&RauthyConfig::get().issuer);
        let state = params
            .state
            .map(|st| format!("?state={st}"))
            .unwrap_or_default();
        let loc = format!("{uri}{state}");

        let mut resp = HttpResponse::build(StatusCode::from_u16(302).unwrap())
            .append_header((header::LOCATION, loc))
            .finish();

        if let Some(sid) = sid {
            let cookie_session = ApiCookie::build(COOKIE_SESSION, &sid, 0);
            resp.add_cookie(&cookie_session)?;

            if RauthyConfig::get().vars.fedcm.experimental_enable {
                let cookie_fed_cm = ApiCookie::build_with_same_site(
                    COOKIE_SESSION_FED_CM,
                    Cow::from(&sid),
                    0,
                    SameSite::None,
                );
                resp.add_cookie(&cookie_fed_cm)?;
            }
        }

        if let Some((n, v)) = cors_header {
            resp.headers_mut().insert(n, v);
            resp.headers_mut().insert(
                ACCESS_CONTROL_ALLOW_METHODS,
                HeaderValue::from_static("POST"),
            );
        }

        Ok(resp)
    }
}

async fn find_session_with_user_fallback(
    sid: Option<String>,
    uid: Option<String>,
) -> Result<(Option<Session>, Option<User>), ErrorResponse> {
    if let Some(sid) = sid {
        match Session::find(sid).await {
            Ok(s) => Ok((Some(s), None)),
            Err(err) => {
                debug!("Could not find `sid` from LogoutToken: {err}");
                if let Some(uid) = uid {
                    Ok((None, Some(User::find(uid).await?)))
                } else {
                    Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "invalid `logout_token`",
                    ))
                }
            }
        }
    } else if let Some(uid) = uid {
        Ok((None, Some(User::find(uid).await?)))
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "neither Session nor User found",
        ))
    }
}

/// Executes a backchannel logout for the given user / session.
///
/// Does NOT invalidate or delete any local sessions - only cares about remote clients with
/// configured backchannel logout.
///
/// If you provide an `sid`, also provide a `uid` which would be used as fallback, if no login
/// states for the given `sid` could be found.
#[tracing::instrument(level = "debug")]
pub async fn execute_backchannel_logout(
    sid: Option<String>,
    uid: Option<String>,
) -> Result<(), ErrorResponse> {
    debug_assert!(sid.is_some() || uid.is_some());
    info!("Executing backchannel logout for uid {uid:?} / sid {sid:?}");

    let (states, sid) = if let Some(sid) = sid {
        let states = UserLoginState::find_by_session(sid.clone()).await?;
        debug!("Login States for sid {sid}: {states:?}");
        // As a fallback, we will log out the whole user if we cannot find the session, just to
        // be sure we never miss any logout. Better logging out some unindented ones than missing
        // an important one.
        if states.is_empty() {
            debug!("No Login States for sid {sid}");

            if let Some(uid) = uid {
                debug!("Searching Login States by user_id {uid:?}");
                (UserLoginState::find_by_user(uid).await?, None)
            } else {
                debug!("No Login States found for both sid and uid - nothing to do");
                return Ok(());
            }
        } else {
            (states, Some(sid))
        }
    } else if let Some(uid) = uid {
        (UserLoginState::find_by_user(uid).await?, None)
    } else {
        debug!("Both sid and uid are None - nothing to od");
        return Ok(());
    };
    debug!(sid, login_states = ?states);
    if states.is_empty() {
        debug!("no login states found");
        return Ok(());
    }

    let uid = states.first().unwrap().user_id.clone();
    let client_ids = states
        .iter()
        .map(|st| st.client_id.as_str())
        .collect::<Vec<_>>();
    let clients = Client::find_all_bcl(&client_ids).await?;
    debug!(backchannel_logout_clients = ?clients);

    if !clients.is_empty() {
        let mut kps: Vec<JwkKeyPair> = Vec::with_capacity(1);
        let mut tasks = JoinSet::new();

        for client in clients {
            let sub = if sid.is_none() {
                Some(uid.clone())
            } else {
                None
            };
            let sid = sid.clone();
            debug!(sub, sid);

            let mut kp = kps.iter().find(|kp| kp.typ.as_str() == client.id_token_alg);
            if kp.is_none() {
                let alg = JwkKeyPairAlg::from_str(client.id_token_alg.as_str())?;
                kps.push(JwkKeyPair::find_latest(alg).await?);
                kp = kps.last();
            }
            debug_assert!(kp.is_some());

            if let Err(err) = send_backchannel_logout(
                client.id.clone(),
                client.backchannel_logout_uri.unwrap_or_default(),
                sub,
                sid,
                kp.as_ref().unwrap(),
                &mut tasks,
            )
            .await
            {
                error!("Error executing Backchannel Logout: {}", err);
            }
        }

        while let Some(res) = tasks.join_next().await {
            if let Err(err) =
                res.map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, err.to_string()))?
            {
                error!("{err}");
            }
        }
    }

    // We want to clean up login states even if there were failures.
    // This makes sure that we would not clean up newly created login states later on with retries
    // and only retry the failed ones explicitly.
    if let Some(sid) = sid {
        UserLoginState::delete_all_by_sid(sid).await?;
    } else {
        UserLoginState::delete_all_by_uid(uid).await?;
    }

    Ok(())
}

/// Executes a backchannel logout for every user on every client in the background.
#[tracing::instrument(level = "debug")]
pub async fn execute_backchannel_logout_for_everything() -> Result<(), ErrorResponse> {
    let clients = Client::find_all()
        .await?
        .into_iter()
        .filter(|c| c.backchannel_logout_uri.is_some())
        .collect::<Vec<_>>();

    for client in clients {
        execute_backchannel_logout_by_client(&client).await?;
    }

    Ok(())
}

#[tracing::instrument(level = "debug")]
pub async fn execute_backchannel_logout_by_client(client: &Client) -> Result<(), ErrorResponse> {
    let Some(uri) = &client.backchannel_logout_uri else {
        UserLoginState::delete_all_by_cid(client.id.clone()).await?;
        return Ok(());
    };

    info!(
        "Executing full backchannel logout for client '{}' via '{uri}'",
        client.id
    );

    // We don't care about specific sessions here. Everything for this client should be logged out.
    // Skipping sessions and logging out whole users reduces the load.
    let states = UserLoginState::find_by_client_without_session(client.id.clone()).await?;
    debug!("{:?}", states);

    let alg = JwkKeyPairAlg::from_str(client.id_token_alg.as_str())?;
    let kp = JwkKeyPair::find_latest(alg).await?;
    let mut tasks = JoinSet::new();

    for state in states {
        if let Err(err) = send_backchannel_logout(
            client.id.clone(),
            uri.to_string(),
            Some(state.user_id),
            None,
            &kp,
            &mut tasks,
        )
        .await
        {
            error!(?err, "executing Backchannel Logout");
        }
    }

    while let Some(res) = tasks.join_next().await {
        if let Err(err) =
            res.map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, err.to_string()))?
        {
            error!(?err);
        }
    }

    UserLoginState::delete_all_by_cid(client.id.clone()).await?;

    Ok(())
}

pub async fn send_backchannel_logout(
    client_id: String,
    backchannel_logout_uri: String,
    sub: Option<String>,
    sid: Option<String>,
    kp: &JwkKeyPair,
    tasks: &mut JoinSet<Result<(), ErrorResponse>>,
) -> Result<(), ErrorResponse> {
    debug_assert!(sub.is_some() || sid.is_some());

    let logout_token = LogoutToken::new(
        &RauthyConfig::get().issuer,
        &client_id,
        sub.as_deref(),
        sid.as_deref(),
        RauthyConfig::get().vars.backchannel_logout.token_lifetime,
    )
    .into_token_with_kp(kp)?;

    tasks.spawn(async move {
        debug!("Sending backchannel logout to {client_id}: {backchannel_logout_uri}");
        let res = http_client()
            .post(backchannel_logout_uri)
            .form(&BackchannelLogoutRequest { logout_token })
            .send()
            .await;

        let err = match res {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    return Ok(());
                }
                let text = resp.text().await.unwrap_or_default();
                format!(
                    "Error during Backchannel Logout for client '{client_id}': HTTP {} - {text}",
                    status.as_u16()
                )
            }
            Err(err) => {
                format!("Error during Backchannel Logout for client '{client_id}': {err}")
            }
        };

        FailedBackchannelLogout::upsert(client_id, sub, sid).await?;

        Err(ErrorResponse::new(ErrorResponseType::BadRequest, err))
    });

    Ok(())
}
