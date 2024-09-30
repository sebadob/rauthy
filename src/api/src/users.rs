use crate::ReqPrincipal;
use actix_web::http::header::{ACCEPT, LOCATION};
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, ResponseError};
use actix_web_validator::{Json, Query};
use rauthy_api_types::generic::{PaginationParams, PasswordPolicyResponse};
use rauthy_api_types::oidc::PasswordResetResponse;
use rauthy_api_types::users::{
    DeviceRequest, DeviceResponse, MfaPurpose, NewUserRegistrationRequest, NewUserRequest,
    PasskeyResponse, PasswordResetRequest, RequestResetRequest, UpdateUserRequest,
    UpdateUserSelfRequest, UserAttrConfigRequest, UserAttrConfigResponse, UserAttrValueResponse,
    UserAttrValuesResponse, UserAttrValuesUpdateRequest, WebIdRequest, WebIdResponse,
    WebauthnAuthFinishRequest, WebauthnAuthStartRequest, WebauthnRegFinishRequest,
    WebauthnRegStartRequest,
};
use rauthy_common::constants::{
    COOKIE_MFA, ENABLE_WEB_ID, HEADER_ALLOW_ALL_ORIGINS, HEADER_HTML, HEADER_JSON, OPEN_USER_REG,
    PWD_RESET_COOKIE, SSP_THRESHOLD, TEXT_TURTLE, USER_REG_DOMAIN_BLACKLIST,
    USER_REG_DOMAIN_RESTRICTION,
};
use rauthy_common::utils::real_ip_from_req;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::api_cookie::ApiCookie;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::entity::continuation_token::ContinuationToken;
use rauthy_models::entity::devices::DeviceEntity;
use rauthy_models::entity::password::PasswordPolicy;
use rauthy_models::entity::pow::PowEntity;
use rauthy_models::entity::user_attr::{UserAttrConfigEntity, UserAttrValueEntity};
use rauthy_models::entity::users::User;
use rauthy_models::entity::users_values::UserValues;
use rauthy_models::entity::webauthn;
use rauthy_models::entity::webauthn::PasskeyEntity;
use rauthy_models::entity::webids::WebId;
use rauthy_models::events::event::Event;
use rauthy_models::language::Language;
use rauthy_models::templates::{Error1Html, Error3Html, ErrorHtml, UserRegisterHtml};
use rauthy_service::password_reset;
use spow::pow::Pow;
use std::ops::Add;
use time::OffsetDateTime;
use tracing::{error, warn};

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
        (status = 200, description = "Ok", body = [UserResponse]),
        (status = 206, description = "PartialContent", body = [UserResponse]),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/users")]
