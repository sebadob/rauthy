use crate::oidc;
use crate::{auth_providers, users};
use actix_web::FromRequest;
use actix_web::{HttpRequest, HttpResponse, web};
use futures::StreamExt;
use rauthy_api_types::auth_providers::ProviderCallbackRequest;
use rauthy_api_types::oidc::{LoginRequest, LogoutRequest};
use rauthy_api_types::users::NewUserRegistrationRequest;
use rauthy_common::constants::DEV_MODE;
use rauthy_error::ErrorResponse;
use rauthy_error::ErrorResponseType;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::principal::Principal;
use rauthy_models::html::templates::HtmlTemplate;
use rauthy_service::oidc::logout;
use tokio::fs;
use validator::Validate;

pub async fn get_template(
    id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    if !*DEV_MODE {
        return Ok(HttpResponse::NotFound().finish());
    }

    let data = web::Data::<AppState>::extract(&req).await?;
    let principal = web::ReqData::<Principal>::extract(&req).await?;
    let session = principal.validate_session_auth().ok().cloned();
    let (tpl, cookie) = HtmlTemplate::build_from_str(data, id.as_str(), session).await?;

    if let Some(cookie) = cookie {
        Ok(HttpResponse::Ok()
            .cookie(cookie)
            .body(tpl.inner().to_string()))
    } else {
        Ok(HttpResponse::Ok().body(tpl.inner().to_string()))
    }
}

pub async fn post_dev_only_endpoints(
    typ: web::Path<String>,
    req: HttpRequest,
    mut payload: web::Payload,
) -> Result<HttpResponse, ErrorResponse> {
    if !*DEV_MODE {
        return Ok(HttpResponse::NotFound().finish());
    }

    // for some weird reason, the `payload` would be empty, if we extract it here from `req`
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > 262_144 {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "payload too large",
            ));
        }
        body.extend_from_slice(&chunk);
    }
    let bytes = body.as_ref();

    let data = web::Data::<AppState>::extract(&req).await?;

    match typ.as_str() {
        "authorize" => {
            let payload = serde_json::from_slice::<LoginRequest>(bytes)?;
            let principal = web::ReqData::<Principal>::extract(&req).await?;
            oidc::post_authorize_handle(data, req, payload, principal).await
        }
        "backchannel_logout" => {
            // This endpoint is only used in integration tests.
            // It will only parse the `logout_token` and save it into a file on disk, so the
            // file can be read from inside test cases and validated properly.
            let params = serde_urlencoded::from_bytes::<LogoutRequest>(bytes)?;
            params.validate()?;
            if params.logout_token.is_none() {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "exptected 'logout_token'",
                ));
            }

            let dir = "src/bin/tests/test_data";
            fs::create_dir_all(dir).await?;
            fs::write(format!("{dir}/logout_token"), params.logout_token.unwrap()).await?;
            Ok(HttpResponse::Ok().finish())
        }
        "logout" => {
            let params = serde_urlencoded::from_bytes::<LogoutRequest>(bytes)?;
            params.validate()?;
            let principal = web::ReqData::<Principal>::extract(&req).await.ok();
            let session = principal.and_then(|p| p.validate_session_auth().ok().cloned());
            logout::post_logout_handle(req, data, params, session).await
        }
        "providers_callback" => {
            let payload = serde_json::from_slice::<ProviderCallbackRequest>(bytes)?;
            let principal = web::ReqData::<Principal>::extract(&req).await?;
            auth_providers::post_provider_callback_handle(req, payload, principal).await
        }
        "register" => {
            let payload = serde_json::from_slice::<NewUserRegistrationRequest>(bytes)?;
            users::post_users_register_handle(data, req, payload).await
        }
        _ => {
            tracing::warn!("unhandled DEV template request: {}", typ);
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
