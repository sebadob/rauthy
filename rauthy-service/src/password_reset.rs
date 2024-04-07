use actix_web::cookie::SameSite;
use actix_web::{cookie, web, HttpRequest, HttpResponse};
use rauthy_common::constants::{PWD_CSRF_HEADER, PWD_RESET_COOKIE};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::{get_rand, real_ip_from_req};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::entity::magic_links::{MagicLink, MagicLinkUsage};
use rauthy_models::entity::password::PasswordPolicy;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::users::User;
use rauthy_models::entity::webauthn;
use rauthy_models::entity::webauthn::WebauthnServiceReq;
use rauthy_models::events::event::Event;
use rauthy_models::language::Language;
use rauthy_models::request::{
    PasswordResetRequest, WebauthnRegFinishRequest, WebauthnRegStartRequest,
};
use rauthy_models::templates::PwdResetHtml;
use time::OffsetDateTime;
use tracing::{debug, error};

pub async fn handle_get_pwd_reset<'a>(
    data: &web::Data<AppState>,
    req: HttpRequest,
    user_id: String,
    reset_id: String,
) -> Result<(String, cookie::Cookie<'a>), ErrorResponse> {
    let mut ml = MagicLink::find(data, &reset_id).await?;
    ml.validate(&user_id, &req, false)?;

    // check if the user has MFA enabled
    let user = User::find(data, ml.user_id.clone()).await?;
    let email = if user.has_webauthn_enabled() {
        Some(&user.email)
    } else {
        None
    };

    // get the html and insert values
    let rules = PasswordPolicy::find(data).await?;
    let colors = ColorEntity::find_rauthy(data).await?;
    let lang = Language::try_from(&req).unwrap_or_default();
    let html = PwdResetHtml::build(&ml.csrf_token, &rules, email, &colors, &lang);

    // generate a cookie value and save it to the magic link
    let cookie_val = get_rand(48);
    ml.cookie = Some(cookie_val);
    ml.save(data).await?;

    let age_secs = ml.exp - OffsetDateTime::now_utc().unix_timestamp();
    let max_age = cookie::time::Duration::seconds(age_secs);
    let cookie = cookie::Cookie::build(PWD_RESET_COOKIE, ml.cookie.unwrap())
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(max_age)
        .path("/auth")
        .finish();

    Ok((html, cookie))
}

#[tracing::instrument(level = "debug", skip_all, fields(user_id = user_id))]
pub async fn handle_put_user_passkey_start<'a>(
    data: &web::Data<AppState>,
    req: HttpRequest,
    user_id: String,
    req_data: WebauthnRegStartRequest,
) -> Result<HttpResponse, ErrorResponse> {
    // validate user_id / given email address
    debug!("getting user");
    let user = User::find(data, user_id).await?;
    if req_data.email != Some(user.email) {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("E-Mail does not match for this user"),
        ));
    }

    debug!("getting magic link");
    // unwrap is safe -> checked in API endpoint already
    let ml_id = req_data.magic_link_id.as_ref().unwrap();
    let ml = MagicLink::find(data, ml_id).await?;
    ml.validate(&user.id, &req, true)?;

    // if we register a new passkey, we need to make sure that the magic link is for a new user
    let usage = MagicLinkUsage::try_from(&ml.usage)?;
    if usage != MagicLinkUsage::NewUser {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "You cannot register a new passkey here for an existing user".to_string(),
        ));
    }

    webauthn::reg_start(data, user.id, req_data)
        .await
        .map(|ccr| HttpResponse::Ok().json(ccr))
}

#[tracing::instrument(level = "debug", skip_all, fields(user_id = user_id))]
pub async fn handle_put_user_passkey_finish<'a>(
    data: &web::Data<AppState>,
    req: HttpRequest,
    user_id: String,
    req_data: WebauthnRegFinishRequest,
) -> Result<HttpResponse, ErrorResponse> {
    // unwrap is safe -> checked in API endpoint already
    let ml_id = req_data.magic_link_id.as_ref().unwrap();
    let mut ml = MagicLink::find(data, ml_id).await?;
    ml.validate(&user_id, &req, true)?;

    // finish webauthn request -> always force UV for passkey only accounts
    debug!("ml is valid - finishing webauthn request");
    webauthn::reg_finish(data, user_id.clone(), req_data).await?;

    // validate csrf token
    match req.headers().get(PWD_CSRF_HEADER) {
        None => {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                String::from("CSRF Token is missing"),
            ));
        }
        Some(token) => {
            if ml.csrf_token != token.to_str().unwrap_or("") {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    String::from("Invalid CSRF Token"),
                ));
            }
        }
    }

    debug!("invalidating magic link pwd");
    // all good
    ml.invalidate(data).await?;
    // we are re-fetching the user on purpose here to not need to modify the general webauthn fn
    let mut user = User::find(data, user_id).await?;
    user.email_verified = true;
    user.save(data, None, None).await?;

    // delete the cookie
    let cookie = cookie::Cookie::build(PWD_RESET_COOKIE, "")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(cookie::time::Duration::ZERO)
        .path("/auth")
        .finish();
    Ok(HttpResponse::Created().cookie(cookie).finish())
}

#[tracing::instrument(level = "debug", skip_all, fields(email = req_data.email))]
pub async fn handle_put_user_password_reset<'a>(
    data: &web::Data<AppState>,
    req: HttpRequest,
    user_id: String,
    req_data: PasswordResetRequest,
) -> Result<cookie::Cookie<'a>, ErrorResponse> {
    // validate user_id / given email address
    let mut user = User::find(data, user_id).await?;
    if user.email != req_data.email {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("E-Mail does not match for this user"),
        ));
    }

    // check MFA code
    if user.has_webauthn_enabled() {
        match req_data.mfa_code {
            None => {
                // TODO delete the whole ML too?
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "MFA code is missing".to_string(),
                ));
            }
            Some(code) => {
                let svc_req = WebauthnServiceReq::find(data, code).await?;
                if svc_req.user_id != user.id {
                    // TODO delete the whole ML too?
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Forbidden,
                        "User ID does not match".to_string(),
                    ));
                }

                svc_req.delete(data).await?;
            }
        }
    }

    let mut ml = MagicLink::find(data, &req_data.magic_link_id).await?;
    ml.validate(&user.id, &req, true)?;

    // validate password
    user.apply_password_rules(data, &req_data.password).await?;

    // all good
    ml.invalidate(data).await?;
    user.email_verified = true;
    user.save(data, None, None).await?;

    let ip = match real_ip_from_req(&req) {
        None => {
            error!("Extracting clients real IP from HttpRequest during password reset");
            "UNKNOWN".to_string()
        }
        Some(ip) => ip,
    };
    data.tx_events
        .send_async(Event::user_password_reset(
            format!("Reset via Password Reset Form: {}", user.email),
            Some(ip),
        ))
        .await
        .unwrap();

    // delete all existing user sessions to have a clean flow
    Session::invalidate_for_user(data, &user.id).await?;

    // delete the cookie
    let cookie = cookie::Cookie::build(PWD_RESET_COOKIE, "")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(cookie::time::Duration::ZERO)
        .path("/auth")
        .finish();
    Ok(cookie)
}
