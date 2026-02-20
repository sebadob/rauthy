use crate::{ReqPrincipal, content_len_limit};
use actix_web::http::StatusCode;
use actix_web::http::header::{ACCEPT, HeaderName, HeaderValue, LOCATION};
use actix_web::mime::TEXT_HTML;
use actix_web::web::{Json, Query};
use actix_web::{HttpRequest, HttpResponse, ResponseError, delete, get, patch, post, put, web};
use chrono::Utc;
use rauthy_api_types::PatchOp;
use rauthy_api_types::generic::{PaginationParams, PasswordPolicyResponse};
use rauthy_api_types::oidc::PasswordResetResponse;
use rauthy_api_types::users::*;
use rauthy_common::constants::{
    COOKIE_MFA, HEADER_ALLOW_ALL_ORIGINS, HEADER_HTML, HEADER_JSON, PWD_CSRF_HEADER,
    PWD_RESET_COOKIE, TEXT_TURTLE,
};
use rauthy_common::utils::real_ip_from_req;
use rauthy_data::api_cookie::ApiCookie;
use rauthy_data::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_data::entity::browser_id::BrowserId;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::clients_scim::ClientScim;
use rauthy_data::entity::continuation_token::ContinuationToken;
use rauthy_data::entity::devices::DeviceEntity;
use rauthy_data::entity::groups::Group;
use rauthy_data::entity::login_locations::LoginLocation;
use rauthy_data::entity::mfa_mod_token::MfaModToken;
use rauthy_data::entity::password::PasswordPolicy;
use rauthy_data::entity::pictures::{PICTURE_STORAGE_TYPE, PictureStorage, UserPicture};
use rauthy_data::entity::pow::PowEntity;
use rauthy_data::entity::refresh_tokens::RefreshToken;
use rauthy_data::entity::sessions::Session;
use rauthy_data::entity::theme::ThemeCssFull;
use rauthy_data::entity::tos::ToS;
use rauthy_data::entity::tos_user_accept::ToSUserAccept;
use rauthy_data::entity::user_attr::{UserAttrConfigEntity, UserAttrValueEntity};
use rauthy_data::entity::user_revoke::UserRevoke;
use rauthy_data::entity::users::User;
use rauthy_data::entity::users_values::UserValues;
use rauthy_data::entity::webauthn;
use rauthy_data::entity::webauthn::{PasskeyEntity, WebauthnAdditionalData, WebauthnServiceReq};
use rauthy_data::entity::webids::WebId;
use rauthy_data::events::event::Event;
use rauthy_data::html::HtmlCached;
use rauthy_data::html::templates::{Error3Html, ErrorHtml, UserRevokeHtml};
use rauthy_data::ipgeo;
use rauthy_data::ipgeo::get_location;
use rauthy_data::language::Language;
use rauthy_data::rauthy_config::{RauthyConfig, UserValueConfigValue, VarsUserValuesConfig};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_jwt::claims::{JwtCommonClaims, JwtTokenType};
use rauthy_jwt::token::JwtToken;
use rauthy_service::oidc::helpers::get_bearer_token_from_header;
use rauthy_service::oidc::logout;
use rauthy_service::password_reset;
use rauthy_service::user_values_validator::UserValuesValidator;
use spow::pow::Pow;
use std::cmp::max;
use std::collections::HashMap;
use std::net::IpAddr;
use tokio::task;
use tracing::{debug, error, info, warn};
use validator::Validate;

/// Returns all existing users
///
/// This endpoint will switch from simply returning all existing users to a server side pagination
/// dynamically, depending on the threshold set via `SSP_THRESHOLD`.
/// If the response contains all existing users, the status code will be an HTTP 200.
/// If the backend is in server side pagination mode, it will return an HTTP 206.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    params(PaginationParams),
    responses(
        (status = 200, description = "Ok", body = [UserResponseSimple]),
        (status = 206, description = "PartialContent", body = [UserResponse]),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/users")]
pub async fn get_users(
    principal: ReqPrincipal,
    Query(params): Query<PaginationParams>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Read)?;
    params.validate()?;

    let user_count = User::count().await?;
    let ssp_threshold = RauthyConfig::get().vars.server.ssp_threshold;
    if user_count >= ssp_threshold as i64 {
        let page_size = max(params.page_size.unwrap_or(20), ssp_threshold) as i64;
        let offset = params.offset.unwrap_or(0) as i64;
        let backwards = params.backwards.unwrap_or(false);
        let continuation_token = if let Some(token) = &params.continuation_token {
            Some(ContinuationToken::try_from(token.as_str())?)
        } else {
            None
        };

        let (users, continuation_token) =
            User::find_paginated(continuation_token, page_size, offset, backwards).await?;
        let x_page_count = (user_count as f64 / page_size as f64).ceil() as u32;

        if let Some(token) = continuation_token {
            Ok(HttpResponse::PartialContent()
                .insert_header(("x-user-count", user_count))
                .insert_header(("x-page-count", x_page_count))
                .insert_header(("x-page-size", page_size as u32))
                .insert_header(token.into_header_pair())
                .json(users))
        } else {
            Ok(HttpResponse::PartialContent()
                .insert_header(("x-user-count", user_count))
                .insert_header(("x-page-count", x_page_count))
                .insert_header(("x-page-size", page_size as u32))
                .json(users))
        }
    } else {
        let users = User::find_all_simple().await?;
        Ok(HttpResponse::Ok()
            .insert_header(("x-user-count", user_count))
            .json(users))
    }
}

/// Adds a new user to the database
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = NewUserRequest,
    responses(
        (status = 200, description = "Ok", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/users")]
pub async fn post_users(
    req: HttpRequest,
    principal: ReqPrincipal,
    Json(payload): Json<NewUserRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Create)?;
    payload.validate()?;
    // We are not using the UserValuesValidator on purpose here.
    // When an admin registers a new user, the user details view will be shown immediately anyway,
    // and an admin may have good reason to not set some values, like e.g. the preferred username.

    let user = User::create_from_new(payload).await?;

    RauthyConfig::get()
        .tx_events
        .send_async(Event::new_user(
            user.email.clone(),
            real_ip_from_req(&req)?.to_string(),
        ))
        .await
        .unwrap();
    if user.is_admin() {
        RauthyConfig::get()
            .tx_events
            .send_async(Event::new_rauthy_admin(
                user.email.clone(),
                real_ip_from_req(&req)?.to_string(),
            ))
            .await
            .unwrap();
    }

    let cloned = user.clone();
    task::spawn(async move {
        let email = cloned.email.clone();
        if let Err(err) = ClientScim::create_update_user(cloned).await {
            error!(
                "Error during SCIM Client user create for {}: {:?}",
                email, err
            );
        }
    });

    Ok(HttpResponse::Ok().json(user.into_response(None)))
}

/// Get the configured / allowed additional custom user attribute
#[utoipa::path(
    get,
    path = "/users/attr",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = UserAttrConfigResponse),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/users/attr")]
