use actix_web::HttpRequest;
use actix_web::http::header;
use actix_web::http::header::{HeaderName, HeaderValue};
use chrono::Utc;
use rauthy_api_types::oidc::{LoginRefreshRequest, LoginRequest};
use rauthy_common::constants::COOKIE_MFA;
use rauthy_common::utils::get_rand;
use rauthy_data::api_cookie::ApiCookie;
use rauthy_data::entity::auth_codes::{AuthCode, AuthCodeToSAwait};
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::login_locations::LoginLocation;
use rauthy_data::entity::sessions::Session;
use rauthy_data::entity::users::{AccountType, User};
use rauthy_data::entity::webauthn::{WebauthnCookie, WebauthnLoginReq};
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_data::{AuthStep, AuthStepAwaitWebauthn, AuthStepLoggedIn, AwaitToSAccept};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::fmt::Write;
use tracing::{trace, warn};

pub async fn post_authorize(
    req: &HttpRequest,
    req_data: LoginRequest,
    mut session: Session,
    has_password_been_hashed: &mut bool,
    add_login_delay: &mut bool,
    user_needs_mfa: &mut bool,
) -> Result<AuthStep, ErrorResponse> {
    *add_login_delay = true;

    let mut user = User::find_by_email(req_data.email).await.inspect_err(|_| {
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
                // has webauthn disabled in the meantime, ignore it
                None
            }
        } else {
            None
        };

    let account_type = user.account_type();

    // Only allow an empty password, if the user has a passkey only account or a valid MFA cookie.
    let user_must_provide_password =
        req_data.password.is_none() && account_type != AccountType::Passkey && mfa_cookie.is_none();
    if user_must_provide_password {
        // if we get here, the UI did the first step from the login form
        // -> username only without password
        // We should not add a delay in that case, because the user did nothing wrong, we just need
        // to get the password, because it is no passkey only account.
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

    if let Some(pwd) = req_data.password {
        *has_password_been_hashed = true;
        user.validate_password(pwd).await?;

        // This would also send a location notification if an attacker only knows a password, but
        // is later on unable to fully compromise an account when MFA is missing. However, this is
        // not really a false positive. We want to inform a user even if only the password got
        // stolen, so that users change them even without full account compromise.
        LoginLocation::spawn_background_check(user.clone(), req)?;

        // update user info
        // in case of webauthn login, the info will be updated in the oidc finish step
        user.last_login = Some(Utc::now().timestamp());
        user.last_failed_login = None;
        user.failed_login_attempts = None;
        user.save(None).await?;
    }
    // If the password was correct, we don't want a login delay anymore.
    // It should only prevent username enumeration and brute force, not degrade the UX.
    *add_login_delay = false;

    let need_tos_accept = user.needs_tos_update().await?;

    // client validations
    let client = Client::find_maybe_ephemeral(req_data.client_id).await?;
    client.validate_enabled()?;
    client.validate_mfa(&user).inspect_err(|_| {
        // in this case, we do not want to add a login delay
        // the user password was correct, we only need a passkey being added to the account
        *user_needs_mfa = true;
    })?;
    client.validate_user_groups(&user)?;
    client.validate_redirect_uri(&req_data.redirect_uri)?;
    client.validate_code_challenge(&req_data.code_challenge, &req_data.code_challenge_method)?;
    let header_origin = client.get_validated_origin_header(req)?;

    // build authorization code
    let config = RauthyConfig::get();
    let mut code_lifetime = client.auth_code_lifetime;
    if user.has_webauthn_enabled() {
        code_lifetime += config.vars.webauthn.req_exp as i32;
    }
    if need_tos_accept {
        code_lifetime += config.vars.tos.accept_timeout as i32;
    }
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
    code.save(code_lifetime).await?;

    let append_char = if req_data.redirect_uri.contains('?') {
        '&'
    } else {
        '?'
    };
    let mut loc = format!("{}{}code={}", req_data.redirect_uri, append_char, code.id);
    if let Some(state) = req_data.state {
        write!(loc, "&state={state}")?;
    };

    if user.has_webauthn_enabled() {
        // Webauthn-enabled account

        session.set_mfa(true).await?;

        let step = AuthStepAwaitWebauthn {
            code: get_rand(48),
            header_csrf: Session::get_csrf_header(&session.csrf_token),
            header_origin,
            user_id: user.id.clone(),
            email: user.email,
            exp: config.vars.webauthn.req_exp as u64,
            session,
        };

        let tos_await_data = if need_tos_accept { todo!() } else { None };
        WebauthnLoginReq {
            code: step.code.clone(),
            user_id: user.id,
            header_loc: loc,
            header_origin: step
                .header_origin
                .as_ref()
                .map(|h| h.1.to_str().unwrap().to_string()),
            tos_await_data,
        }
        .save()
        .await?;

        Ok(AuthStep::AwaitWebauthn(step))
    } else {
        // password only account

        if need_tos_accept {
            let code_await = AuthCodeToSAwait {
                auth_code: code.id,
                await_code: AuthCodeToSAwait::generate_code(),
                auth_code_lifetime: client.auth_code_lifetime,
                header_loc: loc,
                header_origin: header_origin
                    .as_ref()
                    .map(|(_, v)| v.to_str().unwrap().to_string()),
            };
            code_await.save().await?;

            Ok(AuthStep::AwaitToSAccept(AwaitToSAccept {
                code: code_await.await_code,
                user_id: user.id,
                header_csrf: Session::get_csrf_header(&session.csrf_token),
                header_origin,
                session,
            }))
        } else {
            Ok(AuthStep::LoggedIn(AuthStepLoggedIn {
                user_id: user.id,
                email: user.email,
                header_loc: (header::LOCATION, HeaderValue::from_str(&loc)?),
                header_csrf: Session::get_csrf_header(&session.csrf_token),
                header_origin,
            }))
        }
    }
}

pub async fn post_authorize_refresh(
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
    let user = User::find(user_id.clone()).await?;
    user.check_enabled()?;
    user.check_expired()?;

    client.validate_mfa(&user)?;
    client.validate_user_groups(&user)?;

    let scopes = client.sanitize_login_scopes(&req_data.scopes)?;

    let config = RauthyConfig::get();
    let mut code_lifetime = client.auth_code_lifetime;
    if user.has_webauthn_enabled() {
        code_lifetime += config.vars.webauthn.req_exp as i32
    }
    let need_tos_accept = user.needs_tos_update().await?;
    if need_tos_accept {
        code_lifetime += config.vars.tos.accept_timeout as i32;
    }

    todo!("check need_tos_accept return result down below");

    // let code = AuthCode::new(
    //     user.id.clone(),
    //     client.id,
    //     Some(session.id.clone()),
    //     req_data.code_challenge,
    //     req_data.code_challenge_method,
    //     req_data.nonce,
    //     scopes,
    //     code_lifetime,
    // );
    // code.save(code_lifetime).await?;
    //
    // // We don't need another location check - we can only get here with an already authenticated
    // // session and no auth-check is being performed.
    //
    // // build location header
    // let header_loc = if let Some(s) = req_data.state {
    //     format!("{}?code={}&state={s}", req_data.redirect_uri, code.id)
    // } else {
    //     format!("{}?code={}", req_data.redirect_uri, code.id)
    // };
    //
    // // check if we need to validate the 2nd factor
    // if user.has_webauthn_enabled() && RauthyConfig::get().vars.lifetimes.session_renew_mfa {
    //     let step = AuthStepAwaitWebauthn {
    //         code: get_rand(48),
    //         header_csrf: Session::get_csrf_header(&session.csrf_token),
    //         header_origin,
    //         user_id: user.id.clone(),
    //         email: user.email,
    //         exp: webauthn_req_exp as u64,
    //         session: session.clone(),
    //     };
    //
    //     let login_req = WebauthnLoginReq {
    //         code: step.code.clone(),
    //         user_id: user.id,
    //         header_loc,
    //         header_origin: step
    //             .header_origin
    //             .as_ref()
    //             .map(|h| h.1.to_str().unwrap().to_string()),
    //         is_tos_await: need_tos_accept,
    //     };
    //     login_req.save().await?;
    //
    //     Ok(AuthStep::AwaitWebauthn(step))
    // } else {
    //     Ok(AuthStep::LoggedIn(AuthStepLoggedIn {
    //         user_id: user.id,
    //         email: user.email,
    //         header_loc: (header::LOCATION, HeaderValue::from_str(&header_loc)?),
    //         header_csrf: Session::get_csrf_header(&session.csrf_token),
    //         header_origin,
    //     }))
    // }
}