pub async fn get_users(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
    params: Query<PaginationParams>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Read)?;

    let user_count = User::count(&data).await?;

    if user_count >= *SSP_THRESHOLD as i64 || params.page_size.is_some() {
        let page_size = params.page_size.unwrap_or(15) as i64;
        let offset = params.offset.unwrap_or(0) as i64;
        let backwards = params.backwards.unwrap_or(false);
        let continuation_token = if let Some(token) = &params.continuation_token {
            Some(ContinuationToken::try_from(token.as_str())?)
        } else {
            None
        };

        let (users, continuation_token) =
            User::find_paginated(&data, continuation_token, page_size, offset, backwards).await?;
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
        let users = User::find_all_simple(&data).await?;
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
    data: web::Data<AppState>,
    req: HttpRequest,
    principal: ReqPrincipal,
    user: Json<NewUserRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Create)?;

    let user = User::create_from_new(&data, user.into_inner()).await?;

    data.tx_events
        .send_async(Event::new_user(
            user.email.clone(),
            real_ip_from_req(&req)?.to_string(),
        ))
        .await
        .unwrap();
    if user.is_admin() {
        data.tx_events
            .send_async(Event::new_rauthy_admin(
                user.email.clone(),
                real_ip_from_req(&req)?.to_string(),
            ))
            .await
            .unwrap();
    }

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
pub async fn get_cust_attr(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Read)?;

    UserAttrConfigEntity::find_all(&data).await.map(|values| {
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
        (status = 200, description = "Ok"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/users/attr")]
pub async fn post_cust_attr(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
    req_data: Json<UserAttrConfigRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal
        .validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Create)?;

    UserAttrConfigEntity::create(&data, req_data.into_inner())
        .await
        .map(|attr| HttpResponse::Ok().json(attr))
}

/// Update an additional custom user attribute
///
/// The `name` of a custom attribute cannot be updated, only the description.
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
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
    req_data: Json<UserAttrConfigRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal
        .validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Update)?;

    UserAttrConfigEntity::update(&data, path.into_inner(), req_data.into_inner())
        .await
        .map(|a| HttpResponse::Ok().json(a))
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
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal
        .validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Delete)?;

    UserAttrConfigEntity::delete(&data, path.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
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
pub async fn get_users_register(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy(&data).await?;
    let lang = Language::try_from(&req).unwrap_or_default();

    if !*OPEN_USER_REG {
        let status = StatusCode::NOT_FOUND;
        let body = Error1Html::build(
            &colors,
            &lang,
            status,
            Some("Open User Registration is disabled".to_string()),
        );
        return Ok(ErrorHtml::response(body, status));
    }

    let body = UserRegisterHtml::build(&colors, &lang);
    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
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
    data: web::Data<AppState>,
    req: HttpRequest,
    req_data: Json<NewUserRegistrationRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    if !*OPEN_USER_REG {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Open User Registration is not allowed".to_string(),
        ));
    }
    if let Some(restriction) = &*USER_REG_DOMAIN_RESTRICTION {
        if !req_data.email.ends_with(restriction) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "Domain for the open registration are restricted to '@{}'",
                    restriction
                ),
            ));
        }
    } else if let Some(blacklist) = &*USER_REG_DOMAIN_BLACKLIST {
        for blacklisted in blacklist {
            if req_data.email.ends_with(blacklisted) {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Domain is blacklisted",
                ));
            }
        }
    }

    // validate the PoW
    let challenge = Pow::validate(&req_data.pow)?;
    PowEntity::check_prevent_reuse(challenge.to_string()).await?;

    let lang = Language::try_from(&req).unwrap_or_default();
    let user = User::create_from_reg(&data, req_data.into_inner(), lang).await?;

    data.tx_events
        .send_async(Event::new_user(
            user.email,
            real_ip_from_req(&req)?.to_string(),
        ))
        .await
        .unwrap();

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
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let id = path.into_inner();
    // principal must either be an admin or have the same user id
    let api_key_or_admin = principal
        .validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Read)
        .is_ok();
    if !api_key_or_admin {
        principal.is_user(&id)?;
    }

    let user = User::find(&data, id).await?;
    let values = UserValues::find(&data, &user.id).await?;

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
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Read)?;

    let values = UserAttrValueEntity::find_for_user(&data, &path.into_inner())
        .await?
        .drain(..)
        .map(UserAttrValueResponse::from)
        .collect::<Vec<UserAttrValueResponse>>();

    Ok(HttpResponse::Ok().json(UserAttrValuesResponse { values }))
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
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
    req_data: Json<UserAttrValuesUpdateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal
        .validate_api_key_or_admin_session(AccessGroup::UserAttributes, AccessRights::Update)?;

    let values =
        UserAttrValueEntity::update_for_user(&data, &path.into_inner(), req_data.into_inner())
            .await?
            .drain(..)
            .map(UserAttrValueResponse::from)
            .collect::<Vec<UserAttrValueResponse>>();
    Ok(HttpResponse::Ok().json(UserAttrValuesResponse { values }))
}

