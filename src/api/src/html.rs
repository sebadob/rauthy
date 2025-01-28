use crate::{oidc, Assets, ReqPrincipal};
use actix_web::http::header;
use actix_web::{get, post, web, FromRequest, HttpRequest, HttpResponse};
use rauthy_api_types::oidc::LogoutRequest;
use rauthy_common::constants::{DEV_MODE, HEADER_HTML};
use rauthy_error::ErrorResponse;
use rauthy_models::entity::auth_providers::AuthProviderTemplate;
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::entity::principal::Principal;
use rauthy_models::html_templates::{
    AccountHtml, AdminApiKeysHtml, AdminAttributesHtml, AdminBlacklistHtml, AdminClientsHtml,
    AdminConfigHtml, AdminDocsHtml, AdminGroupsHtml, AdminHtml, AdminRolesHtml, AdminScopesHtml,
    AdminSessionsHtml, AdminUsersHtml, DeviceHtml, FedCMHtml, HtmlTemplate, IndexHtml,
    ProvidersHtml,
};
use rauthy_models::language::Language;
use std::borrow::Cow;

// dev-only endpoint - in prod, values will be inserted into the HTML directly.
// Returns the inner template value, as it would be rendered during prod, inside the body,
// which is different depending on the id.
#[get("/template/{id}")]
pub async fn get_template(
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    // TODO maybe auto-block IP as suspicious if used in prod?
    #[cfg(not(debug_assertions))]
    return Ok(HttpResponse::NotFound().finish());

    #[cfg(debug_assertions)]
    {
        if !*DEV_MODE {
            return Ok(HttpResponse::NotFound().finish());
        }

        let tpl = HtmlTemplate::build_from_str(id.as_str(), principal.into_inner().session).await?;
        Ok(HttpResponse::Ok().body(tpl.inner().to_string()))
    }
}

// This is a dev only endpoint. It is used in very specific scenarios only to work around a
// limitation of the vite proxy. E.h. the `/oidc/logout` endpoint is used with `GET` to return
// HTML, and with POST to actually do the logout. However, the vite proxy (at the time of writing)
// cannot be configured in a fine-grained way to only proxy POST request to the backend and handle
// GET via the dev UI. This endpoint solves this issue to provide a better DX and make everything
// usable during local dev.
//
// DEV_MODE MUST be `true` and the code be compiled with `debug_assertions` to make it work.
#[post("/dev/{typ}")]
pub async fn post_dev_only_endpoints(
    typ: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    // TODO maybe auto-block IP as suspicious if used in prod?
    #[cfg(not(debug_assertions))]
    return Ok(HttpResponse::NotFound().finish());

    #[cfg(debug_assertions)]
    {
        if !*DEV_MODE {
            return Ok(HttpResponse::NotFound().finish());
        }

        match typ.as_str() {
            "logout" => {
                let params = web::Form::<LogoutRequest>::extract(&req).await?;
                let principal = web::ReqData::<Principal>::extract(&req).await?;
                oidc::post_logout_handle(params.into_inner(), principal).await
            }
            // "register" => todo!(),
            _ => Ok(HttpResponse::NotFound().finish()),
        }
    }
}

#[get("/{_:.*}")]
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
        None => {
            // Since this may resolve to a sub url path of any length, we cannot now, which
            // error template we need to serve -> just return not found
            HttpResponse::NotFound().finish()
        }
    }
}

#[get("/")]
pub async fn get_index(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let lang = Language::try_from(&req).unwrap_or_default();
    let body = IndexHtml::build(&colors, &lang);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/account")]
pub async fn get_account_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let lang = Language::try_from(&req).unwrap_or_default();
    let providers = AuthProviderTemplate::get_all_json_template().await?;
    // let body = AccountHtml::build(&colors, &lang, Some(providers));
    let body = AccountHtml::build(&colors, &lang, &[HtmlTemplate::AuthProviders(providers)]);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin")]
pub async fn get_admin_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/api_keys")]
pub async fn get_admin_api_keys_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminApiKeysHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/attributes")]
pub async fn get_admin_attr_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminAttributesHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/blacklist")]
pub async fn get_admin_blacklist_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminBlacklistHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/clients")]
pub async fn get_admin_clients_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminClientsHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/config")]
pub async fn get_admin_config_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminConfigHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/docs")]
pub async fn get_admin_docs_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminDocsHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/events")]
pub async fn get_admin_events_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminUsersHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/groups")]
pub async fn get_admin_groups_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminGroupsHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/providers")]
pub async fn get_admin_providers_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = ProvidersHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/roles")]
pub async fn get_admin_roles_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminRolesHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/scopes")]
pub async fn get_admin_scopes_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminScopesHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/sessions")]
pub async fn get_admin_sessions_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminSessionsHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/admin/users")]
pub async fn get_admin_users_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = AdminUsersHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/device")]
pub async fn get_device_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let lang = Language::try_from(&req).unwrap_or_default();
    let body = DeviceHtml::build(&colors, &lang);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

#[get("/fedcm")]
pub async fn get_fed_cm_html() -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy().await?;
    let body = FedCMHtml::build(&colors);
    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}
