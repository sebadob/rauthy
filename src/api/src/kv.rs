use crate::ReqPrincipal;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::web::{Json, Path, Query};
use actix_web::{HttpRequest, HttpResponse, delete, get, post, put};
use rauthy_api_types::kv::{
    KVAccessRequest, KVAccessResponse, KVAccessTestResponse, KVNamespaceRequest,
    KVNamespaceResponse, KVParams, KVValueRequest, KVValueResponse,
};
use rauthy_common::constants::APPLICATION_JSON;
use rauthy_data::entity::kv::{KVAccess, KVNamespace, KVValue};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_service::oidc::helpers::get_bearer_token_from_header;
use validator::Validate;

/// Returns all KV namespaces
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/kv/ns",
    tag = "kv",
    responses(
        (status = 200, description = "Ok", body = [KVNamespaceResponse]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/kv/ns")]
pub async fn get_kv_ns(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let namespaces = KVNamespace::find_all()
        .await?
        .into_iter()
        .map(KVNamespaceResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(namespaces))
}

/// Create a new KV namespace
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/kv/ns/{ns}",
    tag = "kv",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/kv/ns")]
pub async fn post_kv_ns(
    principal: ReqPrincipal,
    Json(payload): Json<KVNamespaceRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;
    principal.validate_admin_session()?;

    KVNamespace::insert(payload.name).await?;
    Ok(HttpResponse::Ok().finish())
}

/// Updates a KV namespace
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/kv/ns/{ns}",
    tag = "kv",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/kv/ns/{ns}")]
pub async fn put_kv_ns(
    principal: ReqPrincipal,
    ns: Path<String>,
    Json(payload): Json<KVNamespaceRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;
    principal.validate_admin_session()?;

    KVNamespace::update(ns.into_inner(), payload.name).await?;
    Ok(HttpResponse::Ok().finish())
}

/// Deletes a KV namespace
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/kv/ns/{ns}",
    tag = "kv",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/kv/ns/{ns}")]
pub async fn delete_kv_ns(
    principal: ReqPrincipal,
    ns: Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    KVNamespace::delete(ns.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}

/// Returns all access keys for the given KV namespace.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/kv/ns/{ns}/access",
    tag = "kv",
    responses(
        (status = 200, description = "Ok", body = [KVAccessResponse]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/kv/ns/{ns}/access")]
pub async fn get_kv_ns_access(
    principal: ReqPrincipal,
    ns: Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let keys = KVAccess::find_all(ns.into_inner()).await?;
    let mut resp = Vec::with_capacity(keys.len());
    for key in keys {
        resp.push(key.into_response()?);
    }

    Ok(HttpResponse::Ok().json(resp))
}

/// Create an access key for the given KV namespace.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/kv/ns/{ns}/access",
    tag = "kv",
    responses(
        (status = 200, description = "Ok", body = KVAccessResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/kv/ns/{ns}/access")]
pub async fn post_kv_ns_access(
    principal: ReqPrincipal,
    ns: Path<String>,
    Json(payload): Json<KVAccessRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;
    principal.validate_admin_session()?;

    let resp = KVAccess::insert(ns.into_inner(), payload.enabled, payload.name).await?;
    Ok(HttpResponse::Created().json(resp))
}

/// Update an access key for the given KV namespace.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/kv/ns/{ns}/access/{id}",
    tag = "kv",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/kv/ns/{ns}/access/{id}")]
pub async fn put_kv_ns_access(
    principal: ReqPrincipal,
    path: Path<(String, String)>,
    Json(payload): Json<KVAccessRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;
    principal.validate_admin_session()?;

    let (ns, id) = path.into_inner();
    let key = KVAccess::find(id).await?;
    if key.ns != ns {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "invalid 'ns' for given key id",
        ));
    }
    KVAccess::update(key.id, payload.enabled, payload.name).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Delete an access key for the given KV namespace.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/kv/ns/{ns}/access/{id}",
    tag = "kv",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/kv/ns/{ns}/access/{id}")]
pub async fn delete_kv_ns_access(
    principal: ReqPrincipal,
    path: Path<(String, String)>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let (ns, id) = path.into_inner();
    let key = KVAccess::find(id).await?;
    if key.ns != ns {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "invalid 'ns' for given key id",
        ));
    }
    KVAccess::delete(key.id).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Rotate a secret for an access key for the given KV namespace
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/kv/ns/{ns}/access/{id}/secret",
    tag = "kv",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/kv/ns/{ns}/access/{id}/secret")]
pub async fn post_kv_ns_access_secret(
    principal: ReqPrincipal,
    path: Path<(String, String)>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let (ns, id) = path.into_inner();
    let key = KVAccess::find(id).await?;
    if key.ns != ns {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "invalid 'ns' for given key id",
        ));
    }
    let resp = key.generate_new_secret().await?;

    Ok(HttpResponse::Ok().json(resp))
}

/// Returns all key / values for the given KV namespace.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/kv/ns/{ns}/values",
    tag = "kv",
    responses(
        (status = 200, description = "Ok", body = [KVValueResponse]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/kv/ns/{ns}/values")]
pub async fn get_kv_ns_values(
    principal: ReqPrincipal,
    ns: Path<String>,
    params: Query<KVParams>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let values = KVValue::find_all(ns.into_inner(), params.into_inner()).await?;
    let mut resp = Vec::with_capacity(values.len());
    for value in values {
        resp.push(value.try_into_response()?)
    }

    Ok(HttpResponse::Ok().json(resp))
}

/// Create a key / value for the given KV namespace
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/kv/ns/{ns}/values",
    tag = "kv",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/kv/ns/{ns}/values")]
pub async fn post_kv_ns_values(
    principal: ReqPrincipal,
    ns: Path<String>,
    Json(payload): Json<KVValueRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    KVValue::insert(
        ns.into_inner(),
        payload.key,
        payload.encrypted,
        payload.value,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

/// Update a value for the given KV namespace + key
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/kv/ns/{ns}/values",
    tag = "kv",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/kv/ns/{ns}/values")]
pub async fn put_kv_ns_values(
    principal: ReqPrincipal,
    ns: Path<String>,
    Json(payload): Json<KVValueRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;
    principal.validate_admin_session()?;

    KVValue::update(
        ns.into_inner(),
        payload.key,
        payload.encrypted,
        payload.value,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

/// Delete a key / value for the given KV namespace
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/kv/ns/{ns}/values/{key}",
    tag = "kv",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/kv/ns/{ns}/values/{key}")]
pub async fn delete_kv_ns_value(
    principal: ReqPrincipal,
    path: Path<(String, String)>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let (ns, key) = path.into_inner();
    KVValue::delete(ns, key).await?;

    Ok(HttpResponse::Ok().finish())
}

// ##########################################################
// The following endpoints are for specific access keys only.
// They are separated for better readability and to be able
// to provide an easier to use public API.
// ##########################################################

/// Returns all keys
///
/// **Permissions**
/// - valid Access Keys for any KV Namespace
#[utoipa::path(
    get,
    path = "/kv/keys",
    tag = "kv",
    responses(
        (status = 200, description = "Ok", body = [String]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/kv/keys")]
pub async fn get_kv_keys_ext(
    req: HttpRequest,
    params: Query<KVParams>,
) -> Result<HttpResponse, ErrorResponse> {
    let bearer = get_bearer_token_from_header(req.headers())?;
    let access = KVAccess::find_validated(&bearer).await?;

    let keys = KVValue::find_all_keys(access.ns, params.limit).await?;
    Ok(HttpResponse::Ok().json(keys))
}

/// Returns all key / values
///
/// **Permissions**
/// - valid Access Keys for any KV Namespace
#[utoipa::path(
    get,
    path = "/kv/values",
    tag = "kv",
    responses(
        (status = 200, description = "Ok", body = [KVValueResponse]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/kv/values")]
pub async fn get_kv_values_ext(
    req: HttpRequest,
    params: Query<KVParams>,
) -> Result<HttpResponse, ErrorResponse> {
    let bearer = get_bearer_token_from_header(req.headers())?;
    let access = KVAccess::find_validated(&bearer).await?;

    let values = KVValue::find_all(access.ns, params.into_inner()).await?;
    let mut resp = Vec::with_capacity(values.len());
    for value in values {
        resp.push(value.try_into_response()?)
    }

    Ok(HttpResponse::Ok().json(resp))
}

/// Creates a new value for a specific key if it does not exist and updates the value otherwise.
///
/// **Permissions**
/// - valid Access Keys for any KV Namespace
#[utoipa::path(
    put,
    path = "/kv/keys",
    tag = "kv",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[put("/kv/keys")]
pub async fn put_kv_value_ext(
    req: HttpRequest,
    Json(payload): Json<KVValueRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let bearer = get_bearer_token_from_header(req.headers())?;
    let access = KVAccess::find_validated(&bearer).await?;

    KVValue::upsert(access.ns, payload).await?;
    Ok(HttpResponse::Ok().finish())
}

/// Returns the value for a specific key
///
/// **Permissions**
/// - valid Access Keys for any KV Namespace
#[utoipa::path(
    get,
    path = "/kv/keys/{key}",
    tag = "kv",
    responses(
        (status = 200, description = "Ok", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/kv/keys/{key}")]
pub async fn get_kv_value_ext(
    req: HttpRequest,
    key: Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    let bearer = get_bearer_token_from_header(req.headers())?;
    let access = KVAccess::find_validated(&bearer).await?;

    let value = KVValue::find(access.ns, key.into_inner()).await?.value()?;
    Ok(HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, APPLICATION_JSON))
        .json(value))
}

/// Deletes the value for a specific key
///
/// **Permissions**
/// - valid Access Keys for any KV Namespace
#[utoipa::path(
    delete,
    path = "/kv/keys/{key}",
    tag = "kv",
    responses(
        (status = 200, description = "Ok", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[delete("/kv/keys/{key}")]
pub async fn delete_kv_value_ext(
    req: HttpRequest,
    key: Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    let bearer = get_bearer_token_from_header(req.headers())?;
    let access = KVAccess::find_validated(&bearer).await?;

    KVValue::delete(access.ns, key.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}

/// Test a KV access key
///
/// **Permissions**
/// - valid Access Keys for any KV Namespace
#[utoipa::path(
    get,
    path = "/kv/test",
    tag = "kv",
    responses(
        (status = 200, description = "Ok", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/kv/test")]
pub async fn get_kv_access_test_ext(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    let bearer = get_bearer_token_from_header(req.headers())?;
    let access = KVAccess::find_validated(&bearer).await?;

    Ok(HttpResponse::Ok().json(KVAccessTestResponse {
        id: access.id,
        ns: access.ns,
        name: access.name,
    }))
}
