use crate::ReqPrincipal;
use actix_web::http::header::{CONTENT_SECURITY_POLICY, HeaderValue};
use actix_web::web::Path;
use actix_web::{HttpResponse, get};
use rauthy_common::constants::APPLICATION_JSON;
use rauthy_error::ErrorResponse;
use rauthy_models::rauthy_config::RauthyConfig;
use std::sync::{Arc, LazyLock, OnceLock};

pub static OPENAPI_JSON: OnceLock<String> = OnceLock::new();
pub static OPENAPI_CONFIG: LazyLock<Arc<utoipa_swagger_ui::Config>> = LazyLock::new(|| {
    Arc::new(
        utoipa_swagger_ui::Config::from("openapi.json")
            .try_it_out_enabled(false)
            .supported_submit_methods(["get"])
            .filter(true),
    )
});

#[get("/docs/openapi.json")]
pub async fn get_openapi_doc(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    let cfg = &RauthyConfig::get().vars.server;
    if !cfg.swagger_ui_enable {
        return Ok(HttpResponse::NotFound().finish());
    }
    if !cfg.swagger_ui_public && principal.validate_session_auth().is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .body(OPENAPI_JSON.get().unwrap().as_str()))
}

#[get("/docs/{_:.*}")]
pub async fn get_swagger_ui(
    path: Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    let cfg = &RauthyConfig::get().vars.server;
    if !cfg.swagger_ui_enable {
        return Ok(HttpResponse::NotFound().finish());
    }
    if !cfg.swagger_ui_public && principal.validate_session_auth().is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    match utoipa_swagger_ui::serve(path.as_str(), OPENAPI_CONFIG.clone()) {
        Ok(swagger_file) => swagger_file
            .map(|file| {
                Ok(HttpResponse::Ok()
                    .content_type(file.content_type)
                    .insert_header((
                        CONTENT_SECURITY_POLICY,
                        HeaderValue::from_str(&format!(
                            "frame-ancestors {}; object-src 'none';",
                            RauthyConfig::get().pub_url_with_scheme
                        ))
                        .unwrap(),
                    ))
                    .body(file.bytes.to_vec()))
            })
            .unwrap_or_else(|| Ok(HttpResponse::NotFound().finish())),
        Err(error) => Ok(HttpResponse::InternalServerError().body(error.to_string())),
    }
}
