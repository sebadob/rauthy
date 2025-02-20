use crate::{AcceptEncoding, ReqPrincipal};
use actix_web::body::BoxBody;
use actix_web::http::header::{CACHE_CONTROL, CONTENT_ENCODING, CONTENT_TYPE, ETAG};
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use rauthy_api_types::themes::ThemeRequestResponse;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::theme::ThemeCssFull;

/// Get the theme for the given `client_id`.
///
/// Returns the Rauthy default, if no custom theme exists.
#[utoipa::path(
    get,
    path = "/theme/{client_id}",
    tag = "clients",
    responses(
        (status = 200, description = "Ok", body = String),
    ),
)]
#[get("/theme/{client_id}")]
pub async fn get_theme(
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    let client_id = path.into_inner();
    let etag = ThemeCssFull::etag(&client_id).await?;

    if let Some(etag_remote) = req.headers().get("if-none-match") {
        let etag_remote = etag_remote.to_str().unwrap_or_default();
        if etag_remote == etag {
            return Ok(HttpResponse::new(StatusCode::from_u16(304).unwrap()));
        }
    }

    let enc = AcceptEncoding::from(req.headers());
    let body = match enc {
        AcceptEncoding::Br => BoxBody::new(ThemeCssFull::br(&client_id).await?),
        AcceptEncoding::Gzip => BoxBody::new(ThemeCssFull::gzip(&client_id).await?),
        AcceptEncoding::None => BoxBody::new(ThemeCssFull::plain(client_id).await?),
    };

    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header((CONTENT_TYPE, "text/css"))
        .insert_header((CONTENT_ENCODING, enc.value()))
        .insert_header((
            CACHE_CONTROL,
            "max-age=43200, stale-while-revalidate=2592000, public",
        ))
        .insert_header((ETAG, etag))
        .body(body))
}

/// Get the theme as a JSON opbject for the Admin UI for the given `client_id`.
///
/// Returns the Rauthy default, if no custom theme exists.
#[utoipa::path(
    post,
    path = "/theme/{client_id}",
    tag = "clients",
    responses(
        (status = 200, description = "Ok", body = ThemeRequestResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[post("/theme/{client_id}")]
pub async fn post_theme(
    principal: ReqPrincipal,
    path: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Read)?;

    let theme = ThemeCssFull::find(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ThemeRequestResponse::from(theme)))
}

/// Update the theme for a client
#[utoipa::path(
    get,
    path = "/theme/{client_id}",
    tag = "clients",
    request_body = ThemeRequestResponse,
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[put("/theme/{client_id}")]
pub async fn put_theme(
    principal: ReqPrincipal,
    path: web::Path<String>,
    Json(payload): Json<ThemeRequestResponse>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Update)?;
    payload.validate()?;

    let client_id = path.into_inner();
    if client_id != payload.client_id {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "mismatch in client_id",
        ));
    }

    // make sure the client exists
    Client::find(client_id).await?;
    ThemeCssFull::upsert(
        payload.client_id,
        payload.light.into(),
        payload.dark.into(),
        payload.border_radius,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

/// Delete the theme for a client
#[utoipa::path(
    get,
    path = "/theme/{client_id}",
    tag = "clients",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[delete("/theme/{client_id}")]
pub async fn delete_theme(
    principal: ReqPrincipal,
    path: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Delete)?;

    ThemeCssFull::delete(path.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}
