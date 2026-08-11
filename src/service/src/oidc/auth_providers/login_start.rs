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
        // SSRF hardening: the atproto OAuth client resolves the user-supplied identifier
        // server-side (did:web -> https fetch of the DID document, handle -> DNS + HTTPS).
        // Validate the target host BEFORE handing it to the resolver so a crafted
        // `did:web:127.0.0.1...` / `user@localhost` cannot probe loopback / private /
        // cloud-metadata endpoints.
        validate_atproto_identifier(&input)?;

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

/// Rejects atproto identifiers whose resolution target is a loopback / private / link-local /
/// multicast / unspecified host. The atproto OAuth client fetches the DID document or handle
/// resolution target server-side, so a user-supplied identifier must not point at internal
/// endpoints (SSRF surface, e.g. `did:web:127.0.0.1:443` or `alice@localhost`).
fn validate_atproto_identifier(input: &str) -> Result<(), ErrorResponse> {
    let host: String = if let Some((_, host)) = input.rsplit_once('@') {
        // handle: `alice@example.com`
        host.to_string()
    } else if let Some(rest) = input.strip_prefix("did:web:") {
        // did:web:example.com[:user][:path]  (':' separators, '%3A' encodes a port)
        rest.split(':')
            .next()
            .unwrap_or_default()
            .replace("%3A", ":")
    } else {
        // did:plc / did:key / other — resolved via the fixed, admin-configured
        // PLC directory; no user-controlled host to validate here
        return Ok(());
    };

    let host = host.trim();

    // Extract the raw IP literal, ignoring brackets and an optional port, so that
    // `alice@127.0.0.1:8080` / `alice@[::1]:8080` are still caught.
    let ip_literal = if let Some(rest) = host.strip_prefix('[') {
        // [ipv6] or [ipv6]:port
        rest.split(']').next().unwrap_or("")
    } else if host.parse::<std::net::IpAddr>().is_ok() {
        host
    } else if let Some((h, _)) = host.rsplit_once(':') {
        // possibly `<ipv4>:<port>` — check the part before the last colon
        if h.parse::<std::net::IpAddr>().is_ok() {
            h
        } else {
            host
        }
    } else {
        host
    };

    let host_plain = host.trim_matches(['[', ']']);
    if host_plain.is_empty() || host_plain.eq_ignore_ascii_case("localhost") {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "atproto identifier must not point to loopback / internal hosts",
        ));
    }
    if let Ok(ip) = ip_literal
        .parse::<std::net::IpAddr>()
        .map(|ip| ip.to_canonical())
    {
        let unsafe_ip = ip.is_loopback()
            || ip.is_unspecified()
            || ip.is_multicast()
            || matches!(ip, std::net::IpAddr::V4(v4) if v4.octets()[0] == 10)
            || matches!(ip, std::net::IpAddr::V4(v4) if v4.octets()[0] == 172 && (16..=31).contains(&v4.octets()[1]))
            || matches!(ip, std::net::IpAddr::V4(v4) if v4.octets()[0] == 192 && v4.octets()[1] == 168)
            || matches!(ip, std::net::IpAddr::V4(v4) if v4.octets()[0] == 169 && v4.octets()[1] == 254)
            || matches!(ip, std::net::IpAddr::V6(v6) if (v6.segments()[0] & 0xfe00) == 0xfc00)
            || matches!(ip, std::net::IpAddr::V6(v6) if (v6.segments()[0] & 0xffc0) == 0xfe80);
        if unsafe_ip {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "atproto identifier must not point to loopback / private networks",
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_atproto_identifier() {
        // public handles / DIDs pass
        assert!(validate_atproto_identifier("alice@example.com").is_ok());
        assert!(validate_atproto_identifier("did:web:example.com").is_ok());
        assert!(validate_atproto_identifier("did:web:example.com:user:path").is_ok());
        assert!(validate_atproto_identifier("did:plc:abc123").is_ok());
        assert!(validate_atproto_identifier("did:key:z6Mk...").is_ok());
        // loopback / localhost rejected
        assert!(validate_atproto_identifier("alice@localhost").is_err());
        assert!(validate_atproto_identifier("alice@127.0.0.1").is_err());
        assert!(validate_atproto_identifier("alice@127.0.0.1:8080").is_err());
        assert!(validate_atproto_identifier("alice@[::1]:8080").is_err());
        assert!(validate_atproto_identifier("did:web:localhost").is_err());
        assert!(validate_atproto_identifier("did:web:127.0.0.1:443:user").is_err());
        // private / link-local / metadata ranges rejected
        assert!(validate_atproto_identifier("did:web:10.0.0.1").is_err());
        assert!(validate_atproto_identifier("did:web:172.16.0.1").is_err());
        assert!(validate_atproto_identifier("did:web:192.168.1.1").is_err());
        assert!(validate_atproto_identifier("did:web:169.254.169.254").is_err());
        assert!(validate_atproto_identifier("alice@[::1]").is_err());
        assert!(validate_atproto_identifier("alice@[fc00::1]").is_err());
        assert!(validate_atproto_identifier("alice@[fe80::1]").is_err());
    }
}