pub async fn get_cust_attr(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Read)?;

    UserAttrConfigEntity::find_all().await.map(|values| {
        HttpResponse::Ok().json(UserAttrConfigResponse {
            values: values.into_iter().map(|v| v.into()).collect(),
        })
    })
}

/// Create a new allowed additional custom user attribute
#[utoipa::path(
    post,
    path = "/users/attr",
    tag = "users",
    request_body = UserAttrConfigRequest,
    responses(
        (status = 200, description = "Ok", body = UserAttrConfigValueResponse),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/users/attr")]
pub async fn post_cust_attr(
    principal: ReqPrincipal,
    Json(payload): Json<UserAttrConfigRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal
        .validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Create)?;
    payload.validate()?;

    UserAttrConfigEntity::create(payload)
        .await
        .map(|attr| HttpResponse::Ok().json(UserAttrConfigValueResponse::from(attr)))
}

/// Update an additional custom user attribute
#[utoipa::path(
    put,
    path = "/users/attr/{name}",
    tag = "users",
    request_body = UserAttrConfigRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[put("/users/attr/{name}")]
pub async fn put_cust_attr(
    path: web::Path<String>,
    principal: ReqPrincipal,
    Json(payload): Json<UserAttrConfigRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal
        .validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Update)?;
    payload.validate()?;

    // Note: Even though in the Admin UI, a change from user-editable back to non-editable is
    // not allowed at all, we do not check this condition here on purpose. The only reason is that
    // we want to have at least the possibility to do this via an API call. The reason we only leave
    // this option for direct API calls is that it should only be done with intention and not easily
    // by clicking a button without thinking twice about the risks and implications.

    let entity = UserAttrConfigEntity::update(path.into_inner(), payload).await?;

    let clients_scim = ClientScim::find_with_attr_mapping(&entity.name).await?;
    if !clients_scim.is_empty() {
        let groups = Group::find_all().await?;
        tokio::spawn(async move {
            let mut groups_remote = HashMap::with_capacity(groups.len());
            for scim in clients_scim {
                if let Err(err) = scim.sync_users(None, &groups, &mut groups_remote).await {
                    error!("{}", err);
                }
            }
        });
    }

    Ok(HttpResponse::Ok().json(UserAttrConfigValueResponse::from(entity)))
}

/// Delete an additional custom user attribute
#[utoipa::path(
    delete,
    path = "/users/attr/{name}",
    tag = "users",
    responses(
        (status = 204, description = "NoContent"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "NotFound"),
    ),
)]
#[delete("/users/attr/{name}")]
pub async fn delete_cust_attr(
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal
        .validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Delete)?;

    let attr_name = path.into_inner();

    let clients_scim = ClientScim::find_with_attr_mapping(&attr_name).await?;
    if !clients_scim.is_empty() {
        let groups = Group::find_all().await?;
        tokio::spawn(async move {
            let mut groups_remote = HashMap::with_capacity(groups.len());
            for scim in clients_scim {
                if let Err(err) = scim.sync_users(None, &groups, &mut groups_remote).await {
                    error!("{}", err);
                }
            }
        });
    }

    UserAttrConfigEntity::delete(attr_name).await?;
    Ok(HttpResponse::Ok().finish())
}

/// GET backend config for user picture uploads
#[utoipa::path(
    get,
    path = "/users/picture_config",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = UserPictureConfig),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[get("/users/picture_config")]
pub async fn get_user_picture_config(
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    if let Err(err) = principal.validate_session_auth()
        && principal
            .validate_api_key(AccessGroup::Users, AccessRights::Read)
            .is_err()
    {
        return Err(err);
    }

    Ok(HttpResponse::Ok().json(UserPictureConfig {
        upload_allowed: *PICTURE_STORAGE_TYPE != PictureStorage::Disabled,
        content_len_limit: RauthyConfig::get().vars.user_pictures.upload_limit_mb as u32
            * 1024
            * 1024,
    }))
}

/// Get the HTML Page for the User Registration
#[utoipa::path(
    get,
    path = "/users/register",
    tag = "users",
    request_body = NewUserRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 403, description = "Forbidden: Open registration may be not allowed via config"),
    ),
)]
#[get("/users/register")]
pub async fn get_users_register(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    if !RauthyConfig::get().vars.user_registration.enable {
        return Ok(HttpResponse::NotFound().finish());
    }
    HtmlCached::UserRegistration
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

/// Creates a new user with almost all values set to default
///
/// This is the endpoint for the possibly allowed open user registration endpoint and can be
/// accessed by anyone, if configured.<br>
/// A Proof of Work (PoW) must be computed by the client to fight automatic bots and spammers.
#[utoipa::path(
    post,
    path = "/users/register",
    tag = "users",
    request_body = NewUserRegistrationRequest,
    responses(
        (status = 204, description = "NoContent"),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/users/register")]
pub async fn post_users_register(
    req: HttpRequest,
    Json(payload): Json<NewUserRegistrationRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    post_users_register_handle(req, payload).await
}

// extracted to be usable from `post_dev_only_endpoints()`
#[inline(always)]
pub async fn post_users_register_handle(
    req: HttpRequest,
    payload: NewUserRegistrationRequest,
) -> Result<HttpResponse, ErrorResponse> {
    if !RauthyConfig::get().vars.user_registration.enable {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Open User Registration is disabled",
        ));
    }

    payload.validate()?;
    UserValuesValidator {
        given_name: payload.given_name.as_deref(),
        family_name: payload.family_name.as_deref(),
        preferred_username: payload.preferred_username.as_deref(),
        user_values: &payload.user_values,
    }
    .validate()?;

    let reg = &RauthyConfig::get().vars.user_registration;

    if let Some(restriction) = &reg.domain_restriction {
        if !payload.email.ends_with(restriction) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Domain for the open registration are restricted to '@{restriction}'"),
            ));
        }
    } else if !reg.domain_blacklist.is_empty() {
        for blacklisted in &reg.domain_blacklist {
            if payload.email.ends_with(blacklisted) {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Domain is blacklisted",
                ));
            }
        }
    }

    // Note: Always keep the PoW validation BEFORE any other expensive checks with DB access.
    let challenge = Pow::validate(&payload.pow)?;
    PowEntity::check_prevent_reuse(challenge.to_string()).await?;

    if let Some(redirect_uri) = &payload.redirect_uri
        && !reg.allow_open_redirect
    {
        let mut allow = false;
        for uri in Client::find_all_client_uris().await? {
            if uri.starts_with(redirect_uri) {
                allow = true;
                break;
            }
        }
        if !allow {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "given `redirect_uri` not allowed",
            ));
        }
    }

    let lang = Language::try_from(&req).unwrap_or_default();
    let user = User::create_from_reg(payload, lang).await?;

    RauthyConfig::get()
        .tx_events
        .send_async(Event::new_user(
            user.email.clone(),
            real_ip_from_req(&req)?.to_string(),
        ))
        .await
        .unwrap();

    if let Some(tos) = ToS::find_latest().await? {
        let ip = real_ip_from_req(&req)?;
        let loc = get_location(&req, ip)?;
        ToSUserAccept::create(user.id.clone(), tos.ts, ip, loc).await?;
    }

    task::spawn(async move {
        let email = user.email.clone();
        if let Err(err) = ClientScim::create_update_user(user).await {
            error!(
                "Error during SCIM Client user create for {}: {:?}",
                email, err
            );
        }
    });

    Ok(HttpResponse::NoContent()
        .insert_header(HEADER_ALLOW_ALL_ORIGINS)
        .finish())
}

