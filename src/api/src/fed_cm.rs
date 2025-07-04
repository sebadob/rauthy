use actix_web::http::header;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::web::{Form, Query};
use actix_web::{HttpRequest, HttpResponse, get, post};
use chrono::Utc;
use rauthy_api_types::clients::EphemeralClientRequest;
use rauthy_api_types::fed_cm::{FedCMAssertionRequest, FedCMClientMetadataRequest};
use rauthy_common::constants::{COOKIE_SESSION_FED_CM, HEADER_ALLOW_ALL_ORIGINS, HEADER_JSON};
use rauthy_common::utils::real_ip_from_req;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::ListenScheme;
use rauthy_models::api_cookie::ApiCookie;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::fed_cm::{
    FedCMAccount, FedCMAccounts, FedCMClientMetadata, FedCMIdPConfig, FedCMLoginStatus,
    FedCMTokenResponse, WebIdentity,
};
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::users::User;
use rauthy_models::rauthy_config::RauthyConfig;
use rauthy_service::token_set::{AuthCodeFlow, AuthTime, DeviceCodeFlow, TokenNonce, TokenSet};
use tracing::{debug, error, warn};
use validator::Validate;

const HEADER_ALLOW_CREDENTIALS: (&str, &str) = ("access-control-allow-credentials", "true");

/// GET accounts linked to the users
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/accounts",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok", body = FedCMAccounts),
        (status = 400, description = "BadRequest"),
    ),
)]
#[get("/fed_cm/accounts")]
#[tracing::instrument(level = "debug", skip_all)]
pub async fn get_fed_cm_accounts(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;

    let (login_status, user_id) = login_status_from_req(&req).await;
    if login_status == FedCMLoginStatus::LoggedOut {
        return Ok(HttpResponse::Unauthorized()
            .insert_header(FedCMLoginStatus::LoggedOut.as_header_pair())
            .json(FedCMAccounts {
                accounts: Vec::default(),
            }));
    }

    let user = User::find_for_fed_cm_validated(user_id).await?;
    let account = FedCMAccount::build(user);
    let accounts = FedCMAccounts {
        accounts: vec![account],
    };
    Ok(HttpResponse::Ok()
        .insert_header(FedCMLoginStatus::LoggedIn.as_header_pair())
        .json(accounts))
}

/// GET metadata for the FedCM client
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/client_meta",
    tag = "fed_cm",
    params(FedCMClientMetadataRequest),
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/fed_cm/client_meta")]
#[tracing::instrument(level = "debug", skip_all, fields(client_id = params.client_id))]
pub async fn get_fed_cm_client_meta(
    req: HttpRequest,
    Query(params): Query<FedCMClientMetadataRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;
    params.validate()?;

    if &params.client_id == "rauthy" {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-forbidden".to_string()),
            "The 'rauthy' client is forbidden to be used with FedCM",
        ));
    }

    let client = Client::find_maybe_ephemeral(params.client_id).await?;
    if !client.enabled {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-disabled".to_string()),
            "This client has been disabled",
        ));
    }
    let origin_header = client_origin_header(&req, &client)?;

    let meta = FedCMClientMetadata::new();
    Ok(HttpResponse::Ok()
        .insert_header(HEADER_ALLOW_CREDENTIALS)
        .insert_header(origin_header)
        .json(meta))
}

/// The FedCM IdP configuration
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/config",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok", body = FedCMIdPConfig),
        (status = 400, description = "BadRequest"),
    ),
)]
#[get("/fed_cm/config")]
#[tracing::instrument(level = "debug", skip_all)]
pub async fn get_fed_cm_config(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;

    let config = FedCMIdPConfig::get().await?;
    Ok(HttpResponse::Ok()
        .insert_header(HEADER_JSON)
        .insert_header(HEADER_ALLOW_ALL_ORIGINS)
        .json(config))
}

// /// Disconnect an account
// ///
// /// https://fedidcg.github.io/FedCM/#idp-api
// #[utoipa::path(
//     get,
//     path = "/fed_cm/disconnect",
//     tag = "fed_cm",
//     responses(
//         (status = 200, description = "Ok"),
//     ),
// )]
// #[get("/fed_cm/disconnect")]
// pub async fn post_fed_cm_disconnect(
//     _data: web::Data<AppState>,
// ) -> Result<HttpResponse, ErrorResponse> {
//     is_fed_cm_enabled()?;
//
//     // TODO like it is defined in the spec now, the disconnect endpoint does not really work with
//     // revoking OIDC refresh tokens, since we only get a `client_id` and an `account_id`, but for
//     // these 2 we don't keep any persistent links. Something like an `id_token` hint would be helpful.
//
//     Ok(HttpResponse::Ok().finish())
// }

