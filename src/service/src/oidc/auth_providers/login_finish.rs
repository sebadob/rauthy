use crate::oidc;
use crate::oidc::authorize::AuthorizeData;
use actix_web::HttpRequest;
use actix_web::cookie::Cookie;
use rauthy_api_types::auth_providers::ProviderCallbackRequest;
use rauthy_common::constants::{COOKIE_UPSTREAM_CALLBACK, PROVIDER_ATPROTO, PROVIDER_LINK_COOKIE};
use rauthy_common::sha256;
use rauthy_common::utils::base64_url_encode;
use rauthy_data::AuthStep;
use rauthy_data::api_cookie::ApiCookie;
use rauthy_data::entity::auth_providers::{
    AuthProvider, AuthProviderCallback, AuthProviderLinkCookie, NewFederatedUserCreated,
    ProviderMfaLogin,
};
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::sessions::Session;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use tracing::error;

/// The callback is single-use: it is deleted as soon as it has been validated.
pub async fn login_finish<'a>(
    req: &'a HttpRequest,
    payload: &'a ProviderCallbackRequest,
    mut session: Session,
) -> Result<(AuthStep, Cookie<'a>, NewFederatedUserCreated), ErrorResponse> {
    // the callback id for the cache should be inside the encrypted cookie
    let callback_id = ApiCookie::from_req(req, COOKIE_UPSTREAM_CALLBACK).ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Missing encrypted callback cookie",
        )
    })?;

    let slf = AuthProviderCallback::find(callback_id).await?;
    let provider = AuthProvider::find(&slf.provider_id).await?;

    // validate state and RFC 9207 iss
    if let Err(err) = validate_callback_response(
        &provider.issuer,
        &slf.callback_id,
        &payload.state,
        payload.iss.as_deref(),
    ) {
        AuthProviderCallback::delete(slf.callback_id).await?;

        error!("{}", err.message);
        return Err(err);
    }

    // validate csrf token
    if !constant_time_eq::constant_time_eq(slf.xsrf_token.as_bytes(), payload.xsrf_token.as_bytes())
    {
        AuthProviderCallback::delete(slf.callback_id).await?;

        error!("invalid CSRF token");
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "invalid CSRF token",
        ));
    }

    // validate PKCE verifier
    let hash_base64 = base64_url_encode(sha256!(payload.pkce_verifier.as_bytes()));
    if !constant_time_eq::constant_time_eq(slf.pkce_challenge.as_bytes(), hash_base64.as_bytes()) {
        AuthProviderCallback::delete(slf.callback_id).await?;

        error!("invalid PKCE verifier");
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "invalid PKCE verifier",
        ));
    }

    // The callback is validated at this point, so we can safely clean up the cache.
    AuthProviderCallback::delete(slf.callback_id.clone()).await?;

    // request is valid -> fetch token for the user

    // extract a possibly existing provider link cookie for
    // linking an existing account to a provider
    let link_cookie = ApiCookie::from_req(req, PROVIDER_LINK_COOKIE)
        .and_then(|value| AuthProviderLinkCookie::try_from(value.as_str()).ok());

    // deserialize payload and validate the information
    let (user, provider_mfa_login, is_new_user) = if provider.issuer == PROVIDER_ATPROTO {
        slf.extract_user_at_proto(&provider, &link_cookie, payload)
            .await?
    } else {
        slf.extract_user(&provider, &link_cookie, payload).await?
    };

    user.check_enabled()?;
    user.check_expired()?;

    if link_cookie.is_some() {
        // If this is the case, we don't need to validate any further client values.
        // We will not generate a new auth code at all -> this is just a request to federate
        // an existing account. The federation has been done in the step above already.
        return Ok((
            AuthStep::ProviderLink,
            AuthProviderLinkCookie::deletion_cookie(),
            is_new_user,
        ));
    }

    // From here on, we deal with a normal login instead of just an account federation.

    let require_webauthn = user.has_webauthn_enabled();
    let require_otp = user.has_otp_enabled().await;
    session
        .set_mfa(provider_mfa_login == ProviderMfaLogin::Yes || require_webauthn || require_otp)
        .await?;

    let client = Client::find_maybe_ephemeral(slf.req_client_id).await?;
    let header_origin = client.get_validated_origin_header(req)?;

    let auth_step = oidc::authorize::finish_authorize(
        user,
        client,
        &mut session,
        AuthorizeData {
            redirect_uri: slf.req_redirect_uri,
            scopes: slf.req_scopes,
            state: slf.req_state,
            nonce: slf.req_nonce,
            code_challenge: slf.req_code_challenge,
            code_challenge_method: slf.req_code_challenge_method,
            resource: slf.req_resource,
            header_origin,
            require_webauthn,
            require_otp,
        },
        None,
        Some(provider_mfa_login),
    )
    .await?;

    // callback data deletion cookie
    let cookie = ApiCookie::build(COOKIE_UPSTREAM_CALLBACK, "", 0);

    Ok((auth_step, cookie, is_new_user))
}

