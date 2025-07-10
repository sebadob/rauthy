use actix_web::{HttpRequest, HttpResponse, post};
use actix_web_lab::extract::Json;
use rauthy_api_types::pam::{
    PamLoginRequest, PamLoginResponse, PamMfaFinishRequest, PamMfaStartRequest,
    PamPreflightRequest, PamPreflightResponse,
};
use rauthy_api_types::users::MfaPurpose;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::users::User;
use rauthy_models::entity::webauthn;
use rauthy_models::entity::webauthn::WebauthnServiceReq;
use validator::Validate;

#[post("/pam/preflight")]
pub async fn post_preflight(
    req: HttpRequest,
    Json(payload): Json<PamPreflightRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let client = Client::find(payload.client_id).await?;
    client.validate_enabled()?;
    // TODO as soon as it exists, validate PAM is allowed in the first place
    client.validate_secret(&payload.client_secret, &req).await?;
    let user = User::find_by_email(payload.user_email).await?;
    user.check_expired()?;
    user.check_enabled()?;
    client.validate_user_groups(&user)?;

    Ok(HttpResponse::Ok().json(PamPreflightResponse {
        // TODO disallow if e.g group prefix does not match
        login_allowed: true,
        mfa_required: user.has_webauthn_enabled(),
    }))
}

#[post("/pam/login")]
pub async fn post_login(
    req: HttpRequest,
    Json(payload): Json<PamLoginRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;
    if payload.user_password.is_none() && payload.webauthn_code.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "at least one of password or webauthn_code must be given",
        ));
    }

    let client = Client::find(payload.client_id).await?;
    client.validate_enabled()?;
    // TODO as soon as it exists, validate PAM is allowed in the first place
    client.validate_secret(&payload.client_secret, &req).await?;
    let user = User::find_by_email(payload.user_email).await?;
    user.check_expired()?;
    user.check_enabled()?;
    client.validate_user_groups(&user)?;

    if let Some(password) = payload.user_password {
        user.validate_password(password).await?;
    }

    if let Some(code) = payload.webauthn_code {
        let svc_req = WebauthnServiceReq::find(code).await?;
        if user.id != svc_req.user_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "User ID mismatch",
            ));
        }
        svc_req.delete().await?;
    } else if client.force_mfa {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "This client requires MFA",
        ));
    }

    Ok(HttpResponse::Ok().json(PamLoginResponse {
        user_id: user.id,
        can_sudo: false, // TODO some client config for golden ticket
    }))
}

#[post("/pam/mfa/start")]
pub async fn post_mfa_start(
    Json(payload): Json<PamMfaStartRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let user = User::find_by_email(payload.user_email).await?;

    let resp = webauthn::auth_start(user.id, MfaPurpose::PamLogin).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[post("/pam/mfa/finish")]
pub async fn post_mfa_finish(
    Json(payload): Json<PamMfaFinishRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    payload.validate()?;

    let resp = webauthn::auth_finish(payload.user_id, payload.data).await?;
    Ok(resp.into_response())
}
