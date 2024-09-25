use crate::{map_auth_step, ReqPrincipal};
use actix_web::http::header::{CACHE_CONTROL, CONTENT_TYPE, LOCATION};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web_lab::__reexports::futures_util::StreamExt;
use actix_web_validator::Json;
use rauthy_api_types::auth_providers::ProviderResponse;
use rauthy_api_types::auth_providers::{
    ProviderCallbackRequest, ProviderLoginRequest, ProviderLookupRequest, ProviderRequest,
};
use rauthy_common::constants::{HEADER_HTML, HEADER_JSON};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::auth_providers::{
    AuthProvider, AuthProviderCallback, AuthProviderLinkCookie, AuthProviderTemplate,
};
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::entity::logos::{Logo, LogoType};
use rauthy_models::entity::users::User;
use rauthy_models::language::Language;
use rauthy_models::templates::ProviderCallbackHtml;
use tracing::debug;

/// GET all upstream auth providers
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    post,
    path = "/providers",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/providers")]
pub async fn post_providers(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let providers = AuthProvider::find_all(&data).await?;
    let mut resp = Vec::with_capacity(providers.len());
    for provider in providers {
        resp.push(ProviderResponse::try_from(provider)?);
    }

    Ok(HttpResponse::Ok().json(resp))
}

/// POST create a new upstream auth provider
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    post,
    path = "/providers/create",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers/create")]
pub async fn post_provider(
    data: web::Data<AppState>,
    payload: Json<ProviderRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    if !payload.use_pkce && payload.client_secret.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Must at least be a confidential client or use PKCE".to_string(),
        ));
    }

    let provider = AuthProvider::create(&data, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ProviderResponse::try_from(provider)?))
}

/// POST possible upstream auth provider config lookup
///
/// This will try to autoconfigure and build and upstream auth provider by the given issuer URL.
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    post,
    path = "/providers/lookup",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderLookupResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers/lookup")]
#[tracing::instrument(
    name = "post_provider_lookup",
    skip_all, fields(issuer = payload.issuer)
)]
pub async fn post_provider_lookup(
    payload: Json<ProviderLookupRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let payload = payload.into_inner();
    let resp = AuthProvider::lookup_config(&payload).await?;

    Ok(HttpResponse::Ok().json(resp))
}

/// Start the login flow for an upstream auth provider
///
/// **Permissions**
/// - `session-init`
/// - `session-auth`
#[utoipa::path(
    post,
    path = "/providers/login",
    tag = "providers",
    responses(
        (status = 202, description = "Accepted"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers/login")]
pub async fn post_provider_login(
    data: web::Data<AppState>,
    payload: Json<ProviderLoginRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;

    let payload = payload.into_inner();
    let (cookie, xsrf_token, location) = AuthProviderCallback::login_start(&data, payload).await?;

    Ok(HttpResponse::Accepted()
        .insert_header((LOCATION, location))
        .cookie(cookie)
        .body(xsrf_token))
}

#[get("/providers/callback")]
pub async fn get_provider_callback_html(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy(&data).await?;
    let lang = Language::try_from(&req).unwrap_or_default();
    let body = ProviderCallbackHtml::build(&colors, &lang);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

/// Callback for an upstream auth provider login
///
/// **Permissions**
/// - `session-init`
/// - `session-auth`
#[utoipa::path(
    post,
    path = "/providers/callback",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderLookupResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers/callback")]
#[tracing::instrument(
    name = "post_provider_callback",
    skip_all, fields(callback_id = payload.state)
)]
pub async fn post_provider_callback(
    data: web::Data<AppState>,
    req: HttpRequest,
    payload: Json<ProviderCallbackRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;

    let payload = payload.into_inner();
    let session = principal.get_session()?;
    let (auth_step, cookie) =
        AuthProviderCallback::login_finish(&data, &req, &payload, session.clone()).await?;

    let mut resp = map_auth_step(auth_step, &req).await?;
    resp.add_cookie(&cookie).map_err(|err| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Error adding cookie after map_auth_step: {}", err),
        )
    })?;
    Ok(resp)
}

