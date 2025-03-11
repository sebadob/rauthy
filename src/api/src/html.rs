use crate::Assets;
use actix_web::http::header;
use actix_web::{HttpRequest, HttpResponse, get, web};
use rauthy_error::ErrorResponse;
use rauthy_models::entity::theme::ThemeCssFull;
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
    let (p, encoding) =
        if path.ends_with(".png") || path.ends_with(".webp") || path.ends_with(".jpg") {
            (Cow::from(path), "none")
        } else if accept_encoding.contains(&"br".parse().unwrap()) {
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
    HtmlCached::Index
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/account")]
pub async fn get_account_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::Account
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin")]
pub async fn get_admin_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::Admin
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/api_keys")]
pub async fn get_admin_api_keys_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminApiKeys
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/attributes")]
pub async fn get_admin_attr_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminAttributes
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/blacklist")]
pub async fn get_admin_blacklist_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminBlacklist
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/clients")]
pub async fn get_admin_clients_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminClients
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/config/argon2")]
pub async fn get_admin_config_argon2_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::ConfigArgon2
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/config/encryption")]
pub async fn get_admin_config_encryption_html(
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::ConfigEncryption
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/config/jwks")]
pub async fn get_admin_config_jwks_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::ConfigJwks
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/config/policy")]
pub async fn get_admin_config_policy_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::ConfigPolicy
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/docs")]
pub async fn get_admin_docs_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::Docs
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/events")]
pub async fn get_admin_events_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminEvents
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/groups")]
pub async fn get_admin_groups_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminGroups
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/providers")]
pub async fn get_admin_providers_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminProviders
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/roles")]
pub async fn get_admin_roles_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminRoles
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/scopes")]
pub async fn get_admin_scopes_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminScopes
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/sessions")]
pub async fn get_admin_sessions_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminSessions
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/admin/users")]
pub async fn get_admin_users_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AdminUsers
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/device")]
pub async fn get_device_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::Device
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

#[get("/fedcm")]
pub async fn get_fed_cm_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::FedCM
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}