/// GET all devices for this user linked via the `device_code` flow
#[utoipa::path(
    get,
    path = "/users/{id}/devices",
    tag = "users",
    responses(
        (status = 200, description = "Ok", body = DeviceResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/users/{id}/devices")]
pub async fn get_user_devices(
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    let user_id = path.into_inner();
    principal.validate_user_or_admin(&user_id)?;

    let resp = DeviceEntity::find_for_user(&data, &user_id)
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
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/users/{id}/devices")]
pub async fn put_user_device_name(
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
    payload: actix_web_validator::Json<DeviceRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let user_id = path.into_inner();
    principal.validate_user_or_admin(&user_id)?;

    let payload = payload.into_inner();
    if let Some(name) = &payload.name {
        DeviceEntity::update_name(&data, &payload.device_id, &user_id, name).await?;
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
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/users/{id}/devices")]
pub async fn delete_user_device(
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
    payload: actix_web_validator::Json<DeviceRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let user_id = path.into_inner();
    principal.validate_user_or_admin(&user_id)?;

    let payload = payload.into_inner();
    let device = DeviceEntity::find(&data, &payload.device_id).await?;
    if device.user_id.as_deref() != Some(&user_id) {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "You don't have access to this device".to_string(),
        ));
    }

    DeviceEntity::revoke_refresh_tokens(&data, &payload.device_id).await?;

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
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> HttpResponse {
    let lang = Language::try_from(&req).unwrap_or_default();
    let (user_id, confirm_id) = path.into_inner();
    match User::confirm_email_address(&data, req, user_id, confirm_id).await {
        Ok(html) => HttpResponse::Ok().insert_header(HEADER_HTML).body(html),
        Err(err) => {
            let colors = ColorEntity::find_rauthy(&data).await.unwrap_or_default();
            let status = err.status_code();
            let body = Error3Html::build(&colors, &lang, status, Some(err.message));
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
    data: web::Data<AppState>,
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

    match password_reset::handle_get_pwd_reset(&data, req, user_id, reset_id, no_html).await {
        Ok((content, cookie)) => {
            if no_html {
                let password_policy = match PasswordPolicy::find(&data).await {
                    Ok(policy) => PasswordPolicyResponse::from(policy),
                    Err(err) => {
                        let colors = ColorEntity::find_rauthy(&data).await.unwrap_or_default();
                        let status = err.status_code();
                        let body = Error3Html::build(&colors, &lang, status, Some(err.message));
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
            let colors = ColorEntity::find_rauthy(&data).await.unwrap_or_default();
            let status = err.status_code();
            let body = Error3Html::build(&colors, &lang, status, Some(err.message));
            ErrorHtml::response(body, status)
        }
    }
}

/// Endpoint for resetting passwords
///
/// On this endpoint, a password reset can be posted. This only works with a valid
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
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
    req_data: Json<PasswordResetRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    password_reset::handle_put_user_password_reset(
        &data,
        req,
        path.into_inner(),
        req_data.into_inner(),
    )
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
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/users/{id}/webauthn")]
pub async fn get_user_webauthn_passkeys(
    data: web::Data<AppState>,
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

    let pks = PasskeyEntity::find_for_user(&data, &id)
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
    data: web::Data<AppState>,
    id: web::Path<String>,
    // The principal here must be optional to make cases like user password reset in a
    // fully new / different browser which does not have any lefter data or cookies
    principal: ReqPrincipal,
    req: HttpRequest,
    req_data: Json<WebauthnAuthStartRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let purpose = req_data.into_inner().purpose;
    let id = match purpose {
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

    webauthn::auth_start(&data, id, purpose)
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
    data: web::Data<AppState>,
    id: web::Path<String>,
    req_data: Json<WebauthnAuthFinishRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let id = id.into_inner();

    // We do not need to further validate the principal here.
    // All of this is done at the /start endpoint.
    // This here will simply fail, if the secret code from the /start does not exist.

    let res = webauthn::auth_finish(&data, id, req_data.into_inner()).await?;
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
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/users/{id}/webauthn/delete/{name}")]
pub async fn delete_webauthn(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
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
        warn!("Passkey delete for user {} for key {}", id, name);
    } else {
        warn!("Passkey delete from admin for user {} for key {}", id, name);
    }

    // if we delete a passkey, we must check if this is the last existing one for the user
    let pks = PasskeyEntity::find_for_user(&data, &id).await?;

    let mut txn = data.db.begin().await?;

    PasskeyEntity::delete_by_id_name(&data, &id, &name, Some(&mut txn)).await?;
    if pks.len() < 2 {
        let mut user = User::find(&data, id).await?;
        user.webauthn_user_id = None;

        // in this case, we need to check against the current password policy, if the password
        // should expire again
        let policy = PasswordPolicy::find(&data).await?;
        if let Some(valid_days) = policy.valid_days {
            if user.password.is_some() {
                user.password_expires = Some(
                    OffsetDateTime::now_utc()
                        .add(time::Duration::days(valid_days as i64))
                        .unix_timestamp(),
                );
            } else {
                user.password_expires = None;
            }
        }

        user.save(&data, None, Some(&mut txn)).await?;
        txn.commit().await?;
    } else {
        txn.commit().await?;
    }

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
        (status = 200, description = "Ok - Returns a default Webauthn CreationChallengeResponse, which cannot be serialized into OpenAPI schema currently"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/users/{id}/webauthn/register/start")]
pub async fn post_webauthn_reg_start(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
    req: HttpRequest,
    req_data: Json<WebauthnRegStartRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    // If we have a magic link ID in the payload, we do not validate the active session / principal.
    // This is mandatory to make registering a passkey for a completely new account work.
    if req_data.magic_link_id.is_some() && req_data.email.is_some() {
        password_reset::handle_put_user_passkey_start(
            &data,
            req,
            id.into_inner(),
            req_data.into_inner(),
        )
        .await
    } else {
        principal.validate_session_auth()?;
        // this endpoint is a CSRF check exception inside the Principal Middleware -> check here!
        principal.validate_session_csrf_exception(&req)?;

        // validate that Principal matches the user
        let id = id.into_inner();
        principal.is_user(&id)?;

        webauthn::reg_start(&data, id, req_data.into_inner())
            .await
            .map(|ccr| HttpResponse::Ok().json(ccr))
    }
}

/// Finishes the registration process for a new WebAuthn Device for this user
///
/// **Permissions**
/// - authenticated and logged in user for this very {id}
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
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
    req: HttpRequest,
    req_data: Json<WebauthnRegFinishRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    // If we have a magic link ID in the payload, we do not validate the active session / principal.
    // This is mandatory to make registering a passkey for a completely new account work.
    if req_data.magic_link_id.is_some() {
        password_reset::handle_put_user_passkey_finish(
            &data,
            req,
            id.into_inner(),
            req_data.into_inner(),
        )
        .await
    } else {
        principal.validate_session_auth()?;
        // this endpoint is a CSRF check exception inside the Principal Middleware -> check here!
        principal.validate_session_csrf_exception(&req)?;

        // validate that Principal matches the user
        let id = id.into_inner();
        principal.is_user(&id)?;

        webauthn::reg_finish(&data, id, req_data.into_inner()).await?;
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
pub async fn get_user_webid(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    // check if webid's are enabled globally
    if !*ENABLE_WEB_ID {
        return Ok(HttpResponse::MethodNotAllowed().finish());
    }

    let id = id.into_inner();
    let webid = WebId::find(&data, id).await?;
    let user = User::find(&data, webid.user_id.clone()).await?;

    let resp = WebIdResponse {
        webid: webid.into(),
        issuer: data.issuer.clone(),
        email: user.email,
        given_name: user.given_name,
        family_name: user.family_name,
        language: user.language.into(),
    };

    WebId::into_turtle(resp)
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
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    // check if webid's are enabled globally
    if !*ENABLE_WEB_ID {
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
    let webid = WebId::find(&data, id).await?;

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
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
    payload: Json<WebIdRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    // check if webid's are enabled globally
    if !*ENABLE_WEB_ID {
        return Ok(HttpResponse::MethodNotAllowed().finish());
    }

    // check auth
    principal.validate_session_auth()?;
    let id = id.into_inner();
    principal.is_user(&id)?;

    let payload = payload.into_inner();
    let web_id = WebId::try_new(id, payload.custom_triples.as_deref(), payload.expose_email)
        .map_err(|e| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Invalid custom data. {}", e),
            )
        })?;

    WebId::upsert(&data, web_id).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Request a password reset
///
/// This Endpoint will always return an `OK` to not provide any additional attack surface.
/// Only if the provided E-Mail exists in the Database, a password reset E-Mail will be sent out,
/// otherwise it will just be ignored but still return an `OK`.
///
/// **Permissions**
/// - authenticated
/// - session-init
/// - session-auth
#[utoipa::path(
    post,
    path = "/users/request_reset",
    tag = "users",
    request_body = RequestResetRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/users/request_reset")]
pub async fn post_user_password_request_reset(
    data: web::Data<AppState>,
    req: HttpRequest,
    payload: Json<RequestResetRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;

    let payload = payload.into_inner();
    match User::find_by_email(&data, payload.email).await {
        Ok(user) => user
            .request_password_reset(&data, req, payload.redirect_uri)
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
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Read)?;

    let user = User::find_by_email(&data, path.into_inner()).await?;
    let values = UserValues::find(&data, &user.id).await?;

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
    data: web::Data<AppState>,
    id: web::Path<String>,
    req: HttpRequest,
    principal: ReqPrincipal,
    user: Json<UpdateUserRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Update)?;

    let (user, user_values, is_new_admin) =
        User::update(&data, id.into_inner(), user.into_inner(), None).await?;

    if is_new_admin {
        data.tx_events
            .send_async(Event::new_rauthy_admin(
                user.email.clone(),
                real_ip_from_req(&req)?.to_string(),
            ))
            .await
            .unwrap();
    }

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
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
    user: Json<UpdateUserSelfRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    // make sure the logged in user can only update itself
    let id = id.into_inner();
    principal.is_user(&id)?;

    let (user, user_values, email_updated) =
        User::update_self_req(&data, id, user.into_inner()).await?;
    if email_updated {
        Ok(HttpResponse::Accepted().json(user.into_response(user_values)))
    } else {
        Ok(HttpResponse::Ok().json(user.into_response(user_values)))
    }
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
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    // make sure the logged in user can only update itself
    let id = id.into_inner();
    principal.is_user(&id)?;

    User::convert_to_passkey(&data, id).await?;
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
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Users, AccessRights::Delete)?;

    let user = User::find(&data, path.into_inner()).await?;
    user.delete(&data).await?;
    Ok(HttpResponse::NoContent().finish())
}
