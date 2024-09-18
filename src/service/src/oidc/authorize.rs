use actix_web::http::header;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{web, HttpRequest};
use chrono::Utc;
use rauthy_api_types::oidc::{LoginRefreshRequest, LoginRequest};
use rauthy_common::constants::{COOKIE_MFA, SESSION_RENEW_MFA, WEBAUTHN_REQ_EXP};
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::api_cookie::ApiCookie;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::auth_codes::AuthCode;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::users::{AccountType, User};
use rauthy_models::entity::webauthn::{WebauthnCookie, WebauthnLoginReq};
use rauthy_models::{AuthStep, AuthStepAwaitWebauthn, AuthStepLoggedIn};
use std::fmt::Write;
use tracing::trace;

pub async fn post_authorize(
    data: &web::Data<AppState>,
    req: &HttpRequest,
    req_data: LoginRequest,
    mut session: Session,
    has_password_been_hashed: &mut bool,
    add_login_delay: &mut bool,
    user_needs_mfa: &mut bool,
) -> Result<AuthStep, ErrorResponse> {
    let mut user = User::find_by_email(data, req_data.email)
        .await
        .inspect_err(|_| {
            // The UI does not show the password input form when there is no user yet.
            // To prevent username enumeration, we should not add a login delay if a user does not
            // even exist, when the UI is in that phase where the user does not provide any
            // password.
            if req_data.password.is_none() {
                *add_login_delay = false;
            }
        })?;

    let mfa_cookie =
        if let Ok(c) = WebauthnCookie::parse_validate(&ApiCookie::from_req(req, COOKIE_MFA)) {
            if c.email == user.email && user.has_webauthn_enabled() {
                Some(c)
            } else {
                // If a possibly existing mfa cookie does not match the given email, or the user
                // has webauthn disabled in the meantime, ignore the cookie
                None
            }
        } else {
            None
        };

    let account_type = user.account_type();

    // only allow an empty password, if the user has a passkey only account or a valid MFA cookie
    let user_must_provide_password =
        req_data.password.is_none() && account_type != AccountType::Passkey && mfa_cookie.is_none();
    if user_must_provide_password {
        // if we get here, the UI did the first step from the login form
        // -> username only without password
        // we should not add a delay in that case, because the user did nothing wrong, we just need
        // to get the password, because it is no passkey only account
        *add_login_delay = false;

        trace!("No user password has been provided");
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "User needs to provide a password",
        ));
    }

    if account_type == AccountType::New {
        // the user has created an account but no password has been set so far
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "The account has not been set up yet",
        ));
    }

    user.check_enabled()?;
    user.check_expired()?;

    // TODO should we move the password hashing as far back as possible? -> most expensive operation
    // maybe it makes sense to do additional DB requests instead of hashing a password?
    // what about brute force attempts in that case?
    // -> identify the best ordering and if it maybe makes sense to check the client first
    if let Some(pwd) = req_data.password {
        *has_password_been_hashed = true;
        user.validate_password(data, pwd).await?;

        // update user info
        // in case of webauthn login, the info will be updated in the oidc finish step
        user.last_login = Some(Utc::now().timestamp());
        user.last_failed_login = None;
        user.failed_login_attempts = None;
        user.save(data, None, None).await?;
    }

    // client validations
    let client = Client::find_maybe_ephemeral(data, req_data.client_id).await?;
    client.validate_mfa(&user).inspect_err(|_| {
        // in this case, we do not want to add a login delay
        // the user password was correct, we only need a passkey being added to the account
        *user_needs_mfa = true;
        *add_login_delay = false;
    })?;
    client.validate_redirect_uri(&req_data.redirect_uri)?;
    client.validate_code_challenge(&req_data.code_challenge, &req_data.code_challenge_method)?;
    let header_origin = client.validate_origin(req, &data.listen_scheme, &data.public_url)?;

    // build authorization code
    let code_lifetime = if user.has_webauthn_enabled() {
        client.auth_code_lifetime + *WEBAUTHN_REQ_EXP as i32
    } else {
        client.auth_code_lifetime
    };
    let scopes = client.sanitize_login_scopes(&req_data.scopes)?;
    let code = AuthCode::new(
        user.id.clone(),
        client.id,
        Some(session.id.clone()),
        req_data.code_challenge,
        req_data.code_challenge_method,
        req_data.nonce,
        scopes,
        code_lifetime,
    );
    code.save().await?;

    // build location header
    let mut loc = format!("{}?code={}", req_data.redirect_uri, code.id);
    if let Some(state) = req_data.state {
        write!(loc, "&state={}", state)?;
    };

    // TODO double check that we do not have any problems with the direct webauthn login here
    // TODO should we allow to skip this step if set so in the config?
    // check if we need to validate the 2nd factor
    if user.has_webauthn_enabled() {
        session.set_mfa(data, true).await?;

        let step = AuthStepAwaitWebauthn {
            code: get_rand(48),
            header_csrf: Session::get_csrf_header(&session.csrf_token),
            header_origin,
            user_id: user.id.clone(),
            email: user.email,
            exp: *WEBAUTHN_REQ_EXP,
            session,
        };

        WebauthnLoginReq {
            code: step.code.clone(),
            user_id: user.id,
            header_loc: loc,
            header_origin: step
                .header_origin
                .as_ref()
                .map(|h| h.1.to_str().unwrap().to_string()),
        }
        .save()
        .await?;

        Ok(AuthStep::AwaitWebauthn(step))
    } else {
        Ok(AuthStep::LoggedIn(AuthStepLoggedIn {
            user_id: user.id,
            email: user.email,
            header_loc: (header::LOCATION, HeaderValue::from_str(&loc).unwrap()),
            header_csrf: Session::get_csrf_header(&session.csrf_token),
            header_origin,
        }))
    }
}

