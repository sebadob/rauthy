use actix_web::http::header;
use actix_web::http::header::{HeaderValue, CONTENT_TYPE};
use actix_web::{get, web, HttpResponse};
use rauthy_common::constants::{APPLICATION_JSON, EXPERIMENTAL_FED_CM_ENABLE};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::well_known::WellKnown;

/// GET accounts linked to the users
///
/// https://fedidcg.github.io/FedCM/
#[utoipa::path(
    get,
    path = "/fed_cm/accounts",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/fed_cm/accounts")]
pub async fn get_fed_cm_accounts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;

    todo!()
}

/// GET metadata for the FedCM client
///
/// https://fedidcg.github.io/FedCM/
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
/// https://fedidcg.github.io/FedCM/
#[utoipa::path(
    get,
    path = "/fed_cm/config",
    tag = "fed_cm",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/fed_cm/config")]
pub async fn get_fed_cm_config(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;

    todo!()
}

/// POST ID assertion
///
/// https://fedidcg.github.io/FedCM/
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
/// https://fedidcg.github.io/FedCM/
///
/// TODO from the spec, this MUST be at eTLD+1:
/// -> not compatible with OIDC spec (<issuer>/.well-known/...)
/// -> redirects ok?
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
) -> Result<HttpResponse, ErrorResponse> {
    is_fed_cm_enabled()?;

    todo!()
    // let wk = WellKnown::json(&data).await?;
    // Ok(HttpResponse::Ok()
    //     .insert_header((CONTENT_TYPE, APPLICATION_JSON))
    //     .insert_header((
    //         header::ACCESS_CONTROL_ALLOW_ORIGIN,
    //         HeaderValue::from_str("*").unwrap(),
    //     ))
    //     .body(wk))
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