/// Validates `state` and the RFC 9207 `iss` of an upstream authorization response.
///
/// ATProto is skipped here: its `state` is generated and validated by the ATProto client
/// itself, which also checks `iss` and binds its app state to `callback_id`.
/// For all other providers, `state` must always match. If the provider sent an `iss`, it must
/// match the provider's issuer (RFC 9207 section 2.4), ignoring only a trailing `/`.
fn validate_callback_response(
    provider_issuer: &str,
    callback_id: &str,
    state: &str,
    iss: Option<&str>,
) -> Result<(), ErrorResponse> {
    if provider_issuer == PROVIDER_ATPROTO {
        return Ok(());
    }

    if !constant_time_eq::constant_time_eq(callback_id.as_bytes(), state.as_bytes()) {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "`state` does not match",
        ));
    }

    if let Some(iss) = iss
        && iss.trim_end_matches('/') != provider_issuer.trim_end_matches('/')
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "`iss` does not match the provider issuer",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ISSUER: &str = "https://upstream.example.com/auth/v1/";

    #[test]
    fn test_validate_callback_response_state() {
        assert!(validate_callback_response(ISSUER, "cb1", "cb1", None).is_ok());

        let err = validate_callback_response(ISSUER, "cb1", "cb2", None).unwrap_err();
        assert_eq!(err.error, ErrorResponseType::BadRequest);

        // an `iss` must never disable the `state` check for a non-ATProto provider
        assert!(validate_callback_response(ISSUER, "cb1", "cb2", Some(ISSUER)).is_err());
        assert!(validate_callback_response(ISSUER, "cb1", "cb2", Some("atproto")).is_err());
    }

    #[test]
    fn test_validate_callback_response_iss() {
        assert!(validate_callback_response(ISSUER, "cb1", "cb1", Some(ISSUER)).is_ok());
        assert!(
            validate_callback_response(
                ISSUER,
                "cb1",
                "cb1",
                Some("https://upstream.example.com/auth/v1")
            )
            .is_ok()
        );

        for iss in [
            "https://attacker.example.com/auth/v1/",
            "https://upstream.example.com/",
            "https://upstream.example.com/auth/v1/x",
            "",
        ] {
            let err = validate_callback_response(ISSUER, "cb1", "cb1", Some(iss)).unwrap_err();
            assert_eq!(err.error, ErrorResponseType::BadRequest, "{iss}");
        }
    }

    #[test]
    fn test_validate_callback_response_atproto() {
        // state and iss are validated by the ATProto client itself
        assert!(validate_callback_response(PROVIDER_ATPROTO, "cb1", "other", None).is_ok());
        assert!(
            validate_callback_response(
                PROVIDER_ATPROTO,
                "cb1",
                "other",
                Some("https://bsky.social")
            )
            .is_ok()
        );
    }
}
