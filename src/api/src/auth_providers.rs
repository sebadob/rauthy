use crate::{ReqPrincipal, content_len_limit, map_auth_step};
use actix_web::http::header::{CACHE_CONTROL, CONTENT_TYPE, LOCATION};
use actix_web::web::{Json, Query};
use actix_web::{HttpRequest, HttpResponse, delete, get, post, put, web};
use actix_web_lab::__reexports::futures_util::StreamExt;
use rauthy_api_types::auth_providers::{
    ProviderCallbackRequest, ProviderLinkedUserResponse, ProviderLoginRequest,
    ProviderLookupRequest, ProviderRequest,
};
use rauthy_api_types::auth_providers::{ProviderLookupResponse, ProviderResponse};
use rauthy_api_types::generic::LogoParams;
use rauthy_api_types::users::{UserResponse, WebauthnLoginResponse};
use rauthy_common::constants::{HEADER_JSON, PROVIDER_ATPROTO};
use rauthy_data::entity::auth_providers::{
    AuthProvider, AuthProviderLinkCookie, AuthProviderTemplate,
};
use rauthy_data::entity::logos::{Logo, LogoType};
use rauthy_data::entity::pow::PowEntity;
use rauthy_data::entity::theme::ThemeCssFull;
use rauthy_data::entity::users::User;
use rauthy_data::html::HtmlCached;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use spow::pow::Pow;
use tracing::debug;
use validator::Validate;

/// GET all upstream auth providers
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    post,
    path = "/providers",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = [ProviderResponse]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/providers")]
pub async fn post_providers(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let providers = AuthProvider::find_all().await?;
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
    request_body = ProviderRequest,
    responses(
        (status = 200, description = "OK", body = ProviderResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers/create")]
pub async fn post_provider(
    Json(payload): Json<ProviderRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    payload.validate()?;

    if payload.issuer == PROVIDER_ATPROTO {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Must not contain a reserved name",
        ));
    }

    if !payload.use_pkce && payload.client_secret.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Must at least be a confidential client or use PKCE",
        ));
    }

    let provider = AuthProvider::create(payload).await?;
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
    request_body = ProviderLookupRequest,
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
    Json(payload): Json<ProviderLookupRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    payload.validate()?;

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
    request_body = ProviderLoginRequest,
    responses(
        (status = 202, description = "Accepted"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers/login")]
pub async fn post_provider_login(
    Json(payload): Json<ProviderLoginRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;
    payload.validate()?;

    let challenge = Pow::validate(&payload.pow)?;
    PowEntity::check_prevent_reuse(challenge.to_string()).await?;

    let (cookie, xsrf_token, location) =
        rauthy_service::oidc::auth_providers::login_start::login_start(payload).await?;

    Ok(HttpResponse::Accepted()
        .insert_header((LOCATION, location))
        .cookie(cookie)
        .body(xsrf_token))
}

#[get("/providers/callback")]
pub async fn get_provider_callback_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AuthProviderCallback
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
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
    request_body = ProviderCallbackRequest,
    responses(
        (status = 200, description = "Correct credentials, but needs to continue with Webauthn MFA Login", body = WebauthnLoginResponse),
        (status = 202, description = "Correct credentials and no MFA Login required, adds Location header"),
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
    req: HttpRequest,
    Json(payload): Json<ProviderCallbackRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    post_provider_callback_handle(req, payload, principal).await
}

// extracted to make it usable in `post_dev_only_endpoints()`
#[inline(always)]
pub async fn post_provider_callback_handle(
    req: HttpRequest,
    payload: ProviderCallbackRequest,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;
    payload.validate()?;

    let (auth_step, cookie, new_user_created) =
        rauthy_service::oidc::auth_providers::login_finish::login_finish(
            &req,
            &payload,
            // unwrap: we already checked for session auth or init above
            principal.into_inner().session.unwrap(),
        )
        .await?;

    let mut resp = map_auth_step(auth_step, &req, new_user_created).await?;
    resp.add_cookie(&cookie).map_err(|err| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Error adding cookie after map_auth_step: {err}"),
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
        (status = 200, description = "OK", body = UserResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
    ),
)]
#[delete("/providers/link")]
pub async fn delete_provider_link(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let user_id = principal.user_id()?.to_string();
    let user = User::provider_unlink(user_id).await?;
    Ok(HttpResponse::Ok().json(user.into_response(None)))
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
        (status = 200, description = "OK", body = [AuthProviderTemplate]),
    ),
)]
#[get("/providers/minimal")]
pub async fn get_providers_minimal() -> Result<HttpResponse, ErrorResponse> {
    // unauthorized - does not leak any sensitive information other than shown in the
    // default login page anyway
    let tpl = AuthProviderTemplate::get_all_json_template().await?;
    Ok(HttpResponse::Ok().insert_header(HEADER_JSON).body(tpl))
}

/// PUT update an upstream auth provider
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    put,
    path = "/providers/{id}",
    tag = "providers",
    request_body = ProviderRequest,
    responses(
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[put("/providers/{id}")]
pub async fn put_provider(
    id: web::Path<String>,
    Json(payload): Json<ProviderRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    payload.validate()?;

    if !payload.use_pkce && payload.client_secret.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Must at least be a confidential client or use PKCE",
        ));
    }
    if payload.client_secret.is_some()
        && !(payload.client_secret_basic || payload.client_secret_post)
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "A confidential client must have a least one of 'client_secret_basic | client_secret_post'",
        ));
    }

    AuthProvider::update(id.into_inner(), payload).await?;
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
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    AuthProvider::delete(&id.into_inner()).await?;
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
        (
            status = 406,
            description = "NotAcceptable - linked users to this provider",
            body = ProviderLinkedUserResponse
        ),
    ),
)]
#[get("/providers/{id}/delete_safe")]
pub async fn get_provider_delete_safe(
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let linked_users = AuthProvider::find_linked_users(&id.into_inner()).await?;
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
    params(LogoParams),
    responses(
        (status = 200, description = "Ok"),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/providers/{id}/img")]
pub async fn get_provider_img(
    id: web::Path<String>,
    params: Query<LogoParams>,
) -> Result<HttpResponse, ErrorResponse> {
    let id = id.into_inner();
    let logo = Logo::find_cached(&id, &LogoType::AuthProvider).await?;

    // we only cache the response if the client properly used the updated param
    // to never run into issues otherwise
    if params.updated.is_some() {
        Ok(HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, logo.content_type))
            .insert_header((
                CACHE_CONTROL,
                "max-age=31104000, stale-while-revalidate=2592000, public",
            ))
            .body(logo.data))
    } else {
        Ok(HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, logo.content_type))
            .body(logo.data))
    }
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
    id: web::Path<String>,
    req: HttpRequest,
    principal: ReqPrincipal,
    mut payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
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
    request_body = ProviderLoginRequest,
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/providers/{id}/link")]
pub async fn post_provider_link(
    provider_id: web::Path<String>,
    principal: ReqPrincipal,
    Json(payload): Json<ProviderLoginRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;
    payload.validate()?;

    let user_id = principal.user_id()?.to_string();
    let user = User::find(user_id).await?;

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
        rauthy_service::oidc::auth_providers::login_start::login_start(payload).await?;

    Ok(HttpResponse::Accepted()
        .insert_header((LOCATION, location))
        .cookie(login_cookie)
        .cookie(link_cookie.build_cookie()?)
        .body(xsrf_token))
}
