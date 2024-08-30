use crate::ReqPrincipal;
use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL, CONTENT_TYPE, WWW_AUTHENTICATE,
};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web_lab::__reexports::futures_util::StreamExt;
use rauthy_api_types::clients::{
    ClientResponse, ColorsRequest, DynamicClientRequest, NewClientRequest, UpdateClientRequest,
};
use rauthy_common::constants::{DYN_CLIENT_REG_TOKEN, ENABLE_DYN_CLIENT_REG};
use rauthy_common::utils::real_ip_from_req;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::clients_dyn::ClientDyn;
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::entity::logos::{Logo, LogoType};
use rauthy_service::client;
use rauthy_service::oidc::helpers;
use tracing::debug;

/// Returns all existing OIDC clients with all their information, except for the client secrets.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/clients",
    tag = "clients",
    responses(
        (status = 200, description = "Ok", body = ClientResponse),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "NotFound"),
    ),
)]
#[tracing::instrument(skip_all)]
#[get("/clients")]
pub async fn get_clients(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Read)?;

    let clients = Client::find_all(&data).await?;

    let mut res = Vec::new();
    clients
        .into_iter()
        .for_each(|c| res.push(ClientResponse::from(c)));

    Ok(HttpResponse::Ok().json(res))
}

/// Returns a single OIDC clients by its *id* with all information's, except for the client secret.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/clients/{id}",
    tag = "clients",
    responses(
        (status = 200, description = "Ok", body = ClientResponse),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "NotFound"),
    ),
)]
#[get("/clients/{id}")]
pub async fn get_client_by_id(
    path: web::Path<String>,
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Read)?;

    Client::find(&data, path.into_inner())
        .await
        .map(|c| HttpResponse::Ok().json(ClientResponse::from(c)))
}

/// Returns the secret in cleartext for a given client by its *id*.
///
/// This is a `POST` request on purpose to do an additional CSRF token check for such a
/// sensitive endpoint.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/clients/{id}/secret",
    tag = "clients",
    responses(
        (status = 200, description = "Ok", body = ClientSecretResponse),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "NotFound"),
    ),
)]
#[post("/clients/{id}/secret")]
pub async fn get_client_secret(
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Secrets, AccessRights::Read)?;

    client::get_client_secret(path.into_inner(), &data)
        .await
        .map(|c| HttpResponse::Ok().json(c))
}

/// Adds a new OIDC client to the database.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/clients",
    tag = "clients",
    request_body = NewClientRequest,
    responses(
        (status = 200, description = "Ok", body = ClientResponse),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "NotFound"),
    ),
)]
#[post("/clients")]
pub async fn post_clients(
    client: actix_web_validator::Json<NewClientRequest>,
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Create)?;

    Client::create(&data, client.into_inner())
        .await
        .map(|r| HttpResponse::Ok().json(ClientResponse::from(r)))
}

/// OIDC Dynamic Client Registration (if enabled)
#[utoipa::path(
    post,
    path = "/clients_dyn",
    tag = "clients",
    request_body = DynamicClientRequest,
    responses(
        (status = 201, description = "Created", body = DynamicClientResponse),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "NotFound"),
    ),
)]
#[post("/clients_dyn")]
pub async fn post_clients_dyn(
    data: web::Data<AppState>,
    payload: actix_web_validator::Json<DynamicClientRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    if !*ENABLE_DYN_CLIENT_REG {
        return Ok(HttpResponse::NotFound().finish());
    }

    if let Some(token) = &*DYN_CLIENT_REG_TOKEN {
        let bearer = helpers::get_bearer_token_from_header(req.headers())?;
        if token != &bearer {
            return Ok(HttpResponse::Unauthorized()
                .insert_header((
                    WWW_AUTHENTICATE,
                    r#"error="invalid_token",
                    error_description="Invalid registration access token"#,
                ))
                .finish());
        }
    } else {
        let ip = real_ip_from_req(&req)?;
        ClientDyn::rate_limit_ip(ip).await?;
    }

    Client::create_dynamic(&data, payload.into_inner())
        .await
        .map(|resp| {
            HttpResponse::Created()
                // The registration should be possible from another Web UI by RFC
                .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                .json(resp)
        })
}

/// GET a dynamic OIDC client
#[utoipa::path(
    get,
    path = "/clients_dyn/{id}",
    tag = "clients",
    responses(
        (status = 200, description = "Ok", body = DynamicClientResponse),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "NotFound"),
    ),
)]
#[get("/clients_dyn/{id}")]
pub async fn get_clients_dyn(
    data: web::Data<AppState>,
    id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    if !*ENABLE_DYN_CLIENT_REG {
        return Ok(HttpResponse::NotFound().finish());
    }

    let bearer = helpers::get_bearer_token_from_header(req.headers())?;
    let id = id.into_inner();
    let client_dyn = ClientDyn::find(&data, id.clone()).await?;
    client_dyn.validate_token(&bearer)?;

    let client = Client::find(&data, id).await?;
    let resp = client.into_dynamic_client_response(&data, client_dyn, false)?;
    Ok(HttpResponse::Ok().json(resp))
}

