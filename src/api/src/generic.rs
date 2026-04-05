use crate::ReqPrincipal;
use actix_web::http::header;
use actix_web::http::header::{CACHE_CONTROL, CONTENT_TYPE, HeaderValue};
use actix_web::web::{Json, Query};
use actix_web::{HttpRequest, HttpResponse, Responder, get, post, put, web};
use chrono::Utc;
use cryptr::EncKeys;
use rauthy_api_types::generic::{
    AppVersionResponse, Argon2ParamsResponse, EncKeyMigrateRequest, EncKeysResponse,
    HealthResponse, I18nConfigResponse, LoginTimeResponse, PasswordHashTimesRequest,
    PasswordPolicyRequest, PasswordPolicyResponse, SearchParams, SearchParamsType,
};
use rauthy_common::compression::compress_br;
use rauthy_common::constants::{
    APP_START, APPLICATION_JSON, CSRF_HEADER, HEADER_ALLOW_ALL_ORIGINS, IDX_LOGIN_TIME,
    PWD_CSRF_HEADER, RAUTHY_VERSION,
};
use rauthy_common::utils::real_ip_from_req;
use rauthy_data::database::{Cache, DB};
use rauthy_data::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_data::entity::app_version::LatestAppVersion;
use rauthy_data::entity::ip_blacklist::IpBlacklist;
use rauthy_data::entity::is_db_alive;
use rauthy_data::entity::password::{PasswordHashTimes, PasswordPolicy};
use rauthy_data::entity::pow::PowEntity;
use rauthy_data::entity::sessions::Session;
use rauthy_data::entity::users::User;
use rauthy_data::events::event::Event;
use rauthy_data::ipgeo;
use rauthy_data::language::Language;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_service::{encryption, suspicious_request_block};
use semver::Version;
use std::fmt::Write;
use std::ops::Sub;
use std::str::FromStr;
use std::sync::LazyLock;
use tracing::{error, info, warn};
use validator::Validate;

pub static I18N_CONFIG: LazyLock<String> = LazyLock::new(|| {
    let common = RauthyConfig::get().vars.i18n.filter_lang_common
        .iter()
        .map(|v| {
                if !["de", "en", "ko", "nb", "uk", "zhhans"].contains(&v.as_ref()) {
                    panic!("Invalid config for `i18n.filter_lang_common`.\nAllowed values: en de ko nb uk zhhans\nfound: {v}");
                }
                Language::from(v.as_ref()).into()
        })
        .collect::<Vec<_>>();

    let admin = RauthyConfig::get()
        .vars
        .i18n
        .filter_lang_admin
        .iter()
        .map(|v| {
            if !["de", "en", "ko", "nb", "uk", "zhhans"].contains(&v.as_ref()) {
                panic!(
                    "Invalid config for `i18n.filter_lang_admin`\nAllowed values: en de ko nb uk zhhans\nfound: {v}",
                );
            }
            Language::from(v.as_ref()).into()
        })
        .collect::<Vec<_>>();

    serde_json::to_string(&I18nConfigResponse { common, admin }).unwrap()
});

pub static TIMEZONES_BR: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let zones = chrono_tz::TZ_VARIANTS
        .iter()
        .map(|tz| tz.name())
        .collect::<Vec<_>>();

    let json = serde_json::to_string(&zones).unwrap();
    compress_br(json.as_bytes()).unwrap()
});

/// Check if the current session is valid
#[utoipa::path(
    get,
    path = "/auth_check",
    tag = "generic",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[get("/auth_check")]
pub async fn get_auth_check(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;
    Ok(HttpResponse::Ok().finish())
}

/// Check if access to the rauthy admin API is allowed
#[utoipa::path(
    get,
    path = "/auth_check_admin",
    tag = "generic",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[get("/auth_check_admin")]
