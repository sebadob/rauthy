use actix_web::http::header;
use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
use actix_web::{get, web, HttpRequest, HttpResponse};
use rauthy_common::constants::{
    APPLICATION_JSON, COOKIE_FED_CM, EXPERIMENTAL_FED_CM_ENABLE, HEADER_ALLOW_ALL_ORIGINS,
    HEADER_JSON,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::api_cookie::ApiCookie;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::fed_cm::{FedCMAccounts, FedCMIdPConfig, WebIdentity};
use rauthy_models::entity::well_known::WellKnown;

/// GET accounts linked to the users
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/accounts",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/fed_cm/accounts")]
pub async fn get_fed_cm_accounts(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;

    // The FedCM endpoints use a special cookie instead of the usual session.
    // This is important to be able to recognize a user for a long period of time even whe
    // there is no active / valid session right now. The Rauthy session cookie though has a very
    // short lifetime, which is why it can't be re-used.
    let user_id = ApiCookie::from_req(&req, COOKIE_FED_CM).ok_or_else(|| {
        // CAUTION: The spec does not have any clarification on how to behave if the user
        // is not logged in or does not exist
        // -> we assume and use the OAuth2 WWW-Authenticate header for now
        // https://github.com/fedidcg/FedCM/issues/218
        ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("user-not-found".to_string()),
            "No user found by FedCM cookie".to_string(),
        )
    })?;

    let accounts = FedCMAccounts::try_for_user_id(&data, user_id).await?;
    Ok(HttpResponse::Ok().json(accounts))
}

/// GET metadata for the FedCM client
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/client_meta",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/fed_cm/client_meta")]
pub async fn get_fed_cm_client_meta(
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;

    todo!()
}

/// The FedCM IdP configuration
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/config.json",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/fed_cm/config.json")]
pub async fn get_fed_cm_config(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;

    let config = FedCMIdPConfig::get(&data).await?;
    Ok(HttpResponse::Ok()
        .insert_header(HEADER_JSON)
        .insert_header(HEADER_ALLOW_ALL_ORIGINS)
        .json(config))
}

/// Disconnect an account
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/disconnect",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/fed_cm/disconnect")]
pub async fn post_fed_cm_disconnect(
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;

    todo!()
}

/// POST ID assertion
///
/// https://fedidcg.github.io/FedCM/#idp-api
#[utoipa::path(
    get,
    path = "/fed_cm/token",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/fed_cm/token")]
pub async fn post_fed_cm_token(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;

    todo!()
}

/// The `.well-known` endpoint for FedCM clients
///
/// https://fedidcg.github.io/FedCM/#idp-api
///
/// TODO from the spec, this MUST be at eTLD+1:
/// -> not compatible with OIDC spec (<issuer>/.well-known/...)
/// -> mention in docs - we must break out of `/auth` sub path
#[utoipa::path(
    get,
    path = "/.well-known/web-identity",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/.well-known/web-identity")]
pub async fn get_fed_cm_well_known(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;
    is_web_identity_fetch(&req)?;

    Ok(HttpResponse::Ok()
        .insert_header(HEADER_JSON)
        .insert_header(HEADER_ALLOW_ALL_ORIGINS)
        .json(WebIdentity::new(&data.issuer)))
}

#[inline(always)]
fn is_fed_cm_enabled() -> Result<(), ErrorResponse> {
    if *EXPERIMENTAL_FED_CM_ENABLE {
        Ok(())
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::Internal,
            "The FedCM API is disabled on this instance".to_string(),
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
        Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Expected header `Sec-Fetch-Dest: webidentity`".to_string(),
        ))
    }
}
