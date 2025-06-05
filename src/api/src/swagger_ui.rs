use crate::ReqPrincipal;
use actix_web::http::header::{CONTENT_SECURITY_POLICY, HeaderValue};
use actix_web::web::Path;
use actix_web::{HttpResponse, get};
use rauthy_common::constants::APPLICATION_JSON;
use rauthy_error::ErrorResponse;
use rauthy_models::rauthy_config::RauthyConfig;
use std::env;
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
pub static SWAGGER_UI_ENABLE: LazyLock<bool> = LazyLock::new(|| {
    env::var("SWAGGER_UI_ENABLE")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("Cannot parse SWAGGER_UI_ENABLE to bool")
});
pub static SWAGGER_UI_PUBLIC: LazyLock<bool> = LazyLock::new(|| {
    env::var("SWAGGER_UI_PUBLIC")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("Cannot parse SWAGGER_UI_PUBLIC to bool")
});

#[get("/docs/openapi.json")]
pub async fn get_openapi_doc(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    if !*SWAGGER_UI_ENABLE {
        return Ok(HttpResponse::NotFound().finish());
    }
    if !*SWAGGER_UI_PUBLIC && principal.validate_session_auth().is_err() {
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
    if !*SWAGGER_UI_ENABLE {
        return Ok(HttpResponse::NotFound().finish());
    }
    if !*SWAGGER_UI_PUBLIC && principal.validate_session_auth().is_err() {
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