/// Returns a single user by its *id*
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[get("/users/{id}")]
pub async fn get_user_by_id(
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    let id = path.into_inner();

    // principal must either be an admin or have the same user id
    let api_key_or_admin = principal
        .validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Read)
        .is_ok();
    if !api_key_or_admin {
        principal.validate_session_auth()?;
        principal.is_user(&id)?;
    }

    let user = User::find(id).await?;
    let values = UserValues::find(&user.id).await?;

    Ok(HttpResponse::Ok().json(user.into_response(values)))
}

/// Returns the additional custom attributes for the given user id
#[utoipa::path(
    get,
    path = "/users/{id}/attr",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = UserAttrValuesResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[get("/users/{id}/attr")]
pub async fn get_user_attr(
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Read)?;

    let values = UserAttrValueEntity::find_for_user(&path.into_inner())
        .await?
        .drain(..)
        .map(UserAttrValueResponse::from)
        .collect::<Vec<UserAttrValueResponse>>();

    Ok(HttpResponse::Ok().json(UserAttrValuesResponse { values }))
}

/// Returns the additional custom attributes for the given user id that are user-editable
#[utoipa::path(
    get,
    path = "/users/{id}/attr/editable",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = UserAttrValuesResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[get("/users/{id}/attr/editable")]
pub async fn get_user_attr_editable(
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;
    let user_id = path.into_inner();
    if user_id != principal.user_id()? {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "mismatch between path and Principal `id`",
        ));
    }

    let configs = UserAttrConfigEntity::find_all_user_editable().await?;
    let mut values = UserAttrValueEntity::find_for_user(&user_id).await?;

    let mut combined = Vec::with_capacity(configs.len());
    for config in configs {
        let default_value = if let Some(default) = config.default_value {
            Some(serde_json::from_slice(&default)?)
        } else {
            None
        };

        let value = if let Some(pos) = values.iter().position(|v| v.key == config.name) {
            let value = values.swap_remove(pos);
            Some(serde_json::from_slice(&value.value)?)
        } else {
            None
        };

        combined.push(UserEditableAttrResponse {
            name: config.name,
            desc: config.desc,
            default_value,
            typ: config.typ,
            value,
        });
    }

    Ok(HttpResponse::Ok().json(UserEditableAttrsResponse { values: combined }))
}

/// Updates the additional custom attributes for the given user id
#[utoipa::path(
    put,
    path = "/users/{id}/attr",
    tag = "users",
    request_body = UserAttrValuesUpdateRequest,
    responses(
        (status = 204, description = "NoContent"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[put("/users/{id}/attr")]
pub async fn put_user_attr(
    path: web::Path<String>,
    principal: ReqPrincipal,
    Json(mut payload): Json<UserAttrValuesUpdateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let user_id = path.into_inner();
    if principal
        .validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Update)
        .is_err()
    {
        principal.validate_session_auth()?;
        if user_id != principal.user_id()? {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "mismatch between path and Principal `id`",
            ));
        }

        // For all non-admins, we want to make sure the payload
        // only contains those keys, that are actually user-editable
        let allowed_keys = UserAttrConfigEntity::find_all_user_editable()
            .await?
            .into_iter()
            .map(|c| c.name)
            .collect::<Vec<_>>();
        payload.values.retain(|v| allowed_keys.contains(&v.key));
    }
    payload.validate()?;

    let user = User::find(user_id).await?;
    let values = UserAttrValueEntity::update_for_user(&user.id, payload)
        .await?
        .drain(..)
        .map(UserAttrValueResponse::from)
        .collect::<Vec<UserAttrValueResponse>>();

    task::spawn(async move {
        let email = user.email.clone();
        if let Err(err) = ClientScim::create_update_user(user).await {
            error!(
                "Error during SCIM Client user attr update for {}: {:?}",
                email, err
            );
        }
    });

    Ok(HttpResponse::Ok().json(UserAttrValuesResponse { values }))
}

/// Retrieve an `MfaModToken` to be able to modify MFA keys
///
/// This endpoint is for password-only. If the user already has registered Passkeys, the token can
/// be retrieved via a Webauthn-Flow.
#[utoipa::path(
    post,
    path = "/users/{id}/mfa_token",
    tag = "users",
    request_body = MfaModTokenRequest,
    responses(
        (status = 200, description = "Ok", body = MfaModTokenResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[post("/users/{id}/mfa_token")]
pub async fn post_user_mfa_token(
    path: web::Path<String>,
    principal: ReqPrincipal,
    req: HttpRequest,
    Json(payload): Json<MfaModTokenRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;
    payload.validate()?;

    let user_id = principal.user_id()?;
    if path.into_inner() != user_id {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "mismatch between path and Principal `id`",
        ));
    }

    let user = User::find(user_id.to_string()).await?;

    if let Some(password) = payload.password {
        if user.has_webauthn_enabled() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "must provide `mfa_code` instead of `password`",
            ));
        }
        if user.validate_password(password).await.is_err() {
            // we don't want to return 401 on purpose to not trigger a redirect to login
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid password",
            ));
        }
    } else if let Some(code) = payload.mfa_code {
        if !user.has_webauthn_enabled() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "must provide `password`",
            ));
        }
        let svc_req = WebauthnServiceReq::find(code).await?;
        if svc_req.user_id != user.id {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "mismatch in UserID",
            ));
        }
        svc_req.delete().await?;
    } else {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "empty payload",
        ));
    }

    let ip = real_ip_from_req(&req)?;
    let token = MfaModToken::new(user_id.to_string(), ip).await?;

    Ok(HttpResponse::Ok().json(MfaModTokenResponse::from(token)))
}

/// Upload a user picture
///
/// Returns the new `picture_id` as the text body.
#[utoipa::path(
    get,
    path = "/users/{user_id}/picture",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = String),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[put("/users/{user_id}/picture")]
pub async fn put_user_picture(
    path: web::Path<String>,
    req: HttpRequest,
    principal: ReqPrincipal,
    payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ErrorResponse> {
    if *PICTURE_STORAGE_TYPE == PictureStorage::Disabled {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "user picture upload is disabled",
        ));
    }

    let user_id = path.into_inner();
    if let Err(err) = principal.validate_user_or_admin(&user_id)
        && principal
            .validate_api_key(AccessGroup::Users, AccessRights::Update)
            .is_err()
    {
        return Err(err);
    }

    content_len_limit(&req, RauthyConfig::get().vars.user_pictures.upload_limit_mb)?;

    let resp = UserPicture::upload(user_id.clone(), payload).await?;

    let user = User::find(user_id).await?;
    task::spawn(async move {
        let email = user.email.clone();
        if let Err(err) = ClientScim::create_update_user(user).await {
            error!(
                "Error during SCIM Client user update (picture) for {}: {:?}",
                email, err
            );
        }
    });

    Ok(resp)
}

