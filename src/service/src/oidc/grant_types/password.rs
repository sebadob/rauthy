use crate::password_login::password_login;
use crate::token_set::{AuthCodeFlow, AuthTime, DeviceCodeFlow, DpopFingerprint, TokenSet};
use actix_web::HttpRequest;
use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, HeaderName, HeaderValue,
};
use rauthy_api_types::oidc::{GrantType, TokenRequest};
use rauthy_common::constants::HEADER_DPOP_NONCE;
use rauthy_common::utils::real_ip_from_req;
use rauthy_data::entity::browser_id::BrowserId;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::dpop_proof::DPoPProof;
use rauthy_data::entity::login_locations::LoginLocation;
use rauthy_data::entity::user_login_states::UserLoginState;
use rauthy_data::events::event::Event;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::str::FromStr;

#[tracing::instrument(skip_all, fields(client_id = req_data.client_id, username = req_data.username))]
pub async fn grant_type_password(
    req: HttpRequest,
    browser_id: BrowserId,
    req_data: TokenRequest,
    user_failed_logins: &mut Option<i64>,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    if req_data.username.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Missing 'username'",
        ));
    }
    if req_data.password.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Missing 'password",
        ));
    }

    let (client_id, client_secret) = req_data.try_get_client_id_secret(&req)?;
    let email = req_data.username.unwrap();
    let password = req_data.password.unwrap();

    let client = Client::find(client_id).await?;
    client.validate_enabled()?;
    let header_origin = client.get_validated_origin_header(&req)?;
    if client.confidential {
        let secret = client_secret.ok_or_else(|| {
            ErrorResponse::new(ErrorResponseType::BadRequest, "Missing 'client_secret'")
        })?;
        client.validate_secret(secret, &req).await?;
    }
    client.validate_flow(GrantType::Password)?;
    // The password grant has no way to perform MFA, so `force_mfa` can never be satisfied here.
    // This is by design and RFC. If an admin allows the `password` flow for a client, not MFA
    // enforcement is possible.

    let mut headers = Vec::new();
    let dpop_fingerprint =
        if let Some(proof) = DPoPProof::opt_validated_from(&req, &header_origin).await? {
            if let Some(nonce) = &proof.claims.nonce {
                headers.push((
                    HeaderName::from_str(HEADER_DPOP_NONCE).unwrap(),
                    HeaderValue::from_str(nonce)?,
                ));
            }
            Some(DpopFingerprint(proof.jwk_fingerprint()?))
        } else {
            None
        };
    if let Some(h) = header_origin {
        headers.push(h);
        headers.push((
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("POST"),
        ));
        headers.push((
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("true"),
        ));
    }

    let ip = real_ip_from_req(&req)?;

    let user = password_login(&client, email, password, ip, user_failed_logins, true).await?;

    let ts = TokenSet::from_user(
        &user,
        &client,
        AuthTime::now(),
        dpop_fingerprint,
        None,
        None,
        None,
        // resource indicators are not supported for the password grant yet
        None,
        AuthCodeFlow::No,
        DeviceCodeFlow::No,
    )
    .await?;

    if RauthyConfig::get().vars.events.generate_token_issued {
        Event::token_issued("password", &client.id, Some(&user.email))
            .send()
            .await?;
    }

    UserLoginState::insert(&user.id, &client.id, None).await?;

    LoginLocation::spawn_background_check(user, &req, browser_id)?;

    Ok((ts, headers))
}