/// Just a sample ephemeral client config for FedCM testing
#[tracing::instrument(level = "debug", skip_all)]
#[get("/fed_cm/client_config")]
pub async fn get_fed_client_config() -> HttpResponse {
    let pub_url_scheme = &RauthyConfig::get().pub_url_with_scheme;
    let config = EphemeralClientRequest {
        client_id: format!("{pub_url_scheme}/auth/v1/fed_cm/client_config"),
        client_name: Some("Rauthy".to_string()),
        client_uri: Some(pub_url_scheme.clone()),
        contacts: RauthyConfig::get()
            .vars
            .email
            .rauthy_admin_email
            .clone()
            .map(|e| vec![e]),
        redirect_uris: vec![format!("{pub_url_scheme}/auth/v1/*")],
        post_logout_redirect_uris: Some(vec![format!("{pub_url_scheme}/auth/v1/*")]),
        grant_types: Some(vec![
            "authorization_code".to_string(),
            "refresh_token".to_string(),
        ]),
        default_max_age: Some(300),
        scope: Some("openid email profile".to_string()),
        require_auth_time: Some(true),
        access_token_signed_response_alg: Some(rauthy_api_types::oidc::JwkKeyPairAlg::EdDSA),
        id_token_signed_response_alg: Some(rauthy_api_types::oidc::JwkKeyPairAlg::EdDSA),
    };

    HttpResponse::Ok().json(config)
}

/// GET the current FedCM login state for the user
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/status",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[tracing::instrument(level = "debug", skip_all)]
#[get("/fed_cm/status")]
pub async fn get_fed_cm_status(req: HttpRequest) -> HttpResponse {
    if is_fed_cm_enabled().is_err() {
        HttpResponse::Unauthorized()
            .insert_header(FedCMLoginStatus::LoggedOut.as_header_pair())
            .finish()
    } else {
        let (login_status, _) = login_status_from_req(&req).await;
        if login_status == FedCMLoginStatus::LoggedOut {
            HttpResponse::Unauthorized()
                .insert_header(FedCMLoginStatus::LoggedOut.as_header_pair())
                .finish()
        } else {
            HttpResponse::Ok()
                .insert_header(FedCMLoginStatus::LoggedIn.as_header_pair())
                .finish()
        }
    }
}

/// POST ID assertion
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    post,
    path = "/fed_cm/token",
    tag = "fed_cm",
    request_body(
        content = FedCMAssertionRequest,
        content_type = "application/x-www-form-urlencoded"
    ),
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[post("/fed_cm/token")]
#[tracing::instrument(level = "debug", skip_all, fields(client_id = payload.client_id))]
pub async fn post_fed_cm_token(
    req: HttpRequest,
    Form(payload): Form<FedCMAssertionRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;
    payload.validate()?;

    let (login_status, user_id) = login_status_from_req(&req).await;
    if login_status == FedCMLoginStatus::LoggedOut {
        return Ok(HttpResponse::Unauthorized()
            .insert_header(FedCMLoginStatus::LoggedOut.as_header_pair())
            .finish());
    }

    // find and check the client
    let client = match Client::find_maybe_ephemeral(payload.client_id).await {
        Ok(c) => c,
        Err(err) => {
            error!(?err, "looking up maybe ephemeral client");
            return Err(err);
        }
    };
    if !client.enabled {
        debug!("client {} is disabled", client.id);
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("client-disabled".to_string()),
            "This client has been disabled",
        ));
    }

    // TODO what about confidential clients? Should we maybe return an auth_code?
    //  impl a new `FedCM` flow for client's and reject if not true?

    let origin_header = client_origin_header(&req, &client)?;
    debug!("built origin header for client: {:?}", origin_header.1);

    // find and check the user
    let user = User::find_for_fed_cm_validated(user_id).await?;
    if payload.account_id != user.id {
        debug!(
            "payload.account_id != user.id -> {} != {}",
            payload.account_id, user.id
        );
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("invalid-user".to_string()),
            "The `account_id` does not match the `user_id` from the active session",
        ));
    }

    // We are good - issue a TokenSet
    let ts = TokenSet::from_user(
        &user,
        &client,
        AuthTime::given(user.last_login.unwrap_or_else(|| Utc::now().timestamp())),
        None,
        payload.nonce.map(TokenNonce),
        None,
        None,
        AuthCodeFlow::No,
        DeviceCodeFlow::No,
    )
    .await?;

    Ok(HttpResponse::Ok()
        .insert_header(HEADER_ALLOW_CREDENTIALS)
        .insert_header(origin_header)
        .insert_header(FedCMLoginStatus::LoggedIn.as_header_pair())
        .json(FedCMTokenResponse {
            token: ts.id_token.unwrap(),
        }))
}

