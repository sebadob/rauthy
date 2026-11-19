use crate::token_set::{DpopFingerprint, TokenScopes, TokenSet};
use actix_web::HttpRequest;
use actix_web::http::header::{HeaderName, HeaderValue};
use chrono::Utc;
use rauthy_api_types::oidc::{GrantType, TokenRequest, TokenType};
use rauthy_common::constants::HEADER_DPOP_NONCE;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::dpop_proof::DPoPProof;
use rauthy_data::entity::issued_tokens::IssuedToken;
use rauthy_data::entity::scopes::Scope;
use rauthy_data::entity::sessions::Session;
use rauthy_data::entity::users::User;
use rauthy_data::events::event::Event;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_jwt::claims::{ActClaim, JwtAccessClaims, JwtIdClaims, JwtTokenType};
use rauthy_jwt::token::JwtToken;
use std::str::FromStr;
use std::time::Duration;

/// RFC 8693 token exchange.
///
/// The exchanging client authenticates as itself and presents a `subject_token`, which is the
/// identity the new token should represent. Without an `actor_token` this is an impersonation:
/// the new token looks like a token for the subject. With an `actor_token` it is a delegation:
/// the new token additionally carries an `act` claim naming the acting party, so a resource
/// server can tell "A acting for B" apart from "B".
#[tracing::instrument(skip_all, fields(client_id = req_data.client_id))]
pub async fn grant_type_token_exchange(
    req: HttpRequest,
    req_data: TokenRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    // RFC 8693 does not mandate a specific client authentication method, so the regular OAuth
    // rules apply and the secret may arrive either in the body or as an `Authorization: Basic`
    // header. `try_get_client_id_secret()` handles both, and the extracted secret is checked
    // below, so there is deliberately no body-only pre-check here.
    let (client_id, client_secret) = req_data.try_get_client_id_secret(&req)?;
    let client = Client::find(client_id).await?;

    // A dynamic client cannot manage `allowed_resources`, which every exchange is validated
    // against. Rejecting it as soon as the client is known prevents privilege escalation.
    if client.is_dynamic() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "the token exchange is not allowed for dynamic clients",
        ));
    }

    client.validate_enabled()?;
    if !client.confidential {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "the token exchange is allowed for confidential clients only",
        ));
    }
    let secret = client_secret.ok_or_else(|| {
        ErrorResponse::new(ErrorResponseType::BadRequest, "'client_secret' is missing")
    })?;
    client.validate_secret(secret, &req).await?;
    // Deny-by-default: a client can never exchange unless it has been opted in explicitly.
    // The value inside `flows_enabled` is the grant type URN itself, like for the device code.
    client.validate_flow(GrantType::TokenExchange)?;
    let header_origin = client.get_validated_origin_header(&req)?;

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

    // `requested_token_type` is optional by RFC and defaults to an access token.
    if let Some(requested) = req_data.requested_token_type.as_deref() {
        match TokenType::from_str(requested).ok() {
            Some(TokenType::AccessToken) => {}
            Some(TokenType::IdJag) => {
                let ts = exchange_id_jag(&client, &req_data, dpop_fingerprint).await?;
                return Ok((ts, headers));
            }
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!(
                        "'requested_token_type' must be '{}' or '{}'",
                        TokenType::AccessToken,
                        TokenType::IdJag
                    ),
                ));
            }
        }
    }

    let Some(subject_token) = req_data.subject_token.as_deref() else {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "'subject_token' is missing",
        ));
    };
    // By RFC, `subject_token_type` is REQUIRED.
    let Some(subject_token_type) = req_data.subject_token_type.as_deref() else {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "'subject_token_type' is missing",
        ));
    };
    // The claims borrow from this buffer, so a single one is enough as long as the subject claims
    // are dropped before it is re-used for the `actor_token`. The only two values needed
    // afterwards are `sub` and `scope`, and both are allocated into a `String` further down
    // anyway, so extracting them here costs nothing extra and saves a second 512 byte buffer.
    let mut buf = Vec::with_capacity(512);

    let (subject_sub, subject_scope, subject_exp) = {
        let subject_claims =
            validate_exchange_token(subject_token, subject_token_type, "subject_token", &mut buf)
                .await?;
        (
            subject_claims.common.sub.map(|s| s.to_string()),
            subject_claims.common.scope.map(|s| s.to_string()),
            subject_claims.common.exp,
        )
    };

    // An `actor_token` turns this into a delegation. By RFC, `actor_token_type` is REQUIRED as
    // soon as an `actor_token` is given.
    let act = if let Some(actor_token) = req_data.actor_token.as_deref() {
        let Some(actor_token_type) = req_data.actor_token_type.as_deref() else {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'actor_token_type' is required when an 'actor_token' is given",
            ));
        };
        // the subject claims are out of scope by now, so the buffer is free to be re-used
        buf.clear();
        let actor_claims =
            validate_exchange_token(actor_token, actor_token_type, "actor_token", &mut buf).await?;

        let Some(actor_sub) = actor_claims.common.sub else {
            return Err(ErrorResponse::new(
                ErrorResponseType::InvalidGrant,
                "the 'actor_token' has no 'sub' claim",
            ));
        };

        // A delegation chain is kept: if the actor was itself acting for someone, that history
        // stays nested inside the new `act`.
        Some(ActClaim {
            sub: actor_sub,
            act: actor_claims.act.map(Box::new),
        })
    } else {
        None
    };

    // RFC 8693 names the exchange target `audience`; `resource` does the same job here and is
    // already part of a token request. Both are validated against `allowed_resources`, which is
    // deny-by-default.
    let target = match (req_data.audience.as_deref(), req_data.resource.as_deref()) {
        (Some(_), Some(_)) => {
            return Err(ErrorResponse::new(
                ErrorResponseType::InvalidTarget,
                "'audience' and 'resource' must not be given at the same time",
            ));
        }
        (Some(audience), None) => Some(audience),
        (None, Some(resource)) => Some(resource),
        (None, None) => None,
    };
    if let Some(target) = target {
        client.validate_resource_request(target)?;
    }

    // The exchanged token can only ever narrow the subject's scopes, never widen them.
    // `openid` is deliberately not forced in here: only access tokens can be exchanged, so the
    // result carries no identity part that would need it, and the caller stays in control of
    // what the exchanged token is scoped to.
    // On top, the exchanging client only ever gets scopes it is allowed itself, with the same
    // rule as `Client::sanitize_login_scopes`: its `default_scopes` plus its `scopes`.
    let matrix_enabled = RauthyConfig::get().vars.matrix.msc3861_enable;
    let client_allows = |s: &str| {
        client.default_scopes.split(',').any(|d| d == s)
            || client
                .scopes
                .split(',')
                .any(|allowed| Scope::matches(allowed, s, matrix_enabled))
    };
    let subject_scope = subject_scope.unwrap_or_default();
    let scope = if let Some(requested) = req_data.scope.as_deref() {
        let mut narrowed = Vec::with_capacity(1);
        for s in requested.split_whitespace() {
            if !subject_scope.split_whitespace().any(|sub| sub == s) {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("the requested scope '{s}' is not granted to the 'subject_token'"),
                ));
            }
            if !client_allows(s) {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("the requested scope '{s}' is not allowed for this client"),
                ));
            }
            narrowed.push(s);
        }
        narrowed.join(" ")
    } else {
        subject_scope
            .split_whitespace()
            .filter(|s| client_allows(s))
            .collect::<Vec<_>>()
            .join(" ")
    };

    // A token exchange without a user is only possible for a `client_credentials` subject token,
    // where the `sub` is the client itself and there is no user to look up. A `NotFound` is
    // therefore expected and fine, while any other error must not be swallowed: that would issue
    // a token without the user's claims.
    let user = if let Some(sub) = subject_sub {
        match User::find(sub).await {
            Ok(user) => Some(user),
            Err(err) if err.error == ErrorResponseType::NotFound => None,
            Err(err) => return Err(err),
        }
    } else {
        None
    };
    if let Some(user) = &user {
        user.check_enabled()?;
        user.check_expired()?;
        client.validate_user_groups(user)?;
        // We do not validate any `force_mfa` setting here. This is
        // 1. impossible, since we just cannot verify it at this stage anyway
        // 2. not necessary at all, since the token exchange is guarded by admin-configured resource
        //    access and validation. If we would require MFA here (which we don't know anyway), it
        //    would destroy a few important flows. Since this is deliberately configured by an
        //    admin, we just don't need to.
    }

    // Never outlive the subject token or the user, or re-exchanging could extend access forever.
    let now = Utc::now().timestamp();
    let mut lifetime = (client.access_token_lifetime as i64).min(subject_exp - now);
    if let Some(user_expires) = user.as_ref().and_then(|u| u.user_expires) {
        lifetime = lifetime.min(user_expires - now);
    }
    if lifetime < 1 {
        return Err(ErrorResponse::new(
            ErrorResponseType::InvalidGrant,
            "the 'subject_token' or its user has expired",
        ));
    }

    let ts = TokenSet::for_token_exchange(
        user.as_ref(),
        &client,
        lifetime,
        dpop_fingerprint,
        TokenScopes::new(scope),
        target,
        act,
    )
    .await?;

    if RauthyConfig::get().vars.events.generate_token_issued {
        Event::token_issued(
            GrantType::TokenExchange.as_str(),
            &client.id,
            user.as_ref().map(|u| u.email.as_str()),
        )
        .send()
        .await?;
    }

    Ok((ts, headers))
}