/// GET / download the user picture
#[utoipa::path(
    get,
    path = "/users/{user_id}/picture/{picture_id}",
    tag = "users",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/users/{user_id}/picture/{picture_id}")]
pub async fn get_user_picture(
    path: web::Path<(String, String)>,
    req: HttpRequest,
    principal: Option<ReqPrincipal>,
) -> Result<HttpResponse, ErrorResponse> {
    let (user_id, picture_id) = path.into_inner();

    let cors_header = if !RauthyConfig::get().vars.user_pictures.public {
        validate_user_picture_access(&req, principal, &user_id).await?
    } else {
        None
    };

    {
        let user = User::find(user_id).await?;
        if user.picture_id.as_deref() != Some(&picture_id) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "invalid picture_id for user",
            ));
        }
    }

    UserPicture::download(picture_id, cors_header).await
}

#[inline]
async fn validate_user_picture_access(
    req: &HttpRequest,
    principal: Option<ReqPrincipal>,
    user_id: &str,
) -> Result<Option<(HeaderName, HeaderValue)>, ErrorResponse> {
    if let Some(principal) = principal {
        if principal.validate_user_or_admin(user_id).is_ok() {
            return Ok(None);
        }
        if principal
            .validate_api_key(AccessGroup::Users, AccessRights::Read)
            .is_ok()
        {
            return Ok(None);
        }
    }

    if let Ok(bearer) = get_bearer_token_from_header(req.headers()) {
        let mut buf = Vec::with_capacity(512);
        if JwtToken::validate_claims_into(&bearer, None, 0, &mut buf)
            .await
            .is_ok()
        {
            let claims: JwtCommonClaims = serde_json::from_slice(&buf)?;
            debug!("token claims are ok: {:?}", claims);
            if (claims.typ == JwtTokenType::Bearer || claims.typ == JwtTokenType::Id)
                && claims.sub == Some(user_id)
                && claims.scope.unwrap_or_default().contains("profile")
            {
                let client = Client::find(claims.azp.to_string()).await?;
                let cors = client.get_validated_origin_header(req)?;
                return Ok(cors);
            }
        }
    }

    Err(ErrorResponse::new(
        ErrorResponseType::Unauthorized,
        "you don't have access to this user picture",
    ))
}

/// DELETE the user picture
#[utoipa::path(
    get,
    path = "/users/{user_id}/picture/{picture_id}",
    tag = "users",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[delete("/users/{user_id}/picture/{picture_id}")]
pub async fn delete_user_picture(
    path: web::Path<(String, String)>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    let (user_id, picture_id) = path.into_inner();

    if let Err(err) = principal.validate_user_or_admin(&user_id)
        && principal
            .validate_api_key(AccessGroup::Users, AccessRights::Delete)
            .is_err()
    {
        return Err(err);
    }

    UserPicture::remove(picture_id, user_id.clone()).await?;

    let user = User::find(user_id).await?;
    task::spawn(async move {
        let email = user.email.clone();
        if let Err(err) = ClientScim::create_update_user(user).await {
            error!(
                "Error during SCIM Client user update (delete picture_id) for {}: {:?}",
                email, err
            );
        }
    });

    Ok(HttpResponse::Ok().finish())
}

/// GET all devices for this user linked via the `device_code` flow
#[utoipa::path(
    get,
    path = "/users/{id}/devices",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = [DeviceResponse]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/users/{id}/devices")]
pub async fn get_user_devices(
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    let user_id = path.into_inner();
    principal.validate_user_or_admin(&user_id)?;

    let resp = DeviceEntity::find_for_user(&user_id)
        .await?
        .into_iter()
        .map(DeviceResponse::from)
        .collect::<Vec<DeviceResponse>>();

    Ok(HttpResponse::Ok().json(resp))
}

/// GET all devices for this user linked via the `device_code` flow
#[utoipa::path(
    put,
    path = "/users/{id}/devices",
    tag = "users",
    request_body = DeviceRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/users/{id}/devices")]
pub async fn put_user_device_name(
    path: web::Path<String>,
    principal: ReqPrincipal,
    Json(payload): Json<DeviceRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let user_id = path.into_inner();
    principal.validate_user_or_admin(&user_id)?;
    payload.validate()?;

    if let Some(name) = &payload.name {
        DeviceEntity::update_name(&payload.device_id, &user_id, name).await?;
    }

    Ok(HttpResponse::Ok().finish())
}

/// DELETE all existing refresh tokens for a user device
///
/// This does NOT revoke already existing access tokens, since they are stateless!
#[utoipa::path(
    delete,
    path = "/users/{id}/devices",
    tag = "users",
    request_body = DeviceRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/users/{id}/devices")]
pub async fn delete_user_device(
    path: web::Path<String>,
    principal: ReqPrincipal,
    Json(payload): Json<DeviceRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let user_id = path.into_inner();
    principal.validate_user_or_admin(&user_id)?;
    payload.validate()?;

    let device = DeviceEntity::find(&payload.device_id).await?;
    if device.user_id.as_deref() != Some(&user_id) {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "You don't have access to this device".to_string(),
        ));
    }

    DeviceEntity::revoke_refresh_tokens(&payload.device_id).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Endpoint for resetting passwords