/// The `.well-known` endpoint for FedCM clients
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/.well-known/web-identity",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok", body = WebIdentity),
    ),
)]
#[get("/.well-known/web-identity")]
pub async fn get_fed_cm_well_known(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;

    Ok(HttpResponse::Ok()
        .insert_header(HEADER_JSON)
        .insert_header(HEADER_ALLOW_ALL_ORIGINS)
        .json(WebIdentity::default()))
}

#[inline(always)]
fn is_fed_cm_enabled() -> Result<(), ErrorResponse> {
    if RauthyConfig::get().vars.fedcm.experimental_enable {
        Ok(())
    } else {
        error!("EXPERIMENTAL_FED_CM_ENABLE is not set");
        Err(ErrorResponse::new(
            ErrorResponseType::Internal,
            "The FedCM API is disabled on this instance",
        ))
    }
}

/// Checks for `Sec-Fetch-Dest: webidentity`
/// Ignores validation of empty origin and referrer headers - not our job
#[inline(always)]
fn is_web_identity_fetch(req: &HttpRequest) -> Result<(), ErrorResponse> {
    if req
        .headers()
        .get("sec-fetch-dest")
        .map(|v| v.to_str().unwrap_or_default())
        == Some("webidentity")
    {
        Ok(())
    } else {
        warn!("`Sec-Fetch-Dest: webidentity` not set`");
        Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Expected header `Sec-Fetch-Dest: webidentity`",
        ))
    }
}

fn client_origin_header(
    req: &HttpRequest,
    client: &Client,
) -> Result<(HeaderName, HeaderValue), ErrorResponse> {
    let origin = req
        .headers()
        .get(header::ORIGIN)
        .map(|v| v.to_str().unwrap_or_default())
        .ok_or_else(|| {
            debug!("Origin header is missing");
            ErrorResponse::new(ErrorResponseType::BadRequest, "Origin header is missing")
        })?;
    debug!("Origin header from request: {}", origin);
    let header = (
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_str(origin)?,
    );

    if client.is_ephemeral() && client.id.starts_with(origin) {
        debug!("The client is ephemeral and its ID matches the origin");
        return Ok(header);
    }

    if let Some(allowed_origins) = &client.allowed_origins {
        for ao in allowed_origins.split(',') {
            debug!("Comparing Allowed Origin '{ao}' to origin '{origin}'");
            if (RauthyConfig::get().listen_scheme == ListenScheme::HttpHttps
                && ao.ends_with(origin))
                || ao.eq(origin)
            {
                debug!("Found matching allowed_origin '{ao}' for origin: '{origin}'");
                return Ok(header);
            }
        }
    }

    // in case we did not have a specific allowed origin, we can validate via allowed
    // `redirect_uri`s
    for uri in client.redirect_uris.split(',') {
        if uri.starts_with(origin) {
            debug!("Found matching redirect_uri '{uri}' for origin: '{origin}'");
            return Ok(header);
        }
    }

    debug!("No match found for allowed origin");
    Err(ErrorResponse::new(
        ErrorResponseType::Forbidden,
        "The origin is not allowed for this client",
    ))
}

#[inline(always)]
async fn login_status_from_req(req: &HttpRequest) -> (FedCMLoginStatus, String) {
    match ApiCookie::from_req(req, COOKIE_SESSION_FED_CM) {
        None => {
            debug!("FedCM session cookie not found -> user_id is logged-out",);
            (FedCMLoginStatus::LoggedOut, String::default())
        }
        Some(sid) => {
            let session = match Session::find(sid).await {
                Ok(s) => s,
                Err(_) => {
                    debug!("FedCM session not found -> user_id is logged-out",);
                    return (FedCMLoginStatus::LoggedOut, String::default());
                }
            };

            if session.is_valid(
                RauthyConfig::get().vars.fedcm.session_timeout,
                real_ip_from_req(req).ok(),
                req.path(),
            ) {
                (
                    FedCMLoginStatus::LoggedIn,
                    session.user_id.unwrap_or_default(),
                )
            } else {
                debug!(
                    "FedCM session is invalid -> user_id {:?} is logged-out",
                    session.user_id
                );
                (
                    FedCMLoginStatus::LoggedOut,
                    session.user_id.unwrap_or_default(),
                )
            }
        }
    }
}