/// DELETE a link between an existing user account and an upstream provider
///
/// This will always unlink the currently logged-in user from its registered
/// upstream auth provider. The user account must have been set up with at least
/// a password or a passkey. Otherwise, this endpoint will return an error.
#[utoipa::path(
    delete,
    path = "/providers/link",
    tag = "providers",
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
    ),
)]
#[delete("/providers/link")]
pub async fn delete_provider_link(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let user_id = principal.user_id()?.to_string();
    let user = User::provider_unlink(&data, user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

/// GET all upstream auth providers as templated minimal JSON
///
/// This returns the same version of the auth providers as used in the templated `/authorize`
/// page which is inserted during SSR.
#[utoipa::path(
    get,
    path = "/providers/minimal",
    tag = "providers",
    responses(
        (status = 200, description = "OK"),
    ),
)]
#[get("/providers/minimal")]
pub async fn get_providers_minimal(
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    // unauthorized - does not leak any sensitive information other than shown in the
    // default login page anyway
    match AuthProviderTemplate::get_all_json_template(&data).await? {
        None => Ok(HttpResponse::Ok().insert_header(HEADER_JSON).body("[]")),
        Some(tpl) => Ok(HttpResponse::Ok().insert_header(HEADER_JSON).body(tpl)),
    }
}

/// PUT update an upstream auth provider
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    put,
    path = "/providers/{id}",
    tag = "providers",
    responses(
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[put("/providers/{id}")]
pub async fn put_provider(
    data: web::Data<AppState>,
    id: web::Path<String>,
    payload: Json<ProviderRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    if !payload.use_pkce && payload.client_secret.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Must at least be a confidential client or use PKCE".to_string(),
        ));
    }

    AuthProvider::update(&data, id.into_inner(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}

/// DELETE update an upstream auth provider
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    delete,
    path = "/providers/{id}",
    tag = "providers",
    responses(
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[delete("/providers/{id}")]
pub async fn delete_provider(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    AuthProvider::delete(&data, &id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}

/// GET information if it's safe to delete this provider
///
/// This will check if existing users are linked to this provider.
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    get,
    path = "/providers/{id}/delete_safe",
    tag = "providers",
    responses(
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/providers/{id}/delete_safe")]
pub async fn get_provider_delete_safe(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let linked_users = AuthProvider::find_linked_users(&data, &id.into_inner()).await?;
    if linked_users.is_empty() {
        Ok(HttpResponse::Ok().json(linked_users))
    } else {
        Ok(HttpResponse::NotAcceptable().json(linked_users))
    }
}

/// GET the uploaded image an auth provider
#[utoipa::path(
    get,
    path = "/providers/{id}/img",
    tag = "providers",
    responses(
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/providers/{id}/img")]
pub async fn get_provider_img(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    let id = id.into_inner();
    let logo = Logo::find_cached(&data, &id, &LogoType::AuthProvider).await?;

    Ok(HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, logo.content_type))
        // clients should cache the logos for 12 hours
        // this means if a logo has been updated, they receive the new one 12 hours
        // later in the worst case
        .insert_header((CACHE_CONTROL, "max-age=43200"))
        .body(logo.data))
}

/// PUT upload an image / icon for an auth provider
///
/// The image can only be max 10MB in size and will be minified automatically.
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    put,
    path = "/providers/{id}/img",
    tag = "providers",
    responses(
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[put("/providers/{id}/img")]
pub async fn put_provider_img(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
    mut payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

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
        &data,
        id.into_inner(),
        buf,
        content_type.unwrap(),
        LogoType::AuthProvider,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

/// POST a link between an existing user account and an upstream provider
///
/// This action will create a link between an already existing, non-linked account and a configured
/// upstream auth provider. This can only be issued from within an authenticated, valid session.
#[utoipa::path(
    post,
    path = "/providers/{id}/link",
    tag = "providers",
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/providers/{id}/link")]
pub async fn post_provider_link(
    data: web::Data<AppState>,
    provider_id: web::Path<String>,
    principal: ReqPrincipal,
    payload: Json<ProviderLoginRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let user_id = principal.user_id()?.to_string();
    let user = User::find(&data, user_id).await?;

    // make sure the user is currently un-linked
    if user.auth_provider_id.is_some() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "user is already federated",
        ));
    }

    // set an encrypted cookie with the provider_id + user_id / email
    let link_cookie = AuthProviderLinkCookie {
        provider_id: provider_id.into_inner(),
        user_id: user.id,
        user_email: user.email,
    };

    // directly redirect to the provider login page
    let (login_cookie, xsrf_token, location) =
        AuthProviderCallback::login_start(&data, payload.into_inner()).await?;

    Ok(HttpResponse::Accepted()
        .insert_header((LOCATION, location))
        .cookie(login_cookie)
        .cookie(link_cookie.build_cookie()?)
        .body(xsrf_token))
}
