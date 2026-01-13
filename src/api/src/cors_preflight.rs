use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_MAX_AGE,
};
use actix_web::{HttpResponse, options};

static ALLOW_HEADERS_WITH_AUTH: &str =
    "Authorization, Accept, Accept-Language, Content-Type, Range";

#[options("/clients_dyn")]
pub async fn options_clients_dyn() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "POST, OPTIONS"))
        .insert_header((ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
        .insert_header((ACCESS_CONTROL_ALLOW_HEADERS, ALLOW_HEADERS_WITH_AUTH))
        .insert_header((ACCESS_CONTROL_MAX_AGE, "31536000"))
        .finish()
}

#[options("/oidc/authorize")]
pub async fn options_authorize() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS"))
        .insert_header((ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
        .insert_header((ACCESS_CONTROL_ALLOW_HEADERS, ALLOW_HEADERS_WITH_AUTH))
        .finish()
}

#[options("/oidc/certs")]
pub async fn options_certs() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "GET, OPTIONS"))
        .insert_header((ACCESS_CONTROL_MAX_AGE, "31536000"))
        .finish()
}

#[options("/oidc/certs/{kid}")]
pub async fn options_certs_by_kid() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "GET, OPTIONS"))
        .insert_header((ACCESS_CONTROL_MAX_AGE, "31536000"))
        .finish()
}

#[options("/oidc/logout")]
pub async fn options_logout() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS"))
        .insert_header((ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
        .insert_header((ACCESS_CONTROL_ALLOW_HEADERS, ALLOW_HEADERS_WITH_AUTH))
        .finish()
}

#[options("/oidc/token")]
pub async fn options_token() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "POST, OPTIONS"))
        .insert_header((ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
        .insert_header((ACCESS_CONTROL_ALLOW_HEADERS, ALLOW_HEADERS_WITH_AUTH))
        .finish()
}

#[options("/oidc/introspect")]
pub async fn options_introspect() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "POST, OPTIONS"))
        .insert_header((ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
        .insert_header((ACCESS_CONTROL_ALLOW_HEADERS, ALLOW_HEADERS_WITH_AUTH))
        .insert_header((ACCESS_CONTROL_MAX_AGE, "31536000"))
        .finish()
}

#[options("/oidc/userinfo")]
pub async fn options_userinfo() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS"))
        .insert_header((ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
        .insert_header((ACCESS_CONTROL_ALLOW_HEADERS, ALLOW_HEADERS_WITH_AUTH))
        .insert_header((ACCESS_CONTROL_MAX_AGE, "31536000"))
        .finish()
}

#[options("/users/{user_id}/picture/{picture_id}")]
pub async fn options_users_picture() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "GET, OPTIONS"))
        .insert_header((ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
        .insert_header((ACCESS_CONTROL_MAX_AGE, "31536000"))
        .finish()
}

#[options("/users/register")]
pub async fn options_users_register() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "POST, OPTIONS"))
        .insert_header((ACCESS_CONTROL_MAX_AGE, "31536000"))
        .finish()
}

#[options("/atproto/client_metadata")]
pub async fn options_atproto_metadata() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "POST, OPTIONS"))
        .insert_header((ACCESS_CONTROL_MAX_AGE, "31536000"))
        .finish()
}

#[options("/.well-known/openid-configuration")]
pub async fn options_openid_configuration() -> HttpResponse {
    HttpResponse::NoContent()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "GET, OPTIONS"))
        .insert_header((ACCESS_CONTROL_MAX_AGE, "31536000"))
        .finish()
}
