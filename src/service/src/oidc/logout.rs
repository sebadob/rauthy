use crate::oidc::bcl_logout_token::LogoutToken;
use crate::oidc::validation;
use actix_web::cookie::SameSite;
use actix_web::http::header::ACCEPT;
use actix_web::http::{StatusCode, header};
use actix_web::{HttpRequest, HttpResponse, web};
use rauthy_api_types::oidc::{BackchannelLogoutRequest, LogoutRequest};
use rauthy_common::constants::{
    APPLICATION_JSON, COOKIE_SESSION, COOKIE_SESSION_FED_CM, RAUTHY_VERSION,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::api_cookie::ApiCookie;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::failed_backchannel_logout::FailedBackchannelLogout;
use rauthy_models::entity::jwk::{JwkKeyPair, JwkKeyPairAlg};
use rauthy_models::entity::refresh_tokens::RefreshToken;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::theme::ThemeCssFull;
use rauthy_models::entity::user_login_states::UserLoginState;
use rauthy_models::entity::users::User;
use rauthy_models::html::HtmlCached;
use rauthy_models::{JwtIdClaims, JwtTokenType};
use std::borrow::Cow;
use std::collections::HashSet;
use std::env;
use std::str::FromStr;
use std::string::ToString;
use std::sync::LazyLock;
use std::time::Duration;
use tokio::task::JoinSet;
use tracing::error;

static BACKCHANNEL_DANGER_ALLOW_HTTP: LazyLock<bool> = LazyLock::new(|| {
    env::var("BACKCHANNEL_DANGER_ALLOW_HTTP")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .expect("Cannot parse BACKCHANNEL_DANGER_ALLOW_HTTP as bool")
});
static BACKCHANNEL_DANGER_ALLOW_INSECURE: LazyLock<bool> = LazyLock::new(|| {
    env::var("BACKCHANNEL_DANGER_ALLOW_INSECURE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .expect("Cannot parse BACKCHANNEL_DANGER_ALLOW_INSECURE as bool")
});

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
        // redirect uri is valid at this point
    }

    HtmlCached::Logout(session.csrf_token)
        .handle(req, theme_ts, false)
        .await
}

#[inline]
pub async fn post_logout_handle(
    req: HttpRequest,
    data: web::Data<AppState>,
    params: LogoutRequest,
    session: Option<Session>,
) -> Result<HttpResponse, ErrorResponse> {
    let is_backchannel = session.is_none()
        || req
            .headers()
            .get(ACCEPT)
            .map(|v| v.to_str().unwrap_or_default() != APPLICATION_JSON)
            .unwrap_or(true);

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

    let sid = session.as_ref().map(|s| s.id.clone());
    if let Some(session) = session {
        let sid = Some(session.id.clone());
        let uid = session.user_id.clone();
        session.invalidate().await?;
        execute_backchannel_logout(&data, sid, uid).await?;
    }

    if let Some(user) = user {
        Session::invalidate_for_user(&user.id).await?;
        RefreshToken::invalidate_for_user(&user.id).await?;
        execute_backchannel_logout(&data, None, Some(user.id)).await?;
    }

    if is_backchannel {
        Ok(HttpResponse::build(StatusCode::OK).finish())
    } else {
        let uri = post_logout_redirect_uri.as_ref().unwrap_or(&data.issuer);
        let state = params
            .state
            .map(|st| format!("?state={}", st))
            .unwrap_or_default();
        let loc = format!("{}{}", uri, state);

        if let Some(sid) = sid {
            let cookie_fed_cm = ApiCookie::build_with_same_site(
                COOKIE_SESSION_FED_CM,
                Cow::from(&sid),
                0,
                SameSite::None,
            );
            let cookie_session = ApiCookie::build(COOKIE_SESSION, &sid, 0);

            Ok(HttpResponse::build(StatusCode::OK)
                .append_header((header::LOCATION, loc))
                .cookie(cookie_session)
                .cookie(cookie_fed_cm)
                .finish())
        } else {
            Ok(HttpResponse::build(StatusCode::OK)
                .append_header((header::LOCATION, loc))
                .finish())
        }
    }
}

