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

/// In case of any error, the callback code will be fully deleted for security reasons.
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

    // validate state
    if payload.iss_atproto.is_none() && callback_id != payload.state {
        AuthProviderCallback::delete(callback_id).await?;

        error!("`state` does not match");
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "`state` does not match",
        ));
    }

    // validate csrf token
    let slf = AuthProviderCallback::find(callback_id).await?;
    if slf.xsrf_token != payload.xsrf_token {
        AuthProviderCallback::delete(slf.callback_id).await?;

        error!("invalid CSRF token");
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "invalid CSRF token",
        ));
    }

    // validate PKCE verifier
    let hash_base64 = base64_url_encode(sha256!(payload.pkce_verifier.as_bytes()));
    if slf.pkce_challenge != hash_base64 {
        AuthProviderCallback::delete(slf.callback_id).await?;

        error!("invalid PKCE verifier");
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "invalid PKCE verifier",
        ));
    }

    // request is valid -> fetch token for the user
    let provider = AuthProvider::find(&slf.provider_id).await?;

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
    session
        .set_mfa(provider_mfa_login == ProviderMfaLogin::Yes || require_webauthn)
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
            header_origin,
            require_webauthn,
        },
        None,
        Some(provider_mfa_login),
    )
    .await?;

    // callback data deletion cookie
    let cookie = ApiCookie::build(COOKIE_UPSTREAM_CALLBACK, "", 0);

    Ok((auth_step, cookie, is_new_user))
}