///
/// The `id` is the user id and `reset_id` is a random 64 character long string sent via E-Mail for a
/// pre-authenticated request.
#[utoipa::path(
    get,
    path = "/users/{id}/email_confirm/{confirm_id}",
    tag = "users",
    responses(
        (status = 200, description = "Ok"),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/users/{id}/email_confirm/{confirm_id}")]
pub async fn get_user_email_confirm(
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> HttpResponse {
    let lang = Language::try_from(&req).unwrap_or_default();
    let (user_id, confirm_id) = path.into_inner();
    match User::confirm_email_address(req, user_id, confirm_id).await {
        Ok(html) => HttpResponse::Ok().insert_header(HEADER_HTML).body(html),
        Err(err) => {
            let status = err.status_code();
            let body = Error3Html::build(
                &lang,
                ThemeCssFull::find_theme_ts_rauthy()
                    .await
                    .unwrap_or_else(|_| Utc::now().timestamp()),
                status,
                err.message,
            );
            ErrorHtml::response(body, status)
        }
    }
}

/// Endpoint for resetting passwords
///
/// The `id` is the user id and `reset_id` is a random 64 character long string sent via E-Mail for a
/// pre-authenticated request.
#[utoipa::path(
    get,
    path = "/users/{id}/reset/{reset_id}",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = PasswordResetResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/users/{id}/reset/{reset_id}")]
pub async fn get_user_password_reset(
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> HttpResponse {
    let (user_id, reset_id) = path.into_inner();
    let lang = Language::try_from(&req).unwrap_or_default();
    let accept = req
        .headers()
        .get(ACCEPT)
        .map(|v| v.to_str().unwrap_or("text/html"))
        .unwrap_or("text/html");
    let no_html = accept == "application/json";

    match password_reset::handle_get_pwd_reset(req, user_id, reset_id, no_html).await {
        Ok((content, cookie)) => {
            if no_html {
                let password_policy = match PasswordPolicy::find().await {
                    Ok(policy) => PasswordPolicyResponse::from(policy),
                    Err(err) => {
                        let status = err.status_code();
                        let body = Error3Html::build(
                            &lang,
                            ThemeCssFull::find_theme_ts_rauthy()
                                .await
                                .unwrap_or_else(|_| Utc::now().timestamp()),
                            status,
                            err.message,
                        );
                        return ErrorHtml::response(body, status);
                    }
                };

                HttpResponse::Ok()
                    .cookie(cookie)
                    .insert_header(HEADER_JSON)
                    .json(PasswordResetResponse {
                        csrf_token: content,
                        password_policy,
                    })
            } else {
                HttpResponse::Ok()
                    .cookie(cookie)
                    .insert_header(HEADER_HTML)
                    .body(content)
            }
        }
        Err(err) => {
            debug!("Error during get_user_password_reset: {:?}", err);
            let status = err.status_code();
            let body = Error3Html::build(
                &lang,
                ThemeCssFull::find_theme_ts_rauthy()
                    .await
                    .unwrap_or_else(|_| Utc::now().timestamp()),
                status,
                err.message,
            );
            ErrorHtml::response(body, status)
        }
    }
}

/// Endpoint for resetting passwords
///
/// On this endpoint, a password reset can be submitted. This only works with a valid
/// `PWD_RESET_COOKIE` + CSRF token.
///
/// Expects the CSRF token to be provided with an HTTP Header called `PWD_CSRF_HEADER`
///
/// **Permissions**
/// - pre-authenticated with pwd-reset cookie from `GET /auth/v1/users/{id}/reset/{reset_id}`
#[utoipa::path(
    put,
    path = "/users/{id}/reset",
    tag = "users",
    request_body = PasswordResetRequest,
    responses(
        (status = 202, description = "Accepted"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/users/{id}/reset")]
pub async fn put_user_password_reset(
    path: web::Path<String>,
    req: HttpRequest,
    Json(payload): Json<PasswordResetRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    password_reset::handle_put_user_password_reset(req, path.into_inner(), payload)
        .await
        .map(|(cookie, location)| {
            if let Some(loc) = location {
                HttpResponse::Ok()
                    .insert_header((LOCATION, loc))
                    .cookie(cookie)
                    .status(StatusCode::ACCEPTED)
                    .finish()
            } else {
                HttpResponse::Ok()
                    .cookie(cookie)
                    .status(StatusCode::ACCEPTED)
                    .finish()
            }
        })
}

/// Revoke a "Login from unknown location"
#[utoipa::path(
    get,
    path = "/users/{id}/revoke/{code}",
    tag = "users",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/users/{id}/revoke/{code}")]
pub async fn get_user_revoke(
    path: web::Path<(String, String)>,
    req: HttpRequest,
    Query(params): Query<UserRevokeParams>,
) -> HttpResponse {
    let (user_id, code) = path.into_inner();

    let lang = Language::try_from(&req).unwrap_or_default();
    let theme_ts = ThemeCssFull::find_theme_ts_rauthy()
        .await
        .unwrap_or_default();

    let html = match user_revoke_handle(user_id, code, params.ip).await {
        Ok(_) => UserRevokeHtml::build(&lang, theme_ts),
        Err(err) => {
            debug!("Error during user revoke: {err}");
            Error3Html::build(&lang, theme_ts, err.status_code(), "Code not found")
        }
    };

    HttpResponse::Ok().content_type(TEXT_HTML).body(html)
}

#[inline]
async fn user_revoke_handle(
    user_id: String,
    code: String,
    bad_ip: IpAddr,
) -> Result<(), ErrorResponse> {
    let revoke = UserRevoke::find(user_id).await?;
    if revoke.code != code {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "invalid revoke code",
        ));
    }

    Session::invalidate_for_user(&revoke.user_id).await?;
    RefreshToken::invalidate_for_user(&revoke.user_id).await?;
    LoginLocation::delete_for_user(revoke.user_id.clone()).await?;
    logout::execute_backchannel_logout(None, Some(revoke.user_id.clone())).await?;

    let user = User::find(revoke.user_id).await?;
    UserRevoke::delete(user.id).await?;

    let location = ipgeo::get_location_from_db(bad_ip)?;
    Event::user_login_revoke(&user.email, bad_ip, location)
        .send()
        .await?;

    Ok(())
}

/// Get all registered Webauthn Passkeys for a user
///
/// **Permissions**
/// - authenticated and logged in user for this very {id}
/// - authenticated and logged in admin
#[utoipa::path(
    get,
    path = "/users/{id}/webauthn",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = [PasskeyResponse]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/users/{id}/webauthn")]
pub async fn get_user_webauthn_passkeys(
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    let id = id.into_inner();

    if principal
        .validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Read)
        .is_err()
    {
        // make sure a non-admin can only access its own information
        principal.validate_session_auth()?;
        principal.is_user(&id)?;
    }

    let pks = PasskeyEntity::find_for_user(&id)
        .await?
        .into_iter()
        .map(PasskeyResponse::from)
        .collect::<Vec<PasskeyResponse>>();

    Ok(HttpResponse::Ok().json(pks))
}

/// Starts the authentication process for a WebAuthn Device for this user
///
/// **Permissions**
/// - authenticated and logged in user for this very {id}
#[utoipa::path(
    post,
    path = "/users/{id}/webauthn/auth/start",
    tag = "mfa",
    request_body = WebauthnAuthStartRequest,
    responses(
        (status = 200, description = "Ok", body = WebauthnAuthStartResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/users/{id}/webauthn/auth/start")]
pub async fn post_webauthn_auth_start(
    id: web::Path<String>,
    // The principal here must be optional to make cases like user password reset in a
    // fully new / different browser which does not have any lefter data or cookies
    principal: ReqPrincipal,
    req: HttpRequest,
    Json(payload): Json<WebauthnAuthStartRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let id = match payload.purpose {
        // only for a Login purpose, this can be accessed without authentication (yet)
        MfaPurpose::Login(_) => {
            // During Login, the session is allowed to be in init only state
            principal.validate_session_auth_or_init()?;
            id.into_inner()
        }

        MfaPurpose::PasswordReset => {
            // A password reset webauthn req can be opened without any session at all.
            // This is mandatory to make password reset flows fully work, even with an old
            // account with linked Passkeys.
            match ApiCookie::from_req(&req, PWD_RESET_COOKIE) {
                None => {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "You are not allowed to do this operation without an active binding cookie"
                            .to_string(),
                    ));
                }
                Some(c) => {
                    if c.len() != 48 {
                        return Err(ErrorResponse::new(
                            ErrorResponseType::BadRequest,
                            "Malformed magic link binding cookie".to_string(),
                        ));
                    };
                }
            };

            id.into_inner()
        }

        _ => {
            // for all other purposes, we need an authenticated session
            principal.validate_session_auth()?;

            // make sure the principal is this very user
            let id = id.into_inner();
            principal.is_user(&id)?;
            id
        }
    };

    webauthn::auth_start(id, payload.purpose)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

/// Finishes the authentication process for a WebAuthn Device for this user
///
/// **Permissions**
/// - authenticated and logged in user for this very {id}
#[utoipa::path(
    post,
    path = "/users/{id}/webauthn/auth/finish",
    tag = "mfa",
    request_body = WebauthnAuthFinishRequest,
    responses(
        (status = 200, description = "Ok", body = WebauthnAdditionalData),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/users/{id}/webauthn/auth/finish")]
pub async fn post_webauthn_auth_finish(
    id: web::Path<String>,
    req: HttpRequest,
    browser_id: BrowserId,
    principal: ReqPrincipal,
    Json(payload): Json<WebauthnAuthFinishRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let id = id.into_inner();

    // We do not need to further validate the principal here.
    // All of this is done at the /start endpoint.
    // This here will simply fail, if the secret code from the /start does not exist
    // -> indirect validation through exising code.

    let principal = principal.into_inner();
    let res = webauthn::auth_finish(id, &req, browser_id, principal.session, payload).await?;
    Ok(res.into_response())
}

/// Deletes the WebAuthn Device for this user in the given slot
///
/// **Permissions**
/// - rauthy_admin
/// - authenticated and logged in user for this very {id}
#[utoipa::path(
    delete,
    path = "/users/{id}/webauthn/delete/{name}",
    tag = "mfa",
    request_body = WebauthnDeleteRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/users/{id}/webauthn/delete/{name}")]
pub async fn delete_webauthn(
    path: web::Path<(String, String)>,
    principal: ReqPrincipal,
    req: HttpRequest,
    Json(payload): Json<WebauthnDeleteRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    // Note: Currently, this is not allowed with an ApiKey on purpose
    let is_admin = match principal.validate_admin_session() {
        Ok(()) => true,
        Err(_) => {
            principal.validate_session_auth()?;
            false
        }
    };

    let (id, name) = path.into_inner();

    // validate that Principal matches the user or is an admin
    if !is_admin {
        principal.is_user(&id)?;

        let Some(token_id) = payload.mfa_mod_token_id else {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "missing `mfa_mod_token_id`",
            ));
        };
        let token = MfaModToken::find(&token_id).await?;
        let ip = real_ip_from_req(&req)?;
        token.validate(principal.user_id()?, ip)?;

        warn!("Passkey delete for user {} for key {}", id, name);
    } else {
        warn!("Passkey delete from admin for user {} for key {}", id, name);
    }

    PasskeyEntity::delete(id, name).await?;

    // make sure to delete any existing MFA cookie when a key is deleted
    let cookie = ApiCookie::build(COOKIE_MFA, "", 0);
    let mut resp = HttpResponse::Ok().finish();
    if let Err(err) = resp.add_cookie(&cookie) {
        error!("Error deleting MFA cookie in post_webauthn_delete: {}", err);
    }
    Ok(resp)
}

/// Starts the registration process for a new WebAuthn Device for this user
///
/// **Permissions**
/// - authenticated and logged in user for this very {id}
#[utoipa::path(
    post,
    path = "/users/{id}/webauthn/register/start",
    tag = "mfa",
    request_body = WebauthnRegStartRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/users/{id}/webauthn/register/start")]
pub async fn post_webauthn_reg_start(
    id: web::Path<String>,
    principal: ReqPrincipal,
    req: HttpRequest,
    Json(payload): Json<WebauthnRegStartRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    // If we have a magic link ID in the payload, we do not validate the active session / principal.
    // This is mandatory to make registering a passkey for a completely new account work.
    if payload.magic_link_id.is_some() && req.headers().get(PWD_CSRF_HEADER).is_some() {
        password_reset::handle_put_user_passkey_start(req, id.into_inner(), payload).await
    } else {
        principal.validate_session_auth()?;
        // this endpoint is a CSRF check exception inside the Principal Middleware -> check here!
        principal.validate_session_csrf_exception(&req)?;

        if payload.mfa_mod_token_id.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "missing `mfa_mod_token_id`",
            ));
        }
        let mod_token = MfaModToken::find(payload.mfa_mod_token_id.as_ref().unwrap()).await?;
        let ip = real_ip_from_req(&req)?;
        mod_token.validate(principal.user_id()?, ip)?;

        // validate that Principal matches the user
        let id = id.into_inner();
        principal.is_user(&id)?;

        webauthn::reg_start(id, payload)
            .await
            .map(|ccr| HttpResponse::Ok().json(ccr))
    }
}