/// Executes a backchannel logout for the given user / session.
///
/// Does NOT invalidate or delete any local sessions - only cares about remote clients with
/// configured backchannel logout.
pub async fn execute_backchannel_logout(
    data: &web::Data<AppState>,
    sid: Option<String>,
    uid: Option<String>,
) -> Result<(), ErrorResponse> {
    let (states, sid) = if let Some(sid) = sid {
        let states = UserLoginState::find_by_session(sid.clone()).await?;
        // As a fallback, we will log out the whole user if we cannot find the session, just to
        // be sure we never miss any logout. Better logging out some unindented ones than missing
        // an important one.
        if states.is_empty() {
            if uid.is_none() {
                return Ok(());
            }
            (UserLoginState::find_by_user(uid.unwrap()).await?, None)
        } else {
            (states, Some(sid))
        }
    } else if uid.is_none() {
        return Ok(());
    } else {
        (UserLoginState::find_by_user(uid.unwrap()).await?, None)
    };
    if states.is_empty() {
        return Ok(());
    }

    let uid = states.first().unwrap().user_id.clone();
    let client_ids = states
        .iter()
        .map(|st| st.client_id.as_str())
        .collect::<HashSet<_>>();
    let clients = Client::find_all_bcl(&client_ids).await?;

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

            if let Err(err) =
                send_backchannel_logout(client, data.issuer.clone(), sub, sid, &mut kps, &mut tasks)
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

    // We want to clean up login states even if they were failures.
    // This makes sure that we would not clean up newly created login states later on with retries
    // and only retry the failed ones explicitly.
    if let Some(sid) = sid {
        UserLoginState::delete_all_by_sid(sid).await?;
    } else {
        UserLoginState::delete_all_by_uid(uid).await?;
    }

    Ok(())
}

pub async fn send_backchannel_logout(
    client: Client,
    issuer: String,
    sub: Option<String>,
    sid: Option<String>,
    key_pairs: &mut Vec<JwkKeyPair>,
    tasks: &mut JoinSet<Result<(), ErrorResponse>>,
) -> Result<(), ErrorResponse> {
    let mut kp = key_pairs
        .iter()
        .find(|kp| kp.typ.as_str() == client.id_token_alg);
    if kp.is_none() {
        let alg = JwkKeyPairAlg::from_str(client.id_token_alg.as_str())?;
        key_pairs.push(JwkKeyPair::find_latest(alg).await?);
        kp = key_pairs.last();
    }

    let logout_token = LogoutToken::new(issuer, client.id.clone(), sub.clone(), sid.clone())
        .into_token_with_kp(kp.unwrap())?;

    tasks.spawn(async move {
        let res = http_client()
            .post(client.backchannel_logout_uri.unwrap_or_default())
            .form(&BackchannelLogoutRequest { logout_token })
            .send()
            .await;

        let err = match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    return Ok(());
                }

                let text = resp.text().await.unwrap_or_default();
                format!(
                    "Error during Backchannel Logout for client {}: {}",
                    client.id, text
                )
            }
            Err(err) => {
                format!(
                    "Error during Backchannel Logout for client {}: {}",
                    client.id, err
                )
            }
        };

        FailedBackchannelLogout::upsert(client.id, sub, sid, 0).await?;

        Err(ErrorResponse::new(ErrorResponseType::BadRequest, err))
    });

    Ok(())
}

// TODO most probably move this into a LazyLock to make efficient use of connection pooling
#[inline]
fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(10))
        .https_only(!*BACKCHANNEL_DANGER_ALLOW_HTTP)
        .danger_accept_invalid_certs(*BACKCHANNEL_DANGER_ALLOW_INSECURE)
        .user_agent(format!("Rauthy OIDC Client v{}", RAUTHY_VERSION))
        .build()
        .unwrap()
}
