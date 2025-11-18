use actix_web::cookie::Cookie;
use actix_web::http::header::HeaderValue;
use atrium_oauth::{AuthorizeOptions, KnownScope, Scope as ScopeAtproto};
use cryptr::utils::secure_random_alnum;
use rauthy_api_types::auth_providers::ProviderLoginRequest;
use rauthy_common::constants::{
    COOKIE_UPSTREAM_CALLBACK, PROVIDER_ATPROTO, UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS,
};
use rauthy_data::api_cookie::ApiCookie;
use rauthy_data::entity::atproto;
use rauthy_data::entity::auth_providers::{AuthProvider, AuthProviderCallback};
use rauthy_data::entity::clients::Client;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::fmt::Write;
use tracing::error;

/// returns (encrypted cookie, xsrf token, location header, optional allowed origins)
pub async fn login_start<'a>(
    payload: ProviderLoginRequest,
) -> Result<(Cookie<'a>, String, HeaderValue), ErrorResponse> {
    let provider = AuthProvider::find(&payload.provider_id).await?;

    if !RauthyConfig::get().vars.atproto.enable && provider.issuer == PROVIDER_ATPROTO {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "atproto is disabled",
        ));
    }

    let client = Client::find(payload.client_id).await?;

    let slf = AuthProviderCallback {
        callback_id: secure_random_alnum(32),
        xsrf_token: secure_random_alnum(32),
        typ: provider.typ,

        req_client_id: client.id,
        req_scopes: payload.scopes,
        req_redirect_uri: payload.redirect_uri,
        req_state: payload.state,
        req_nonce: payload.nonce,
        req_code_challenge: payload.code_challenge,
        req_code_challenge_method: payload.code_challenge_method,

        provider_id: provider.id,

        pkce_challenge: payload.pkce_challenge,
    };

    let mut location = format!(
        "{}{}client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
        provider.authorization_endpoint,
        // append parameters if there are already some parameters
        if provider.authorization_endpoint.contains('?') {
            '&'
        } else {
            '?'
        },
        provider.client_id,
        RauthyConfig::get().provider_callback_uri_encoded,
        provider.scope,
        slf.callback_id
    );
    if provider.use_pkce {
        write!(
            location,
            "&code_challenge={}&code_challenge_method=S256",
            slf.pkce_challenge
        )
        .expect("write to always succeed");
    }

    if let Some(input) = payload
        .handle
        .filter(|_| provider.issuer == PROVIDER_ATPROTO)
    {
        let atproto = atproto::Client::get();

        let options = AuthorizeOptions {
            state: Some(slf.callback_id.clone()),
            redirect_uri: Some(RauthyConfig::get().provider_callback_uri.clone()),
            scopes: vec![
                ScopeAtproto::Unknown("transition:email".to_owned()),
                ScopeAtproto::Known(KnownScope::Atproto),
                ScopeAtproto::Known(KnownScope::TransitionGeneric),
            ],
            ..Default::default()
        };

        location = atproto
            .authorize(input, options)
            .await
            .map_err(|error| {
                error!(%error, "failed to start authorization for ATProto");
            })
            .unwrap();
    }

    let cookie = ApiCookie::build(
        COOKIE_UPSTREAM_CALLBACK,
        &slf.callback_id,
        UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS as i64,
    );

    slf.save().await?;

    Ok((
        cookie,
        slf.xsrf_token,
        HeaderValue::from_str(&location).expect("Location HeaderValue to be correct"),
    ))
}