/// Finishes the registration process for a new WebAuthn Device for this user
///
/// **Permissions**
/// - authenticated and logged-in user for this very {id}
/// - valid MagicLink code during password reset
#[utoipa::path(
    post,
    path = "/users/{id}/webauthn/register/finish",
    tag = "mfa",
    request_body = WebauthnRegFinishRequest,
    responses(
        (status = 201, description = "Created"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/users/{id}/webauthn/register/finish")]
pub async fn post_webauthn_reg_finish(
    id: web::Path<String>,
    principal: ReqPrincipal,
    req: HttpRequest,
    Json(payload): Json<WebauthnRegFinishRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    // If we have a magic link ID in the payload, we do not validate the active session / principal.
    // This is mandatory to make registering a passkey for a completely new account work.
    if payload.magic_link_id.is_some() {
        password_reset::handle_put_user_passkey_finish(req, id.into_inner(), payload).await
    } else {
        principal.validate_session_auth()?;
        // this endpoint is a CSRF check exception inside the Principal Middleware -> check here!
        principal.validate_session_csrf_exception(&req)?;

        // validate that Principal matches the user
        let id = id.into_inner();
        principal.is_user(&id)?;

        webauthn::reg_finish(id, payload).await?;
        Ok(HttpResponse::Created().finish())
    }
}

/// Returns a user's webid document, if enabled
///
/// Note: The way the SwaggerUI is set up currently, the path will not be correct for this single
/// endpoint. It is an absolute path: `/auth/webid/{id}/profile`
#[utoipa::path(
    get,
    path = "/auth/{id}/profile",
    tag = "webid",
    responses(
        (status = 200, description = "Ok", body = WebIdResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
        (status = 405, description = "MethodNotAllowed"),
    ),
)]
#[get("/{id}/profile")]
pub async fn get_user_webid(id: web::Path<String>) -> Result<HttpResponse, ErrorResponse> {
    // check if webid's are enabled globally
    if !RauthyConfig::get().vars.ephemeral_clients.enable_web_id {
        return Ok(HttpResponse::MethodNotAllowed().finish());
    }

    let id = id.into_inner();
    let webid = WebId::find(id).await?;
    let user = User::find(webid.user_id.clone()).await?;

    let resp = WebIdResponse {
        webid: webid.into(),
        issuer: RauthyConfig::get().issuer.clone(),
        email: user.email,
        given_name: if user.given_name.is_empty() {
            None
        } else {
            Some(user.given_name)
        },
        family_name: user.family_name,
        language: user.language.into(),
    };

    WebId::try_into_turtle(resp)
        .map(|content| HttpResponse::Ok().content_type(TEXT_TURTLE).body(content))
}

/// Returns data and options set by the user for the `webid` preferences
#[utoipa::path(
    get,
    path = "/users/{id}/webid/data",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = WebId),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
        (status = 405, description = "MethodNotAllowed"),
    ),
)]
#[get("/users/{id}/webid/data")]
pub async fn get_user_webid_data(
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    // check if webid's are enabled globally
    if !RauthyConfig::get().vars.ephemeral_clients.enable_web_id {
        return Ok(HttpResponse::MethodNotAllowed().finish());
    }

    // check auth
    principal.validate_session_auth()?;

    let id = id.into_inner();
    if principal.is_user(&id).is_err() {
        // if the user id does not match, check if the principal is an admin
        principal.validate_admin_session()?;
    }

    // request is valid -> either the user requests own data, or it is an admin
    let webid = WebId::find(id).await?;

    Ok(HttpResponse::Ok().json(webid))
}

