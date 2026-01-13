use crate::{ReqPrincipal, content_len_limit};
use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL, CONTENT_SECURITY_POLICY, CONTENT_TYPE,
    WWW_AUTHENTICATE,
};
use actix_web::web::{Json, Query};
use actix_web::{HttpRequest, HttpResponse, delete, get, post, put, web};
use actix_web_lab::__reexports::futures_util::StreamExt;
use rauthy_api_types::clients::{
    ClientResponse, ClientSecretRequest, ClientSecretResponse, DynamicClientRequest,
    DynamicClientResponse, NewClientRequest, UpdateClientRequest,
};
use rauthy_api_types::forward_auth::{ForwardAuthCallbackParams, ForwardAuthParams};
use rauthy_api_types::generic::LogoParams;
use rauthy_common::utils::real_ip_from_req;
use rauthy_data::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::clients_dyn::ClientDyn;
use rauthy_data::entity::clients_scim::ClientScim;
use rauthy_data::entity::failed_backchannel_logout::FailedBackchannelLogout;
use rauthy_data::entity::logos::{Logo, LogoType};
use rauthy_data::entity::user_login_states::UserLoginState;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_service::oidc::{helpers, logout};
use rauthy_service::{client, forward_auth};
use tracing::{debug, error, warn};
use validator::Validate;

/// Returns all existing OIDC clients with all their information, except for the client secrets.
///
/// This endpoint will NOT include any `SCIM` configurations. These will only be included on direct
/// `GET /clients/{id}` requests.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/clients",
    tag = "clients",
    responses(
        (status = 200, description = "Ok", body = [ClientResponse]),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "NotFound"),
    ),
)]
#[tracing::instrument(skip_all)]
#[get("/clients")]
pub async fn get_clients(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Read)?;

    let clients = Client::find_all().await?;

    let mut res = Vec::new();
    clients
        .into_iter()
        .for_each(|c| res.push(c.into_response(None)));

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
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Read)?;

    let client = Client::find(path.into_inner()).await?;
    let scim = ClientScim::find_opt(client.id.clone()).await?;

    Ok(HttpResponse::Ok().json(client.into_response(scim)))
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
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Secrets, AccessRights::Read)?;

    client::get_client_secret(path.into_inner())
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
    Json(payload): Json<NewClientRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Create)?;
    payload.validate()?;

    // The `NewClientRequest` does not allow setting up SCIM immediately - no need to trigger here

    let client = Client::create(payload).await?;
    Ok(HttpResponse::Ok().json(client.into_response(None)))
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
    Json(payload): Json<DynamicClientRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    if !RauthyConfig::get().vars.dynamic_clients.enable {
        warn!("Dynamic Clients are disabled - rejecting request");
        return Ok(HttpResponse::NotFound().finish());
    }
    payload.validate()?;

    if let Some(token) = &RauthyConfig::get().vars.dynamic_clients.reg_token {
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

    Client::create_dynamic(payload).await.map(|resp| {
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
    id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    if !RauthyConfig::get().vars.dynamic_clients.enable {
        return Ok(HttpResponse::NotFound().finish());
    }

    let bearer = helpers::get_bearer_token_from_header(req.headers())?;
    let id = id.into_inner();
    let client_dyn = ClientDyn::find(id.clone()).await?;
    client_dyn.validate_token(&bearer)?;

    let client = Client::find(id).await?;
    let resp = client.into_dynamic_client_response(client_dyn, false)?;
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
    Json(payload): Json<DynamicClientRequest>,
    id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    if !RauthyConfig::get().vars.dynamic_clients.enable {
        return Ok(HttpResponse::NotFound().finish());
    }
    payload.validate()?;

    let bearer = helpers::get_bearer_token_from_header(req.headers())?;
    let id = id.into_inner();
    let client_dyn = ClientDyn::find(id.clone()).await?;
    client_dyn.validate_token(&bearer)?;

    let resp = Client::update_dynamic(payload, client_dyn).await?;
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
    Json(payload): Json<UpdateClientRequest>,
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Update)?;
    payload.validate()?;

    let client_id = path.into_inner();
    let (client, scim) = client::update_client(client_id, payload).await?;
    debug!("scim: {:?}", scim);

    let resp = if let Some((scim, needs_sync)) = scim {
        let resp = client.into_response(Some(scim.clone()));
        debug!("scim needs sync: {:?}", needs_sync);
        if needs_sync {
            scim.sync_full().await?;
        }
        resp
    } else {
        client.into_response(None)
    };

    Ok(HttpResponse::Ok().json(resp))
}

/// Retrieve a custom logo for the login page for this client
#[utoipa::path(
    get,
    path = "/clients/{id}/logo",
    tag = "clients",
    params(LogoParams),
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/clients/{id}/logo")]
pub async fn get_client_logo(
    id: web::Path<String>,
    params: Query<LogoParams>,
) -> Result<HttpResponse, ErrorResponse> {
    let id = id.into_inner();

    let (content_type, data) = if let Ok(logo) = Logo::find_cached(&id, &LogoType::Client).await {
        (logo.content_type, logo.data)
    } else if id != "rauthy"
        && let Ok(logo) = Logo::find_cached("rauthy", &LogoType::Client).await
    {
        // use the rauthy logo as fallback
        (logo.content_type, logo.data)
    } else {
        return Ok(HttpResponse::NotFound().finish());
    };

    // we only cache the response if the client properly used the updated param
    // to never run into issues otherwise
    let csp = "connect-src 'none'; script-src 'none'; frame-ancestors 'none'; object-src 'none';";
    if params.updated.is_some() {
        Ok(HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, content_type))
            .insert_header((CONTENT_SECURITY_POLICY, csp))
            .insert_header((
                CACHE_CONTROL,
                "max-age=31104000, stale-while-revalidate=2592000, public",
            ))
            .body(data))
    } else {
        Ok(HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, content_type))
            .insert_header((CONTENT_SECURITY_POLICY, csp))
            .body(data))
    }
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
    id: web::Path<String>,
    req: HttpRequest,
    principal: ReqPrincipal,
    mut payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Update)?;
    content_len_limit(&req, 10)?;

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
                    "content_type is missing",
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
        id.into_inner(),
        buf,
        content_type.unwrap(),
        LogoType::Client,
    )
    .await?;

    Ok(HttpResponse::Ok()
        .insert_header(("Clear-Site-Data", "cache"))
        .finish())
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
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Delete)?;

    Logo::delete(id.as_str(), &LogoType::Client).await?;

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
    id: web::Path<String>,
    principal: ReqPrincipal,
    payload: Option<Json<ClientSecretRequest>>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Secrets, AccessRights::Update)?;
    let payload = if let Some(p) = payload {
        p.into_inner()
    } else {
        Default::default()
    };
    payload.validate()?;

    client::generate_new_secret(id.into_inner(), payload.cache_current_hours)
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
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Clients, AccessRights::Delete)?;

    let id = id.into_inner();

    if id == "rauthy" {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "The `rauthy` client must not be deleted",
        ));
    }

    let client = Client::find(id).await?;

    // We do NOT want to execute the potentially long-running backchannel logout in the background
    // this time. Before the backchannel logout is not finished, we will not be able to delete the
    // client because of foreign key constraints.
    if let Err(err) = logout::execute_backchannel_logout_by_client(&client).await {
        error!(
            "Error during async backchannel logout after client delete: {:?}",
            err
        );
        UserLoginState::delete_all_by_cid(client.id.clone()).await?;
    }
    // If we had some failed backchannel logouts, we will not retry them.
    // They would fail anyway, because the client is deleted from this point on.
    if let Err(err) = FailedBackchannelLogout::delete_all_by_client(client.id.clone()).await {
        error!("Error cleaning up FailedBackchannelLogouts: {:?}", err);
    }

    client.delete().await?;

    Ok(HttpResponse::Ok().finish())
}