pub async fn get_auth_check_admin(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    Ok(HttpResponse::Ok().finish())
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
        (status = 200, description = "Ok", body = EncKeysResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/encryption/keys")]
pub async fn get_enc_keys(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Secrets, AccessRights::Read)?;

    let enc_keys = EncKeys::get_static();
    let active = &enc_keys.enc_key_active;
    let keys = enc_keys
        .enc_keys
        .iter()
        .map(|(id, _)| id.as_str())
        .collect::<Vec<&str>>();

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
    request_body = EncKeyMigrateRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[post("/encryption/migrate")]
pub async fn post_migrate_enc_key(
    req: HttpRequest,
    principal: ReqPrincipal,
    Json(payload): Json<EncKeyMigrateRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Secrets, AccessRights::Update)?;
    payload.validate()?;

    let ip = real_ip_from_req(&req)?;
    encryption::migrate_encryption_alg(&payload.key_id).await?;

    RauthyConfig::get()
        .tx_events
        .send_async(Event::secrets_migrated(ip))
        .await
        .unwrap();

    Ok(HttpResponse::Ok().finish())
}

/// Returns the languages to show in the UI
#[utoipa::path(
    get,
    path = "/i18n_config",
    tag = "generic",
    responses(
        (status = 200, description = "Ok", body = I18nConfigResponse),
    ),
)]
#[get("/i18n_config")]
pub async fn get_i18n_config() -> Result<HttpResponse, ErrorResponse> {
    Ok(HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, APPLICATION_JSON))
        .insert_header((CACHE_CONTROL, "max-age=300, stale-while-revalidate=2592000"))
        .body(&**I18N_CONFIG))
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
        (status = 200, description = "Ok", body = LoginTimeResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/login_time")]
pub async fn get_login_time(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Generic, AccessRights::Read)?;

    let login_time: u32 = DB::hql()
        .get(Cache::App, IDX_LOGIN_TIME)
        .await?
        .unwrap_or(2000);

    let params = &RauthyConfig::get().argon2_params;
    let argon2_params = Argon2ParamsResponse {
        m_cost: params.m_cost(),
        t_cost: params.t_cost(),
        p_cost: params.p_cost(),
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
pub async fn post_password_hash_times(
    principal: ReqPrincipal,
    Json(payload): Json<PasswordHashTimesRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Generic, AccessRights::Create)?;
    payload.validate()?;

    PasswordHashTimes::compute(payload)
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
pub async fn get_password_policy(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;
    let rules = PasswordPolicy::find().await?;
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
pub async fn put_password_policy(
    principal: ReqPrincipal,
    Json(payload): Json<PasswordPolicyRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Secrets, AccessRights::Update)?;
    payload.validate()?;

    let mut rules = PasswordPolicy::find().await?;
    rules.apply_req(payload);
    rules.save().await?;
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
    post,
    path = "/pow",
    tag = "generic",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[post("/pow")]
pub async fn post_pow() -> Result<HttpResponse, ErrorResponse> {
    // TODO can we limit the creation of new pows in a way that makes sense?
    //  By IP could be problematic if lots of users have the same public IP.
    let pow = PowEntity::create().await?;
    Ok(HttpResponse::Ok()
        .insert_header(HEADER_ALLOW_ALL_ORIGINS)
        .body(pow.to_string()))
}

/// Search endpoint used for searching from the Admin UI with active server side pagination
#[utoipa::path(
    get,
    path = "/search",
    tag = "generic",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest"),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[get("/search")]
pub async fn get_search(
    Query(params): Query<SearchParams>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;
    params.validate()?;

    let limit = params.limit.unwrap_or(100) as i64;
    match params.ty {
        SearchParamsType::Session => {
            let res = Session::search(&params.idx, &params.q, limit).await?;
            Ok(HttpResponse::Ok().json(res))
        }
        SearchParamsType::User => {
            let res = User::search(&params.idx, &params.q, limit).await?;
            Ok(HttpResponse::Ok().json(res))
        }
    }
}

/// GET all known Timezones
#[utoipa::path(
    get,
    path = "/timezones",
    tag = "generic",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/timezones")]
pub async fn get_timezones(accept_encoding: web::Header<header::AcceptEncoding>) -> HttpResponse {
    let accept_encoding = accept_encoding.into_inner();
    if accept_encoding.contains(&"br".parse().unwrap()) {
        // In almost all cases, `br` will be accepted on the client side.
        // No need to save a plain or `gz` json as well.

        HttpResponse::Ok()
            // caches for 30 days - Timezones are almost never updated
            .insert_header(("cache-control", "max-age=2592000; public"))
            .insert_header(("content-encoding", "br"))
            .content_type(APPLICATION_JSON)
            .body(TIMEZONES_BR.as_slice())
    } else {
        let zones = chrono_tz::TZ_VARIANTS
            .iter()
            .map(|tz| tz.name())
            .collect::<Vec<_>>();

        HttpResponse::Ok()
            .insert_header(("cache-control", "max-age=2592000; public"))
            .json(zones)
    }
}

/// Updates the language for the logged-in principal depending on the `locale` cookie
#[utoipa::path(
    post,
    path = "/update_language",
    tag = "generic",
    responses(
        (status = 200, description = "Valid Token / Session"),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[post("/update_language")]
pub async fn post_update_language(
    principal: ReqPrincipal,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let user_id = principal.user_id()?;
    let mut user = User::find(user_id.to_string()).await?;

    user.language = Language::try_from(&req).unwrap_or_default();
    user.update_language().await?;

    Ok(HttpResponse::Ok().finish())
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
pub async fn get_health() -> impl Responder {
    if Utc::now().sub(*APP_START).num_seconds()
        < RauthyConfig::get().vars.database.health_check_delay_secs as i64
    {
        info!("Early health check within the HEALTH_CHECK_DELAY_SECS timeframe - returning true");
        HttpResponse::Ok().json(HealthResponse {
            db_healthy: true,
            cache_healthy: true,
        })
    } else {
        let db_healthy = is_db_alive().await;
        let cache_healthy = DB::hql().is_healthy_cache().await.is_ok();

        let body = HealthResponse {
            db_healthy,
            cache_healthy,
        };

        if db_healthy && cache_healthy {
            HttpResponse::Ok().json(body)
        } else {
            HttpResponse::InternalServerError().json(body)
        }
    }
}

/// Ready endpoint for kubernetes / docker ready checks.
#[utoipa::path(
    get,
    path = "/ready",
    tag = "health",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/ready")]
pub async fn get_ready() -> impl Responder {
    HttpResponse::Ok().finish()
}

/// Catch all - redirects from root to the "real root" /auth/v1/
/// If `BLACKLIST_SUSPICIOUS_REQUESTS` is set, it will also compare the
/// request path against common bot / hacker scan targets and blacklist preemptively.
#[get("/{_:.*}")]
pub async fn catch_all(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    let path = req.path();
    let ip = real_ip_from_req(&req)?;

    let vars = &RauthyConfig::get().vars.suspicious_requests;
    if vars.log && path.len() > 1 {
        // TODO create a new event type for these? maybe too many events ...?
        warn!("Suspicious request path '{}' from {}", path, ip)
    }

    if vars.blacklist > 0
        // `/` will be the path of length 1
        && path.len() > 1
        && suspicious_request_block::is_scan_target(path)
    {
        warn!(
            "Blacklisting suspicious target path request '{}' from {}",
            path, ip,
        );

        if let Err(err) = IpBlacklist::put(ip.to_string(), vars.blacklist as i64).await {
            error!(
                "Error blacklisting suspicious request - please repot this bug: {:?}",
                err
            );
        }

        let location = ipgeo::get_location_from_db(ip)?;
        Event::suspicious_request(path, ip, location).send().await?;

        return Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "You have been blocked because of API scanning. This incident has been reported.",
        ));
    }

    Ok(HttpResponse::MovedPermanently()
        .insert_header((header::LOCATION, HeaderValue::from_static("/auth/v1/")))
        .finish())
}

#[get("/v1")]
pub async fn redirect_v1() -> HttpResponse {
    HttpResponse::MovedPermanently()
        .insert_header(("location", "/auth/v1/"))
        .finish()
}

/// Returns the current Rauthy Version
#[utoipa::path(
    get,
    path = "/version",
    tag = "generic",
    responses(
        (status = 200, description = "Ok", body = AppVersionResponse),
    ),
)]
#[get("/version")]
pub async fn get_version() -> Result<HttpResponse, ErrorResponse> {
    // TODO maybe require an auth session here and only show to admins?

    let resp = match LatestAppVersion::find().await {
        Some(latest) => {
            let update_available = match Version::from_str(RAUTHY_VERSION) {
                Ok(current) => latest.latest_version > current,
                Err(err) => {
                    error!("Cannot parse RAUTHY_VERSION: {:?}", err);
                    false
                }
            };
            AppVersionResponse {
                current: RAUTHY_VERSION.to_string(),
                last_check: Some(latest.timestamp),
                latest: Some(latest.latest_version.to_string()),
                latest_url: Some(latest.release_url.to_string()),
                update_available,
            }
        }
        None => AppVersionResponse {
            current: RAUTHY_VERSION.to_string(),
            last_check: None,
            latest: None,
            latest_url: None,
            update_available: false,
        },
    };
    Ok(HttpResponse::Ok().json(resp))
}

/// Returns the remote IP that Rauthy has extracted for this client
///
/// During development, with `debug_assertions` enabled, this endpoint returns the full set of HTTP
/// headers the request comes in with. A release build though will only return the extracted
/// "real IP". This is useful for checking any reverse proxy configuration and making sure the
/// setup is correct.
#[utoipa::path(
    get,
    path = "/whoami",
    tag = "generic",
    responses(
        (status = 200, description = "Ok"),
    ),
)]
#[get("/whoami")]
pub async fn get_whoami(req: HttpRequest) -> String {
    if RauthyConfig::get().vars.access.whoami_headers {
        let mut s = String::with_capacity(32);

        let ip = real_ip_from_req(&req)
            .map(|ip| ip.to_string())
            .unwrap_or_default();
        let _ = writeln!(s, "{ip}\n");

        for (k, v) in req.headers() {
            let key = k.as_str();
            let value = if key == "cookie" || key == CSRF_HEADER || key == PWD_CSRF_HEADER {
                "<hidden>"
            } else {
                v.to_str().unwrap_or_default()
            };
            let _ = writeln!(s, "{key}: {value}, ");
        }

        s
    } else {
        real_ip_from_req(&req)
            .map(|ip| ip.to_string())
            .unwrap_or_default()
    }
}