/// Returns data and options set by the user for the `webid` preferences. Data must be serialized
/// in ntriples.
#[utoipa::path(
    put,
    path = "/users/{id}/webid/data",
    tag = "users",
    request_body = WebIdRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 405, description = "MethodNotAllowed"),
    ),
)]
#[put("/users/{id}/webid/data")]
pub async fn put_user_webid_data(
    id: web::Path<String>,
    principal: ReqPrincipal,
    Json(payload): Json<WebIdRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let vars = &RauthyConfig::get().vars;
    if !vars.ephemeral_clients.enable_web_id {
        return Ok(HttpResponse::MethodNotAllowed().finish());
    }
    payload.validate()?;

    // check auth
    principal.validate_session_auth()?;
    let id = id.into_inner();
    principal.is_user(&id)?;

    let web_id = WebId::try_new(
        id,
        payload.custom_triples.as_deref(),
        payload.expose_email,
        vars.server.pub_url.as_str(),
    )
    .map_err(|e| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Invalid custom data. {e}"),
        )
    })?;

    WebId::upsert(web_id).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Request a password reset
///
/// This Endpoint will always return an `OK` to not provide any additional attack surface.
/// Only if the provided E-Mail exists in the Database, a password reset E-Mail will be sent out,
/// otherwise it will just be ignored but still return an `OK`.
#[utoipa::path(
    post,
    path = "/users/request_reset",
    tag = "users",
    request_body = RequestResetRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
    ),
)]
#[post("/users/request_reset")]
pub async fn post_user_password_request_reset(
    req: HttpRequest,
    Json(payload): Json<RequestResetRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    info!(
        "Password reset request for '{}' from IP {}",
        payload.email,
        real_ip_from_req(&req)?
    );
    let challenge = Pow::validate(&payload.pow)?;
    PowEntity::check_prevent_reuse(challenge.to_string()).await?;

    match User::find_by_email(payload.email).await {
        Ok(user) => user
            .request_password_reset(payload.redirect_uri)
            .await
            .map(|_| HttpResponse::Ok().status(StatusCode::OK).finish()),
        Err(_) => {
            // always return OK, no matter what, for username enumeration prevention
            Ok(HttpResponse::Ok().status(StatusCode::OK).finish())
        }
    }
}

/// Returns a single user by its *email*
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/users/email/{email}",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/users/email/{email}")]
pub async fn get_user_by_email(
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Read)?;

    let user = User::find_by_email(path.into_inner()).await?;
    let values = UserValues::find(&user.id).await?;

    Ok(HttpResponse::Ok().json(user.into_response(values)))
}

/// Modifies a user
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/users/{id}",
    tag = "users",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Ok", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/users/{id}")]
pub async fn put_user_by_id(
    id: web::Path<String>,
    req: HttpRequest,
    principal: ReqPrincipal,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Update)?;
    payload.validate()?;

    UserValuesValidator {
        given_name: payload.given_name.as_deref(),
        family_name: payload.family_name.as_deref(),
        // has its own endpoint for updates
        preferred_username: None,
        user_values: &payload.user_values,
    }
    .validate()?;

    let preferred_username = UserValues::find_preferred_username(&id).await?;
    handle_put_user_by_id(id.into_inner(), req, payload, preferred_username).await
}

/// Modifies a user via a patch operation
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    patch,
    path = "/users/{id}",
    tag = "users",
    request_body = PatchOp,
    responses(
        (status = 200, description = "Ok", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[patch("/users/{id}")]
pub async fn patch_user(
    id: web::Path<String>,
    req: HttpRequest,
    principal: ReqPrincipal,
    Json(payload): Json<PatchOp>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Update)?;

    let user_id = id.into_inner();
    let (upd_req, has_preferred_username) = User::patch(user_id.clone(), payload).await?;
    upd_req.validate()?;

    handle_put_user_by_id(user_id, req, upd_req, has_preferred_username).await
}

#[inline]
async fn handle_put_user_by_id(
    user_id: String,
    req: HttpRequest,
    payload: UpdateUserRequest,
    preferred_username: Option<String>,
) -> Result<HttpResponse, ErrorResponse> {
    let (user, user_values, is_new_admin) =
        User::update(user_id, payload, None, preferred_username).await?;

    if is_new_admin {
        RauthyConfig::get()
            .tx_events
            .send_async(Event::new_rauthy_admin(
                user.email.clone(),
                real_ip_from_req(&req)?.to_string(),
            ))
            .await
            .unwrap();
    }

    let cloned = user.clone();
    task::spawn(async move {
        let email = cloned.email.clone();
        if let Err(err) = ClientScim::create_update_user(cloned).await {
            error!(
                "Error during SCIM Client user update for {}: {:?}",
                email, err
            );
        }
    });

    Ok(HttpResponse::Ok().json(user.into_response(user_values)))
}

/// Allows modification of specific user values from the user himself
///
/// **Permissions**
/// - authenticated user
#[utoipa::path(
    put,
    path = "/users/{id}/self",
    tag = "users",
    request_body = UpdateUserSelfRequest,
    responses(
        (status = 200, description = "Ok", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/users/{id}/self")]
pub async fn put_user_self(
    id: web::Path<String>,
    principal: ReqPrincipal,
    Json(payload): Json<UpdateUserSelfRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;
    payload.validate()?;

    // make sure the logged-in user can only update itself
    let id = id.into_inner();
    principal.is_user(&id)?;

    let (user, user_values, email_updated) = User::update_self_req(id, payload).await?;

    let cloned = user.clone();
    task::spawn(async move {
        let email = cloned.email.clone();
        if let Err(err) = ClientScim::create_update_user(cloned).await {
            error!(
                "Error during SCIM Client user update (self) for {}: {:?}",
                email, err
            );
        }
    });

    if email_updated {
        Ok(HttpResponse::Accepted().json(user.into_response(user_values)))
    } else {
        Ok(HttpResponse::Ok().json(user.into_response(user_values)))
    }
}

/// Check if user self-deletion is allowed.
///
/// Self-deletion is forbidden, if the user is assigned to the `rauthy_admin` rule as part of the
/// anti-lockout rules.
///
/// **Permissions**
/// - authenticated user
#[utoipa::path(
    get,
    path = "/users/{id}/self/delete",
    tag = "users",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 406, description = "NotAccepted", body = ErrorResponse),
    ),
)]
#[get("/users/{id}/self/delete")]
pub async fn get_user_self_delete_config(
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    if !RauthyConfig::get().vars.user_delete.enable_self_delete {
        return Err(ErrorResponse::new(
            ErrorResponseType::NotAccepted,
            "User self-delete is disabled",
        ));
    }

    // make sure the logged-in user can only delete itself
    let id = id.into_inner();
    principal.is_user(&id)?;

    let user = User::find(id).await?;
    if user.roles_iter().any(|r| r == "rauthy_admin") {
        Err(ErrorResponse::new(
            ErrorResponseType::NotAccepted,
            "A `rauthy_admin` cannot be self-deleted",
        ))
    } else {
        Ok(HttpResponse::Accepted().finish())
    }
}

