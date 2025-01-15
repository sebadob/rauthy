use crate::Assets;
use actix_web::http::header;
use actix_web::{get, web, HttpRequest, HttpResponse};
use rauthy_common::constants::HEADER_HTML;
use rauthy_error::ErrorResponse;
use rauthy_models::entity::auth_providers::AuthProviderTemplate;
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::language::Language;
use rauthy_models::templates::{
    AccountHtml, AdminApiKeysHtml, AdminAttributesHtml, AdminBlacklistHtml, AdminClientsHtml,
    AdminConfigHtml, AdminDocsHtml, AdminGroupsHtml, AdminHtml, AdminRolesHtml, AdminScopesHtml,
    AdminSessionsHtml, AdminUsersHtml, DeviceHtml, FedCMHtml, IndexHtml, ProvidersHtml,
};
use std::borrow::Cow;

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
    let body = AccountHtml::build(&colors, &lang, providers);

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
