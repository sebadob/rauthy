use crate::{build_csp_header, Assets};
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::{get, post, put, web, HttpRequest, HttpResponse, Responder};
use actix_web_grants::proc_macro::{has_any_permission, has_permissions, has_roles};
use rauthy_common::constants::{CACHE_NAME_LOGIN_DELAY, HEADER_HTML, IDX_LOGIN_TIME};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::entity::password::{PasswordHashTimes, PasswordPolicy};
use rauthy_models::entity::pow::Pow;
use rauthy_models::entity::principal::Principal;
use rauthy_models::entity::sessions::Session;
use rauthy_models::request::{
    EncKeyMigrateRequest, PasswordHashTimesRequest, PasswordPolicyRequest,
};
use rauthy_models::response::{
    Argon2ParamsResponse, EncKeysResponse, HealthResponse, LoginTimeResponse,
    PasswordPolicyResponse,
};
use rauthy_models::templates::{AccountHtml, AdminHtml, IndexHtml};
use rauthy_service::encryption;
use redhac::{cache_get, cache_get_from, cache_get_value};
use std::borrow::Cow;

#[get("/")]
#[has_permissions("all")]
pub async fn get_index(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy(&data).await?;
    let (body, nonce) = IndexHtml::build(&colors);

    Ok(HttpResponse::Ok()
        .insert_header(HEADER_HTML)
        .insert_header(build_csp_header(&nonce))
        .body(body))
}

