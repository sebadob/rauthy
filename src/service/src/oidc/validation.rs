use crate::oidc::consent_grant::{consent_as_invalid_grant, effective_scope};
use crate::token_set::{
    AuthCodeFlow, AuthTime, ConsentCheck, DeviceCodeFlow, DpopFingerprint, SessionId, TokenScopes,
    TokenSet,
};
use actix_web::HttpRequest;
use actix_web::http::header::{HeaderName, HeaderValue};
use chrono::Utc;
use rauthy_api_types::oidc::GrantType;
use rauthy_common::constants::REFRESH_TOKEN_VALIDATION_LEN;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::devices::DeviceEntity;
use rauthy_data::entity::dpop_proof::DPoPProof;
use rauthy_data::entity::refresh_tokens::RefreshToken;
use rauthy_data::entity::refresh_tokens_devices::RefreshTokenDevice;
use rauthy_data::entity::user_consents::UserConsent;
use rauthy_data::entity::users::User;
use rauthy_data::events::event::Event;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_jwt::claims::{JwtRefreshClaims, JwtTokenType};
use rauthy_jwt::token::JwtToken;
use std::ops::Add;
use tracing::{debug, error};

/// Validates request parameters for the authorization and refresh endpoints
pub async fn validate_auth_code_req_params(
    req: &HttpRequest,
    client_id: &str,
    redirect_uri: &str,
    code_challenge: &Option<String>,
    code_challenge_method: &Option<String>,
) -> Result<(Client, Option<(HeaderName, HeaderValue)>), ErrorResponse> {
    let client = Client::find_maybe_ephemeral(String::from(client_id))
        .await
        .map_err(|mut err| {
            if err.error == ErrorResponseType::NotFound {
                err.message = format!("Client '{client_id}' does not exist").into();
            }
            err
        })?;

    client.validate_enabled()?;
    client.validate_redirect_uri(redirect_uri)?;
    client.validate_flow(GrantType::AuthorizationCode)?;
    let header = client.get_validated_origin_header(req)?;

    if client.challenge.is_some() {
        if code_challenge.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'code_challenge' is missing",
            ));
        } else {
            // 'plain' is the default method to be assumed by the OAuth specification when it is not
            // further specified.
            let method = if let Some(m) = code_challenge_method {
                m.to_owned()
            } else {
                String::from("plain")
            };
            client.validate_challenge_method(&method)?;
        }
    }

    Ok((client, header))
}