/// Forward auth endpoint with OIDC flow.
///
/// In contrast to the much more simple `/oidc/forward_auth`, which only check the `Authorization`
/// header for a valid JWT token, this endpoint expects an encrypted cookie and redirects to the
/// login / callback if invalid.
#[utoipa::path(
    get,
    path = "/clients/{id}/forward_auth",
    tag = "clients",
    params(ForwardAuthParams),
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
    ),
)]
#[tracing::instrument(level = "debug", skip(req))]
#[get("/clients/{id}/forward_auth")]
pub async fn get_forward_auth_oidc(
    id: web::Path<String>,
    req: HttpRequest,
    params: Query<ForwardAuthParams>,
) -> Result<HttpResponse, ErrorResponse> {
    params.validate()?;

    match forward_auth::get_forward_auth_client(id.into_inner(), req, params.into_inner()).await {
        Ok(r) => {
            debug!("Forward Auth is OK");
            Ok(r)
        }
        Err(err) => {
            debug!("Error during GET /clients/{{id}}/forward_auth: {:?}", err);
            Err(err)
        }
    }
}

#[utoipa::path(
    get,
    path = "/clients/{id}/forward_auth/callback",
    tag = "clients",
    params(ForwardAuthCallbackParams),
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
    ),
)]
#[tracing::instrument(level = "debug", skip(req))]
#[get("/clients/{id}/forward_auth/callback")]
pub async fn get_forward_auth_callback(
    id: web::Path<String>,
    req: HttpRequest,
    params: Query<ForwardAuthCallbackParams>,
) -> Result<HttpResponse, ErrorResponse> {
    params.validate()?;

    match forward_auth::get_forward_auth_client_callback(id.into_inner(), req, params.into_inner())
        .await
    {
        Ok(r) => Ok(r),
        Err(err) => {
            debug!("Error: {:?}", err);
            Err(err)
        }
    }
}