#[get("/{_:.*}")]
#[has_permissions("all")]
pub async fn get_static_assets(
    path: web::Path<String>,
    accept_encoding: web::Header<header::AcceptEncoding>,
) -> HttpResponse {
    let path = path.into_inner();
    let accept_encoding = accept_encoding.into_inner();

    let mime = mime_guess::from_path(&path);
    let (p, encoding) = if accept_encoding.contains(&"br".parse().unwrap()) {
        (Cow::from(format!("{}.br", path)), "br")
    } else if accept_encoding.contains(&"gzip".parse().unwrap()) {
        (Cow::from(format!("{}.gz", path)), "gzip")
    } else {
        (Cow::from(path), "none")
    };

    match Assets::get(p.as_ref()) {
        Some(content) => HttpResponse::Ok()
            .insert_header(("cache-control", "max-age=2592000"))
            .insert_header(("content-encoding", encoding))
            .content_type(mime.first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/account")]
#[has_permissions("all")]
pub async fn get_account_html(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy(&data).await?;
    let (body, nonce) = AccountHtml::build(&colors);

    Ok(HttpResponse::Ok()
        .insert_header(HEADER_HTML)
        .insert_header(build_csp_header(&nonce))
        .body(body))
}

#[get("/admin")]
#[has_permissions("all")]
pub async fn get_admin_html(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy(&data).await?;
    let (body, nonce) = AdminHtml::build(&colors);

    Ok(HttpResponse::Ok()
        .insert_header(HEADER_HTML)
        .insert_header(build_csp_header(&nonce))
        .body(body))
}

/// Check if the given JWT Token is valid
///
/// This endpoint only exists for checking if the User accessing it has a valid token / session or
/// not.<br>
/// This could be used for instance for [Traefiks](https://traefik.io/traefik/) ForwardAuth middleware.
#[utoipa::path(
    get,
    path = "/auth_check",
    tag = "generic",
    responses(
        (status = 200, description = "Valid Token / Session"),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[get("/auth_check")]
#[has_any_permission("token-auth", "session-auth")]
pub async fn get_auth_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Check if access to the rauthy admin API is allowed
#[utoipa::path(
    get,
    path = "/auth_check_admin",
    tag = "generic",
    responses(
        (status = 200, description = "Valid Token / Session"),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[get("/auth_check_admin")]
#[has_roles("rauthy_admin")]
pub async fn get_auth_check_admin(
    principal_opt: Option<web::ReqData<Option<Principal>>>,
) -> Result<HttpResponse, ErrorResponse> {
    match principal_opt {
        None => Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Not authenticated".to_string(),
        )),
        Some(p) => {
            let principal = Principal::get_from_req(p.into_inner())?;
            principal.validate_rauthy_admin()?;
            Ok(HttpResponse::Ok().finish())
        }
    }
}

/// Returns the existing encryption key ID's
///
/// Only the Key ID's are returned and never the key itself.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/encryption/keys",
    tag = "generic",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/encryption/keys")]
#[has_roles("rauthy_admin")]
pub async fn get_enc_keys(
    data: web::Data<AppState>,
    principal: web::ReqData<Option<Principal>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::get_from_req(principal.into_inner())?;
    principal.validate_rauthy_admin()?;

    let active = &data.enc_key_active;
    let mut keys = Vec::with_capacity(data.enc_keys.len());
    for key in data.enc_keys.keys() {
        keys.push(key.as_str());
    }

    let resp = EncKeysResponse { active, keys };
    Ok(HttpResponse::Ok().json(resp))
}

/// Migration of secrets to the new encryption key
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/encryption/migrate",
    tag = "generic",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/encryption/migrate")]
#[has_roles("rauthy_admin")]
pub async fn post_migrate_enc_key(
    data: web::Data<AppState>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
    req_data: actix_web_validator::Json<EncKeyMigrateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::get_from_req(principal.into_inner())?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

    encryption::migrate_encryption_alg(&data, &req_data.key_id).await?;
    Ok(HttpResponse::Ok().finish())
}

/// Returns the current Argon2ID parameters and the login time
///
/// The `login time` is the time it takes to complete a full login workflow incl password hashing.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/login_time",
    tag = "generic",
    responses(
        (status = 200, description = "LoginTimeResponse"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/login_time")]
#[has_roles("rauthy_admin")]
pub async fn get_login_time(
    data: web::Data<AppState>,
    principal: web::ReqData<Option<Principal>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::get_from_req(principal.into_inner())?;
    principal.validate_rauthy_admin()?;

    let login_time = cache_get!(
        u32,
        CACHE_NAME_LOGIN_DELAY.to_string(),
        IDX_LOGIN_TIME.to_string(),
        &data.caches.ha_cache_config,
        false
    )
    .await?
    .unwrap_or(2000);
    let argon2_params = Argon2ParamsResponse {
        m_cost: data.argon2_params.params.m_cost(),
        t_cost: data.argon2_params.params.t_cost(),
        p_cost: data.argon2_params.params.p_cost(),
    };
    let resp = LoginTimeResponse {
        argon2_params,
        login_time,
        num_cpus: num_cpus::get(),
    };
    Ok(HttpResponse::Ok().json(resp))
}

/// Find best the settings for argon2id
///
/// Does multiple argon2id computations to find the best settings for the given target values.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/password_hash_times",
    tag = "generic",
    request_body = PasswordHashTimesRequest,
    responses(
        (status = 200, description = "Ok", body = PasswordHashTimes),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/password_hash_times")]
#[has_roles("rauthy_admin")]
pub async fn post_password_hash_times(
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
    req_data: actix_web_validator::Json<PasswordHashTimesRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::get_from_req(principal.into_inner())?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

    PasswordHashTimes::compute(req_data.into_inner())
        .await
        .map(|r| HttpResponse::Ok().json(r))
}

/// Returns the currently configured password policy
///
/// **Permissions**
/// - authenticated
#[utoipa::path(
    get,
    path = "/password_policy",
    tag = "generic",
    responses(
        (status = 200, description = "Ok", body = PasswordPolicyResponse),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[get("/password_policy")]
#[has_any_permission("token-auth", "session-auth")]
pub async fn get_password_policy(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let rules = PasswordPolicy::find(&data).await?;
    Ok(HttpResponse::Ok().json(PasswordPolicyResponse::from(rules)))
}

/// Update the currently configured password policy
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/password_policy",
    tag = "generic",
    request_body = PasswordPolicyRequest,
    responses(
        (status = 200, description = "Ok", body = PasswordPolicyResponse),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[put("/password_policy")]
#[has_roles("rauthy_admin")]
pub async fn put_password_policy(
    data: web::Data<AppState>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
    req_data: actix_web_validator::Json<PasswordPolicyRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::get_from_req(principal.into_inner())?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

    let mut rules = PasswordPolicy::find(&data).await?;
    rules.apply_req(req_data.into_inner());
    rules.save(&data).await?;
    Ok(HttpResponse::Ok().json(PasswordPolicyResponse::from(rules)))
}

/// Ping -> Pong
#[utoipa::path(
    get,
    path = "/ping",
    tag = "health",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().finish()
}

/// Request a Proof-of-Work Challenge
#[utoipa::path(
    get,
    path = "/pow",
    tag = "generic",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/pow")]
pub async fn get_pow(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let pow = Pow::create(&data).await?;
    Ok(HttpResponse::Ok().json(pow))
}

/// Backend health state
///
/// Health endpoint to get some additional information about the backend status, if it exists.
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Ok", body = HealthResponse),
    ),
)]
#[get("/health")]
#[has_permissions("all")]
pub async fn get_health(data: web::Data<AppState>) -> impl Responder {
    match data.caches.ha_cache_config.rx_health_state.borrow().clone() {
        None => HttpResponse::Ok().finish(),
        Some(hs) => {
            let body = HealthResponse {
                health: hs.health,
                state: hs.state,
                connected_hosts: hs.connected_hosts,
            };
            HttpResponse::Ok().json(body)
        }
    }
}

/// Ready endpoint for kubernetes / docker health checks
#[utoipa::path(
    get,
    path = "/ready",
    tag = "health",
    responses(
        (status = 200, description = "Ok"),
        (status = 503, description = "ServiceUnavailable"),
    ),
)]
#[get("/ready")]
#[has_permissions("all")]
pub async fn get_ready(data: web::Data<AppState>) -> impl Responder {
    match data.caches.ha_cache_config.rx_health_state.borrow().clone() {
        None => {}
        Some(hs) => {
            if hs.health == redhac::QuorumHealth::Bad {
                return HttpResponse::ServiceUnavailable().finish();
            }
        }
    };

    HttpResponse::Ok().finish()
}

/// Redirects from root to the "real root" /auth/v1/
#[get("/")]
#[has_permissions("all")]
pub async fn redirect() -> impl Responder {
    HttpResponse::MovedPermanently()
        .insert_header((
            header::LOCATION,
            HeaderValue::from_str("/auth/v1/").unwrap(),
        ))
        .finish()
}

#[get("/v1")]
#[has_permissions("all")]
pub async fn redirect_v1() -> HttpResponse {
    HttpResponse::MovedPermanently()
        .insert_header(("location", "/auth/v1/"))
        .finish()
}

/// Simple `whoami` endpoint for debugging purposes
///
/// Returns a body with the original requests headers
#[utoipa::path(
    get,
    path = "/whoami",
    tag = "generic",
    responses(
        (status = 200, description = "Ok"),
        (status = 503, description = "ServiceUnavailable"),
    ),
)]
#[get("/whoami")]
#[has_permissions("all")]
pub async fn whoami(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body(
        req.headers()
            .iter()
            .map(|(k, v)| format!("{}: {:?}\n", k, v.to_str().unwrap()))
            .collect::<String>(),
    )
}