/// `resource` is the optional RFC 8707 `resource` of the token request. It may only narrow the
/// resource granted to the refresh token, which with a single granted resource means it must
/// match it. Without it, the granted resource is carried forward unchanged.
pub async fn validate_and_refresh_token(
    client: Client,
    refresh_token: &str,
    resource: Option<&str>,
    req: &HttpRequest,
) -> Result<(TokenSet, Option<String>), ErrorResponse> {
    let clock_skew_secs = RauthyConfig::get().vars.lifetimes.refresh_token_grace_time;
    let mut buf = Vec::with_capacity(256);
    JwtToken::validate_claims_into(
        refresh_token,
        Some(JwtTokenType::Refresh),
        clock_skew_secs,
        &mut buf,
        false,
    )
    .await?;
    let claims: JwtRefreshClaims = serde_json::from_slice(&buf)?;

    // An underflow is impossible because the token is already validated at this point.
    // However, doing this additional check is a bit safer.
    // TODO use `std::hint::unlikely` as soon as it's part of stable rust
    if refresh_token.len() < REFRESH_TOKEN_VALIDATION_LEN {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Invalid refresh token",
        ));
    }
    let (_, validation_str) =
        refresh_token.split_at(refresh_token.len() - REFRESH_TOKEN_VALIDATION_LEN);

    if client.id != claims.common.azp {
        // If this is a non-matching client.id, this is most probably a malicious request.
        // -> invalidate the refresh token immediately
        let device = if claims.common.did.is_some() {
            RefreshTokenDevice::find_opt(validation_str)
                .await
                .unwrap_or_else(|err| {
                    error!(?err, "looking up the refresh token of a client.id mismatch");
                    None
                })
        } else {
            None
        };
        let deleted = if let Some(rt) = device {
            rt.delete().await
        } else {
            match RefreshToken::find_opt(validation_str).await {
                Ok(Some(rt)) => rt.delete().await,
                Ok(None) => Ok(()),
                Err(err) => Err(err),
            }
        };
        if let Err(err) = deleted {
            error!(?err, "deleting the refresh token of a failed refresh");
        }
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Mismatch in client.id",
        ));
    }

    // RFC 8707: a refresh can never widen or switch the granted resource
    if let Some(requested) = resource
        && claims.resource != Some(requested)
    {
        return Err(ErrorResponse::new(
            ErrorResponseType::InvalidTarget,
            "the requested `resource` was not granted to the refresh token",
        ));
    }

    let header_origin = client.get_validated_origin_header(req)?;

    // validate DPoP proof
    let (dpop_fingerprint, dpop_nonce) = if let Some(cnf) = &claims.common.cnf {
        // if the refresh token contains the 'cnf' header, we must validate the DPoP as well
        if let Some(proof) = DPoPProof::opt_validated_from(req, &header_origin).await? {
            let fingerprint = proof.jwk_fingerprint()?;
            if fingerprint != cnf.jkt {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "The refresh token is bound to a missing DPoP proof",
                ));
            }
            debug!("DPoP-Bound refresh token accepted");
            (Some(DpopFingerprint(fingerprint)), proof.claims.nonce)
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "The refresh token is bound to a missing DPoP proof",
            ));
        }
    } else {
        (None, None)
    };

    let user = User::find(claims.uid.to_string()).await?;
    user.check_enabled()?;
    user.check_expired()?;
    client.validate_user_groups(&user)?;

    // a device token issued before `consent_exempt` existed is exempt by its signed `did`
    let consent = if claims.consent_exempt || claims.common.did.is_some() {
        ConsentCheck::Exempt
    } else {
        ConsentCheck::Required
    };

    enum Source {
        Token(RefreshToken),
        Device(RefreshTokenDevice),
    }

    // Both kinds of refresh tokens are kept until their replacement exists, to fence the rotation.
    // A failed row check is permanent for the token -> it is deleted.
    let now_plus_skew = Utc::now().add(clock_skew_secs).timestamp();
    let (rt_scope, session_id, source) = if let Some(device_id) = claims.common.did {
        let mut rt = RefreshTokenDevice::find_opt(validation_str)
            .await?
            .ok_or_else(|| {
                ErrorResponse::new(
                    ErrorResponseType::NotFound,
                    "Device Refresh Token not found",
                )
            })?;

        let check = if rt.exp < now_plus_skew {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Device Refresh Token has expired",
            ))
        } else if rt.device_id != device_id {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "'device_id' does not match",
            ))
        } else if rt.user_id != user.id {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "'user_id' does not match",
            ))
        } else {
            Ok(())
        };
        if let Err(err) = check {
            if let Err(err) = rt.delete().await {
                error!(?err, "deleting the refresh token of a failed refresh");
            }
            return Err(err);
        }

        let device = match DeviceEntity::find(device_id).await {
            Err(err) if err.error == ErrorResponseType::NotFound => {
                if let Err(err) = rt.delete().await {
                    error!(?err, "deleting the refresh token of a failed refresh");
                }
                return Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "Invalid refresh token",
                ));
            }
            Err(err) => return Err(err),
            Ok(device) => device,
        };
        if !device.is_for_client(&client) {
            // cascades to the device's refresh tokens
            if let Err(err) = DeviceEntity::invalidate(device_id).await {
                error!(?err, "deleting device authorized for a deleted client");
                if let Err(err) = rt.delete().await {
                    error!(?err, "deleting the refresh token of a failed refresh");
                }
            }
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid refresh token",
            ));
        }

        (rt.scope.take(), None, Source::Device(rt))
    } else {
        let mut rt = RefreshToken::find_opt(validation_str)
            .await?
            .ok_or_else(|| {
                ErrorResponse::new(ErrorResponseType::NotFound, "Refresh Token not found")
            })?;

        let check = if rt.exp < now_plus_skew {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Refresh Token has expired",
            ))
        } else if rt.user_id != user.id {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "'user_id' does not match",
            ))
        } else if rt.client_id.as_deref().is_some_and(|c| c != client.id) {
            // `client_id` is NULL for legacy rows issued before the column existed -> accept those
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "'client_id' does not match",
            ))
        } else if !rt.is_for_client(&client) {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid refresh token",
            ))
        } else {
            Ok(())
        };
        if let Err(err) = check {
            if let Err(err) = rt.delete().await {
                error!(?err, "deleting the refresh token of a failed refresh");
            }
            return Err(err);
        }

        let rt_scope = rt.scope.take();
        let vars = &RauthyConfig::get().vars.consent;
        let rt_scope = if vars.enable && consent == ConsentCheck::Required {
            if vars.requires_resource() && client.is_ephemeral() && claims.resource.is_none() {
                tracing::warn!("refresh token without a `resource` for a client that requires one");
                return Err(ErrorResponse::new(
                    ErrorResponseType::InvalidGrant,
                    "a `resource` is required",
                ));
            }
            let Some(grant) = UserConsent::find(&user.id, &client.id, claims.resource).await?
            else {
                tracing::warn!("no consent grant for the refresh token");
                // it may have escaped a revocation and must not come back with a new consent
                RefreshToken::discard(&rt.id).await?;
                return Err(ErrorResponse::new(
                    ErrorResponseType::InvalidGrant,
                    "no consent grant for this client",
                ));
            };
            // `None` means the client's `default_scopes`, which the grant never narrows
            match rt_scope {
                None => None,
                Some(rt_scope) => {
                    let scope = effective_scope(&rt_scope, &grant, &client);
                    if scope.is_empty() {
                        tracing::warn!(
                            "no scope of the refresh token is covered by the consent grant"
                        );
                        return Err(ErrorResponse::new(
                            ErrorResponseType::InvalidGrant,
                            "the consent grant covers no scope of the refresh token",
                        ));
                    }
                    Some(scope)
                }
            }
        } else {
            rt_scope
        };

        (
            rt_scope,
            rt.session_id.take().map(SessionId),
            Source::Token(rt),
        )
    };

    // at this point, everything has been validated -> we can issue a new TokenSet safely
    debug!("Refresh Token - all good!");

    // A token refresh is NOT a fresh user authentication, so `last_login` must not be bumped
    // here - otherwise `auth_time` (derived from `last_login`) would advance on every refresh.
    // The `auth_time` is carried on the refresh token claims instead (see below).

    let auth_time = if let Some(ts) = claims.auth_time {
        AuthTime::given(ts)
    } else {
        // This is not 100% correct but will make the migration from older to new refresh
        // tokens smooth. In a future release, the `auth_time` can be set to 'required' in the
        // claims. Optional for now to still accept older tokens.
        // As soon as no old refresh tokens exist anymore, this branch will never be used anyway.
        AuthTime::now()
    };

    let result = TokenSet::from_user(
        &user,
        &client,
        auth_time,
        dpop_fingerprint,
        None,
        rt_scope.map(TokenScopes::new),
        session_id,
        // carry the granted resource forward so the refreshed access token keeps its
        // audience binding; a refresh can never widen it
        claims.resource.map(String::from),
        AuthCodeFlow::No,
        // TODO I guess we need to provide the ID if this is a refresh from a device flow?
        DeviceCodeFlow::No,
        consent,
    )
    .await;
    let ts = match result {
        Err(err) => {
            let deleted = match &source {
                Source::Token(rt) => rt.delete().await,
                Source::Device(rt) => rt.delete().await,
            };
            if let Err(err) = deleted {
                error!(?err, "deleting the refresh token of a failed refresh");
            }
            return Err(consent_as_invalid_grant(err));
        }
        Ok(ts) => ts,
    };

    let replacement_id = ts
        .refresh_token
        .as_deref()
        .map(|replacement| {
            replacement
                .len()
                .checked_sub(REFRESH_TOKEN_VALIDATION_LEN)
                .and_then(|start| replacement.get(start..))
                .ok_or_else(|| {
                    ErrorResponse::new(ErrorResponseType::Internal, "Invalid rotated refresh token")
                })
        })
        .transpose()?;
    match source {
        Source::Token(rt) => {
            let Some(replacement_id) = replacement_id else {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    "Missing rotated refresh token",
                ));
            };
            rt.finish_rotation(replacement_id).await?;
        }
        Source::Device(rt) => match replacement_id {
            // nothing to fence: the device token is consumed
            None => rt.delete_checked().await?,
            Some(replacement_id) => rt.finish_rotation(replacement_id).await?,
        },
    }

    if RauthyConfig::get().vars.events.generate_token_issued {
        Event::token_issued("refresh", &client.id, Some(&user.email))
            .send()
            .await?;
    }

    Ok((ts, dpop_nonce))
}
