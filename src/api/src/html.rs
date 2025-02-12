use crate::Assets;
use actix_web::http::header;
use actix_web::{get, web, HttpRequest, HttpResponse};
use rauthy_common::constants::HEADER_HTML;
use rauthy_error::ErrorResponse;
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::html::templates::{
    AdminApiKeysHtml, AdminAttributesHtml, AdminBlacklistHtml, AdminClientsHtml, AdminGroupsHtml,
    AdminHtml, AdminRolesHtml, AdminScopesHtml, AdminSessionsHtml, AdminUsersHtml,
};
use rauthy_models::html::HtmlCached;
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
    HtmlCached::Index.handle(req, true).await
}

#[get("/account")]
pub async fn get_account_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::Account.handle(req, true).await
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

#[get("/admin/config/argon2")]
pub async fn get_admin_config_argon2_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::ConfigArgon2.handle(req, true).await
}

#[get("/admin/config/encryption")]
pub async fn get_admin_config_encryption_html(
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::ConfigEncryption.handle(req, true).await
}

#[get("/admin/config/jwks")]
pub async fn get_admin_config_jwks_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::ConfigJwks.handle(req, true).await
}

#[get("/admin/config/policy")]
pub async fn get_admin_config_policy_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::ConfigPolicy.handle(req, true).await
}

#[get("/admin/docs")]
pub async fn get_admin_docs_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::Docs.handle(req, true).await
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
pub async fn get_admin_providers_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminProviders.handle(req, true).await
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
    HtmlCached::Device.handle(req, true).await
}

#[get("/fedcm")]
pub async fn get_fed_cm_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::FedCM.handle(req, true).await
}
