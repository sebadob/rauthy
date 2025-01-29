use actix_web::web::Json;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use rauthy_api_types::auth_providers::ProviderCallbackRequest;
use rauthy_api_types::oidc::LoginRequest;
use rauthy_common::constants::DEV_MODE;
use rauthy_error::ErrorResponse;

use crate::auth_providers;
#[cfg(debug_assertions)]
use rauthy_models::entity::principal::Principal;

// dev-only endpoint - in prod, values will be inserted into the HTML directly.
// Returns the inner template value, as it would be rendered during prod, inside the body,
// which is different depending on the id.
#[get("/template/{id}")]
pub async fn get_template(
    id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    // TODO maybe auto-block IP as suspicious if used in prod?
    #[cfg(not(debug_assertions))]
    return Ok(HttpResponse::NotFound().finish());

    #[cfg(debug_assertions)]
    {
        use actix_web::FromRequest;
        use rauthy_models::html_templates::HtmlTemplate;

        if !*DEV_MODE {
            return Ok(HttpResponse::NotFound().finish());
        }

        let principal = web::ReqData::<Principal>::extract(&req).await?;
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
    // TODO for some weird reason, the extraction for LoginRequest works here but not when done
    // manually below -> figure out why
    login_req: Option<Json<LoginRequest>>,
) -> Result<HttpResponse, ErrorResponse> {
    // TODO maybe auto-block IP as suspicious if used in prod?
    #[cfg(not(debug_assertions))]
    return Ok(HttpResponse::NotFound().finish());

    #[cfg(debug_assertions)]
    {
        use crate::oidc;
        use actix_web::FromRequest;
        use rauthy_api_types::oidc::{LoginRequest, LogoutRequest};
        use rauthy_models::app_state::AppState;

        if !*DEV_MODE {
            return Ok(HttpResponse::NotFound().finish());
        }

        let data = web::Data::<AppState>::extract(&req).await?;

        match typ.as_str() {
            "authorize" => {
                if let Some(payload) = login_req {
                    // TODO for some reason, this does not work...
                    // let payload = web::Json::<LoginRequest>::extract(&req).await?;
                    let principal = web::ReqData::<Principal>::extract(&req).await?;
                    oidc::post_authorize_handle(data, req, payload.into_inner(), principal).await
                } else {
                    Ok(HttpResponse::InternalServerError().finish())
                }
            }
            "logout" => {
                let params = web::Form::<LogoutRequest>::extract(&req).await?;
                let principal = web::ReqData::<Principal>::extract(&req).await?;
                oidc::post_logout_handle(params.into_inner(), principal).await
            }
            "providers_callback" => {
                let payload = web::Json::<ProviderCallbackRequest>::extract(&req).await?;
                let principal = web::ReqData::<Principal>::extract(&req).await?;
                auth_providers::post_provider_callback_handle(
                    data,
                    req,
                    payload.into_inner(),
                    principal,
                )
                .await
            }
            // "register" => todo!(),
            _ => Ok(HttpResponse::NotFound().finish()),
        }
    }
}