/// DELETE for a user by self-service, if enabled
///
/// This endpoint always deletes this very user the session is valid for.
///
/// **Permissions**
/// - authenticated user
#[utoipa::path(
    delete,
    path = "/users/{id}/self/delete",
    tag = "users",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 406, description = "NotAccepted", body = ErrorResponse),
    ),
)]
#[delete("/users/{id}/self/delete")]
pub async fn delete_user_self(
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    if !RauthyConfig::get().vars.user_delete.enable_self_delete {
        return Err(ErrorResponse::new(
            ErrorResponseType::NotAccepted,
            "User self-delete is disabled",
        ));
    }

    // make sure the logged-in user can only delete itself
    let id = id.into_inner();
    principal.is_user(&id)?;

    let user = User::find(id).await?;
    if user.roles_iter().any(|r| r == "rauthy_admin") {
        return Err(ErrorResponse::new(
            ErrorResponseType::NotAccepted,
            "A `rauthy_admin` cannot be self-deleted",
        ));
    }

    handle_user_delete(user).await
}

async fn handle_user_delete(user: User) -> Result<HttpResponse, ErrorResponse> {
    logout::execute_backchannel_logout(None, Some(user.id.clone())).await?;

    // sessions and other tables have a cascading FK to the users table
    user.delete().await?;

    let clients_scim = ClientScim::find_all().await?;
    task::spawn(async move {
        let email = user.email.clone();

        for client_scim in clients_scim {
            if let Err(err) = client_scim.delete_user(&user).await {
                error!(
                    "Error during SCIM Client user delete for {}: {:?}",
                    email, err
                );
            }
        }
    });

    Ok(HttpResponse::NoContent().finish())
}

/// Allows an authenticated and logged-in user to convert his account to passkey only
///
/// **Permissions**
/// - authenticated user
#[utoipa::path(
    post,
    path = "/users/{id}/self/convert_passkey",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/users/{id}/self/convert_passkey")]
pub async fn post_user_self_convert_passkey(
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    let id = id.into_inner();
    principal.validate_user_session(&id)?;

    User::convert_to_passkey(id).await?;
    Ok(HttpResponse::Ok().finish())
}

/// Retrieve the UserValues config.
///
/// This is the same config as the one being inserted into the HTML `<template>` during registration
/// or user values editing.
#[utoipa::path(
    get,
    path = "/users/values_config",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = VarsUserValuesConfig),
    ),
)]
#[get("/users/values_config")]
pub async fn get_user_values_config(
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    // There is no need to validate session or API key if the registration is open anyway.
    // In this case, anyone can pull out the same information from the HTML.
    if !RauthyConfig::get().vars.user_registration.enable {
        principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Read)?;
    }
    Ok(HttpResponse::Ok().json(&RauthyConfig::get().vars.user_values))
}

/// Allows modification of specific user values from the user himself
///
/// **Permissions**
/// - authenticated user
#[utoipa::path(
    put,
    path = "/users/{id}/self/preferred_username",
    tag = "users",
    request_body = PreferredUsernameRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 406, description = "NotAccepted", body = ErrorResponse),
    ),
)]
#[put("/users/{id}/self/preferred_username")]
pub async fn put_user_self_preferred_username(
    id: web::Path<String>,
    principal: ReqPrincipal,
    payload: Json<PreferredUsernameRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let id = id.into_inner();
    let payload = payload.into_inner();
    let force_overwrite = payload.force_overwrite == Some(true);

    if principal
        .validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Update)
        .is_err()
    {
        // make sure the logged-in, non-admin can only update its own username
        principal.validate_user_session(&id)?;

        if force_overwrite {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Only an admin can 'force_overwrite' the 'preferred_username'",
            ));
        }
    }

    let config = &RauthyConfig::get().vars.user_values.preferred_username;
    let is_empty =
        payload.preferred_username.is_none() || payload.preferred_username.as_deref() == Some("");

    if is_empty {
        if config.preferred_username == UserValueConfigValue::Required {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "The 'preferred_username' is required and must not be empty",
            ));
        }
    } else if config.blacklist.contains(
        &payload
            .preferred_username
            .as_ref()
            .unwrap()
            .to_lowercase()
            .into(),
    ) {
        return Err(ErrorResponse::new(
            ErrorResponseType::NotAccepted,
            format!(
                "username '{}' is blacklisted",
                payload.preferred_username.as_ref().unwrap()
            ),
        ));
    }

    if !force_overwrite
        && config.immutable
        && let Some(values) = UserValues::find(&id).await?
        && values.preferred_username.is_some()
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "The 'preferred_username' is immutable",
        ));
    }

    if is_empty {
        UserValues::delete_preferred_username(id).await?;
    } else {
        UserValues::upsert_preferred_username(id, payload.preferred_username.unwrap()).await?;
    }

    Ok(HttpResponse::Ok().finish())
}

/// Deletes a user
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
    responses(
        (status = 204, description = "NoContent"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/users/{id}")]
pub async fn delete_user_by_id(
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Delete)?;
    let user = User::find(id.into_inner()).await?;
    handle_user_delete(user).await
}