/// Update a dynamic OIDC client
#[utoipa::path(
    put,
    path = "/clients_dyn/{id}",
    tag = "clients",
    request_body = DynamicClientRequest,
    responses(
        (status = 200, description = "Ok", body = DynamicClientResponse),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "NotFound"),
    ),
)]
#[put("/clients_dyn/{id}")]
pub async fn put_clients_dyn(
    data: web::Data<AppState>,
    payload: actix_web_validator::Json<DynamicClientRequest>,
    id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    if !*ENABLE_DYN_CLIENT_REG {
        return Ok(HttpResponse::NotFound().finish());
    }

    let bearer = helpers::get_bearer_token_from_header(req.headers())?;
    let id = id.into_inner();
    let client_dyn = ClientDyn::find(&data, id.clone()).await?;
    client_dyn.validate_token(&bearer)?;

    let resp = Client::update_dynamic(&data, payload.into_inner(), client_dyn).await?;
    Ok(HttpResponse::Ok().json(resp))
}

/// Modifies an OIDC client
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/clients/{id}",
    tag = "clients",
    request_body = UpdateClientRequest,
    responses(
        (status = 200, description = "Ok", body = ClientResponse),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "NotFound"),
    ),
)]
#[put("/clients/{id}")]
pub async fn put_clients(
    data: web::Data<AppState>,
    client: actix_web_validator::Json<UpdateClientRequest>,
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Update)?;

    client::update_client(&data, path.into_inner(), client.into_inner())
        .await
        .map(|r| HttpResponse::Ok().json(ClientResponse::from(r)))
}

/// Returns the color scheme for the login page for this client
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/clients/{id}/colors",
    tag = "clients",
    responses(
        (status = 200, description = "Ok", body = Colors),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/clients/{id}/colors")]
pub async fn get_client_colors(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Read)?;

    ColorEntity::find(&data, id.as_str())
        .await
        .map(|c| HttpResponse::Ok().json(c))
}

/// Set the color scheme for the login page for this client
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/clients/{id}/colors",
    tag = "clients",
    request_body = ColorsRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/clients/{id}/colors")]
pub async fn put_client_colors(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
    req_data: actix_web_validator::Json<ColorsRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Update)?;

    let colors = req_data.into_inner();
    colors.validate_css()?;
    ColorEntity::update(&data, id.as_str(), colors).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Reset the color scheme for the login page for this client to default
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/clients/{id}/colors",
    tag = "clients",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/clients/{id}/colors")]
pub async fn delete_client_colors(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Delete)?;

    ColorEntity::delete(&data, id.as_str()).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Retrieve a custom logo for the login page for this client
#[utoipa::path(
    get,
    path = "/clients/{id}/logo",
    tag = "clients",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/clients/{id}/logo")]
pub async fn get_client_logo(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    let id = id.into_inner();
    debug!("Looking up client logo for id {}", id);
    let logo = match Logo::find_cached(&data, &id, &LogoType::Client).await {
        Ok(logo) => logo,
        Err(_) => {
            debug!("no specific logo for id {} - using Rauthy default", id);
            // If this client does not have a custom logo, we will always serve
            // Rauthy's logo as default
            Logo::find_cached(&data, "rauthy", &LogoType::Client).await?
        }
    };

    Ok(HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, logo.content_type))
        // clients should cache the logos for 12 hours
        // this means if a logo has been updated, they receive the new one 12 hours
        // later in the worst case
        .insert_header((CACHE_CONTROL, "max-age=43200"))
        .body(logo.data))
}

/// Upload a custom logo for the login page for this client
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/clients/{id}/logo",
    tag = "clients",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/clients/{id}/logo")]
pub async fn put_client_logo(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
    mut payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Update)?;

    // we only accept a single field from the Multipart upload -> no looping here
    let mut buf: Vec<u8> = Vec::with_capacity(128 * 1024);
    let mut content_type = None;
    if let Some(part) = payload.next().await {
        let mut field = part?;

        match field.content_type() {
            Some(mime) => {
                debug!("content_type: {:?}", mime);
                content_type = Some(mime.clone());
            }
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "content_type is missing".to_string(),
                ));
            }
        }

        while let Some(chunk) = field.next().await {
            let bytes = chunk?;
            buf.extend(bytes);
        }
    }

    // content_type unwrap cannot panic -> checked above
    Logo::upsert(
        &data,
        id.into_inner(),
        buf,
        content_type.unwrap(),
        LogoType::Client,
    )
    .await?;
    Ok(HttpResponse::Ok().finish())
}

/// Deletes a custom logo for this client
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
delete,
    path = "/clients/{id}/logo",
    tag = "clients",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/clients/{id}/logo")]
pub async fn delete_client_logo(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Delete)?;

    if id.as_str() == "rauthy" {
        Logo::upsert_rauthy_default(&data).await?;
    } else {
        Logo::delete(&data, id.as_str(), &LogoType::Client).await?;
    }

    Ok(HttpResponse::Ok().finish())
}

/// Generates a new client secret
///
/// Generates a new secret for the given client id and sets the client to `confidential` too, if it was
/// not the case yet.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/clients/{id}/secret",
    tag = "clients",
    responses(
        (status = 200, description = "Ok", body = ClientSecretResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[put("/clients/{id}/secret")]
pub async fn put_generate_client_secret(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Secrets, AccessRights::Update)?;

    client::generate_new_secret(id.into_inner(), &data)
        .await
        .map(|r| HttpResponse::Ok().json(r))
}

/// Deletes an OIDC client
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/clients/{id}",
    tag = "clients",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[delete("/clients/{id}")]
pub async fn delete_client(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Delete)?;

    let id = id.into_inner();

    if &id == "rauthy" {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "The `rauthy` client must not be deleted".to_string(),
        ));
    }

    let client = Client::find(&data, id).await?;
    client.delete(&data).await?;
    Ok(HttpResponse::Ok().finish())
}