/// Issues an ID-JAG (draft-ietf-oauth-identity-assertion-authz-grant) for an ID token that was
/// issued to the exchanging client. The `audience` is the Resource Authorization Server, and it
/// decides together with the client, `resource` and `scope` against `[[id_jag.audiences]]`.
async fn exchange_id_jag(
    client: &Client,
    req_data: &TokenRequest,
    dpop_fingerprint: Option<DpopFingerprint>,
) -> Result<TokenSet, ErrorResponse> {
    if req_data.actor_token.is_some() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "an 'actor_token' is not supported for an ID-JAG",
        ));
    }

    let Some(audience) = req_data.audience.as_deref() else {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "'audience' is required for an ID-JAG",
        ));
    };
    let Some(policy) = RauthyConfig::get().vars.id_jag.audiences.get(audience) else {
        return Err(ErrorResponse::new(
            ErrorResponseType::InvalidTarget,
            "the 'audience' is not a configured ID-JAG audience",
        ));
    };
    if !policy.clients.iter().any(|c| c == &client.id) {
        return Err(ErrorResponse::new(
            ErrorResponseType::AccessDenied,
            "this client may not request an ID-JAG for this 'audience'",
        ));
    }
    if let Some(resource) = req_data.resource.as_deref()
        && !policy.resources.iter().any(|r| r == resource)
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::InvalidTarget,
            "the 'resource' is not allowed for this 'audience'",
        ));
    }
    if let Some(scope) = req_data.scope.as_deref()
        && let Some(s) = scope
            .split_whitespace()
            .find(|s| !policy.scopes.iter().any(|allowed| allowed == s))
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::AccessDenied,
            format!("the requested scope '{s}' is not allowed for this 'audience'"),
        ));
    }

    let Some(subject_token) = req_data.subject_token.as_deref() else {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "'subject_token' is missing",
        ));
    };
    if req_data.subject_token_type.as_deref() != Some(TokenType::IdToken.as_str()) {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!(
                "the only supported 'subject_token_type' for an ID-JAG is '{}'",
                TokenType::IdToken
            ),
        ));
    }

    let mut buf = Vec::with_capacity(512);
    if JwtToken::validate_claims_into(
        subject_token,
        Some(JwtTokenType::Id),
        Duration::from_secs(0),
        &mut buf,
        false,
    )
    .await
    .is_err()
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::InvalidGrant,
            "the given 'subject_token' is not a valid id token",
        ));
    }
    let subject = serde_json::from_slice::<JwtIdClaims>(&buf)?;
    if subject.common.azp != client.id || !subject.common.aud.contains(&client.id) {
        return Err(ErrorResponse::new(
            ErrorResponseType::InvalidGrant,
            "the 'subject_token' was not issued to this client",
        ));
    }
    if let Some(cnf) = &subject.common.cnf
        && dpop_fingerprint.as_ref().map(|f| f.0.as_str()) != Some(cnf.jkt)
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::InvalidGrant,
            "the 'subject_token' is DPoP-bound and needs a DPoP proof with the same key",
        ));
    }

    let Some(sub) = subject.common.sub else {
        return Err(ErrorResponse::new(
            ErrorResponseType::InvalidGrant,
            "the 'subject_token' has no 'sub' claim",
        ));
    };
    // An id token cannot be revoked, so it is only accepted while its session is valid.
    let session_timeout = RauthyConfig::get().vars.lifetimes.session_timeout.as_secs() as i64;
    let session = match subject.sid {
        Some(sid) => match Session::find(sid.to_string()).await {
            Ok(session) => Some(session),
            Err(err) if err.error == ErrorResponseType::NotFound => None,
            Err(err) => return Err(err),
        },
        None => None,
    };
    let Some(session) = session
        .filter(|s| s.user_id.as_deref() == Some(sub) && s.is_valid(session_timeout, None, ""))
    else {
        return Err(ErrorResponse::new(
            ErrorResponseType::InvalidGrant,
            "the 'subject_token' has no valid session",
        ));
    };
    let user = User::find(sub.to_string()).await?;
    user.check_enabled()?;
    user.check_expired()?;
    client.validate_user_groups(&user)?;

    let now = Utc::now().timestamp();
    let mut exp = (now + client.access_token_lifetime as i64)
        .min(subject.common.exp)
        .min(session.exp);
    if let Some(user_expires) = user.user_expires {
        exp = exp.min(user_expires);
    }
    if exp - now < 1 {
        return Err(ErrorResponse::new(
            ErrorResponseType::InvalidGrant,
            "the 'subject_token' or its user has expired",
        ));
    }

    let ts = TokenSet::for_id_jag(
        &user,
        client,
        exp,
        dpop_fingerprint,
        &subject,
        audience,
        req_data.resource.as_deref(),
        req_data.scope.as_deref(),
    )
    .await?;

    if RauthyConfig::get().vars.events.generate_token_issued {
        Event::token_issued(
            GrantType::TokenExchange.as_str(),
            &client.id,
            Some(user.email.as_str()),
        )
        .send()
        .await?;
    }

    Ok(ts)
}

/// Validates a `subject_token` / `actor_token`. Only access tokens are accepted, and the token
/// must not have been revoked.
async fn validate_exchange_token<'a>(
    token: &str,
    token_type: &str,
    name: &str,
    buf: &'a mut Vec<u8>,
) -> Result<JwtAccessClaims<'a>, ErrorResponse> {
    if TokenType::from_str(token_type).ok() != Some(TokenType::AccessToken) {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!(
                "the only supported '{name}_type' is '{}'",
                TokenType::AccessToken
            ),
        ));
    }

    if JwtToken::validate_claims_into(
        token,
        Some(JwtTokenType::Bearer),
        Duration::from_secs(0),
        buf,
        false,
    )
    .await
    .is_err()
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::InvalidGrant,
            format!("the given '{name}' is not a valid access token"),
        ));
    }
    let claims = serde_json::from_slice::<JwtAccessClaims>(buf)?;

    if let Some(jti) = claims.common.jti {
        IssuedToken::validate_not_revoked(jti).await?;
    }

    Ok(claims)
}