pub async fn post_authorize_refresh(
    data: &web::Data<AppState>,
    session: &Session,
    client: Client,
    header_origin: Option<(HeaderName, HeaderValue)>,
    req_data: LoginRefreshRequest,
) -> Result<AuthStep, ErrorResponse> {
    let user_id = session.user_id.as_ref().ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            "No linked user_id for already validated session",
        )
    })?;
    let user = User::find(data, user_id.clone()).await?;
    user.check_enabled()?;
    user.check_expired()?;

    client.validate_mfa(&user)?;

    let scopes = client.sanitize_login_scopes(&req_data.scopes)?;
    let code_lifetime = if user.has_webauthn_enabled() {
        client.auth_code_lifetime + *WEBAUTHN_REQ_EXP as i32
    } else {
        client.auth_code_lifetime
    };

    let code = AuthCode::new(
        user.id.clone(),
        client.id,
        Some(session.id.clone()),
        req_data.code_challenge,
        req_data.code_challenge_method,
        req_data.nonce,
        scopes,
        code_lifetime,
    );
    code.save().await?;

    // build location header
    let header_loc = if let Some(s) = req_data.state {
        format!("{}?code={}&state={}", req_data.redirect_uri, code.id, s)
    } else {
        format!("{}?code={}", req_data.redirect_uri, code.id)
    };

    // check if we need to validate the 2nd factor
    if user.has_webauthn_enabled() && *SESSION_RENEW_MFA {
        let step = AuthStepAwaitWebauthn {
            code: get_rand(48),
            header_csrf: Session::get_csrf_header(&session.csrf_token),
            header_origin,
            user_id: user.id.clone(),
            email: user.email,
            exp: *WEBAUTHN_REQ_EXP,
            session: session.clone(),
        };

        let login_req = WebauthnLoginReq {
            code: step.code.clone(),
            user_id: user.id,
            header_loc,
            header_origin: step
                .header_origin
                .as_ref()
                .map(|h| h.1.to_str().unwrap().to_string()),
        };
        login_req.save().await?;

        Ok(AuthStep::AwaitWebauthn(step))
    } else {
        Ok(AuthStep::LoggedIn(AuthStepLoggedIn {
            user_id: user.id,
            email: user.email,
            header_loc: (
                header::LOCATION,
                HeaderValue::from_str(&header_loc).unwrap(),
            ),
            header_csrf: Session::get_csrf_header(&session.csrf_token),
            header_origin,
        }))
    }
}
