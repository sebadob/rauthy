use crate::oidc::consent_grant::{RefreshGrant, refresh_consent_check};
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
use rauthy_data::entity::users::User;
use rauthy_data::events::event::Event;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_jwt::claims::{JwtRefreshClaims, JwtTokenType};
use rauthy_jwt::token::JwtToken;
use std::future::Future;
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
        let err = ErrorResponse::new(ErrorResponseType::Forbidden, "Mismatch in client.id");
        if claims.common.did.is_some()
            && let Ok(Some(rt)) = RefreshTokenDevice::find_opt(validation_str).await
        {
            return Err(delete_after(rt.delete(), err).await);
        } else if let Ok(Some(rt)) = RefreshToken::find_opt(validation_str).await {
            return Err(delete_after(rt.delete(), err).await);
        }
        return Err(err);
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

    // Both kinds of refresh tokens are kept until their replacement exists. A deletion of the
    // source that commits before `finish_rotation` deletes it makes `finish_rotation` remove the
    // replacement. Whether the deleting side sees a replacement whose source the refresh deleted
    // first depends on its statement order: a consent revoke covers the device sources, with
    // `consent.enable` also the legacy ones, and every source it locks, but not a replacement
    // rotated again while it runs (`pg_revoke_steps`); a logout covers the device refresh tokens.
    let mut source = if let Some(device_id) = claims.common.did {
        let rt = RefreshTokenDevice::find_opt(validation_str)
            .await?
            .ok_or_else(|| {
                ErrorResponse::new(
                    ErrorResponseType::NotFound,
                    "Device Refresh Token not found",
                )
            })?;
        RotationSource::Device(rt, device_id)
    } else {
        let rt = RefreshToken::find_opt(validation_str)
            .await?
            .ok_or_else(|| {
                ErrorResponse::new(ErrorResponseType::NotFound, "Refresh Token not found")
            })?;
        RotationSource::Token(rt)
    };

    // a failed row check is permanent for this token -> it is deleted
    let now_plus_skew = Utc::now().add(clock_skew_secs).timestamp();
    if let Err(err) = source.check_row(&user.id, &client, now_plus_skew) {
        return Err(source.delete_after(err).await);
    }

    if let RotationSource::Device(_, device_id) = &source {
        let device = require_device(DeviceEntity::find(device_id).await, || {
            source.delete_after(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid refresh token",
            ))
        })
        .await?;
        if !device.is_for_client(&client) {
            // cascades to the device's refresh tokens
            if let Err(err) = DeviceEntity::invalidate(device_id).await {
                error!(?err, "deleting device authorized for a deleted client");
            }
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid refresh token",
            ));
        }
    }

    let (rt_scope, session_id) = match &mut source {
        RotationSource::Device(rt, _) => (rt.scope.take(), None),
        RotationSource::Token(rt) => (rt.scope.take(), rt.session_id.clone().map(SessionId)),
    };

    let consent = claims_consent_check(&claims);
    // after the refresh token row checks and before the rotation. A missing grant is permanent
    // for this token like the failed row checks above, so the source is deleted and its access
    // token revoked: it may have escaped a revocation and must not come back once the user
    // consents again. A failed lookup keeps it.
    let grant = RefreshGrant::lookup(consent, &client, &user.id, claims.resource).await?;
    let grant = RefreshGrant::require(grant, || source.discard()).await?;
    let rt_scope = grant.scope(rt_scope, &client.get_default_scopes())?;

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
        Ok(ts) => ts,
        Err(err) => return Err(source.delete_after(err).await),
    };
    source.finish_rotation(ts.refresh_token.as_deref()).await?;

    if RauthyConfig::get().vars.events.generate_token_issued {
        Event::token_issued("refresh", &client.id, Some(&user.email))
            .send()
            .await?;
    }

    Ok((ts, dpop_nonce))
}

/// With `consent.enable`, the user's consent grant must still exist, unless the token comes from
/// a grant without a consent step (`consent_exempt`, or a signed `did` of a device token issued
/// before `consent_exempt` existed). The rotation re-issues the token without a `did`
/// (`DeviceCodeFlow::No`), but with `consent_exempt` set from the returned check, so a device
/// token stays exempt.
fn claims_consent_check(claims: &JwtRefreshClaims) -> ConsentCheck {
    refresh_consent_check(claims.consent_exempt, claims.common.did.is_some())
}

/// The row checks of a device refresh token. On `Err`, the token is deleted.
fn check_device_row(
    rt: &RefreshTokenDevice,
    device_id: &str,
    user_id: &str,
    now_plus_skew: i64,
) -> Result<(), ErrorResponse> {
    if rt.exp < now_plus_skew {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Device Refresh Token has expired",
        ));
    }
    if rt.device_id != device_id {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "'device_id' does not match",
        ));
    }
    if rt.user_id != user_id {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "'user_id' does not match",
        ));
    }
    Ok(())
}

/// The row checks of a refresh token. On `Err`, the token is deleted.
fn check_token_row(
    rt: &RefreshToken,
    user_id: &str,
    client: &Client,
    now_plus_skew: i64,
) -> Result<(), ErrorResponse> {
    if rt.exp < now_plus_skew {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Refresh Token has expired",
        ));
    }
    if rt.user_id != user_id {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "'user_id' does not match",
        ));
    }
    // `client_id` is NULL for legacy rows issued before the column existed -> accept those
    if rt.client_id.as_deref().is_some_and(|c| c != client.id) {
        return Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "'client_id' does not match",
        ));
    }
    if !rt.is_for_client(client) {
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Invalid refresh token",
        ));
    }
    Ok(())
}

/// The device of a device refresh token, from `DeviceEntity::find`. A missing device means the
/// token is invalid: `discard` deletes it and returns the error to answer with. Any other lookup
/// error keeps the token and is returned as is.
async fn require_device<T, D, Fut>(
    found: Result<T, ErrorResponse>,
    discard: D,
) -> Result<T, ErrorResponse>
where
    D: FnOnce() -> Fut,
    Fut: Future<Output = ErrorResponse>,
{
    match found {
        Ok(device) => Ok(device),
        Err(err) if err.error == ErrorResponseType::NotFound => Err(discard().await),
        Err(err) => Err(err),
    }
}

/// Awaits `delete` and returns `err`, the cause of the failed refresh. A failed `delete` is
/// logged, it must not mask `err`.
async fn delete_after<D>(delete: D, err: ErrorResponse) -> ErrorResponse
where
    D: Future<Output = Result<(), ErrorResponse>>,
{
    if let Err(delete_err) = delete.await {
        error!(
            ?delete_err,
            "deleting the refresh token of a failed refresh"
        );
    }
    err
}

/// How a rotation ends, from the refresh token of the new token set.
#[derive(Debug, PartialEq)]
enum RotationEnd<'a> {
    /// The validation id of the replacement refresh token
    Replace(&'a str),
    /// No replacement: the source is consumed
    Consume,
}

/// `replacement` is the refresh token of the new token set. Only a device token can be rotated
/// without a replacement.
fn rotation_end(device: bool, replacement: Option<&str>) -> Result<RotationEnd<'_>, ErrorResponse> {
    match replacement {
        Some(replacement) => replacement
            .len()
            .checked_sub(REFRESH_TOKEN_VALIDATION_LEN)
            .and_then(|start| replacement.get(start..))
            .map(RotationEnd::Replace)
            .ok_or_else(|| {
                ErrorResponse::new(ErrorResponseType::Internal, "Invalid rotated refresh token")
            }),
        // nothing to fence: the device token is consumed, as before without a replacement
        None if device => Ok(RotationEnd::Consume),
        None => Err(ErrorResponse::new(
            ErrorResponseType::Internal,
            "Missing rotated refresh token",
        )),
    }
}

/// The refresh token a refresh is made with. It is kept until its replacement exists, so that a
/// logout, consent revocation or device deletion committing in between, which cannot see the
/// replacement yet, still fences the rotation: `finish_rotation` finds the source gone and
/// removes the replacement.
enum RotationSource<'a> {
    Token(RefreshToken),
    /// The token and the signed `did` of its claims
    Device(RefreshTokenDevice, &'a str),
}

impl RotationSource<'_> {
    fn check_row(
        &self,
        user_id: &str,
        client: &Client,
        now_plus_skew: i64,
    ) -> Result<(), ErrorResponse> {
        match self {
            Self::Token(rt) => check_token_row(rt, user_id, client, now_plus_skew),
            Self::Device(rt, device_id) => check_device_row(rt, device_id, user_id, now_plus_skew),
        }
    }

    async fn delete(&self) -> Result<(), ErrorResponse> {
        match self {
            Self::Token(rt) => rt.delete().await,
            Self::Device(rt, _) => rt.delete().await,
        }
    }

    /// Deletes the source after a failed check or issuance and returns `err`, see
    /// [`delete_after`].
    async fn delete_after(&self, err: ErrorResponse) -> ErrorResponse {
        delete_after(self.delete(), err).await
    }

    /// Deletes the source for good and revokes the access token issued with it, in one
    /// transaction, see [`RefreshToken::discard`]: on Postgres the row is deleted first and the
    /// access token it named revoked after (`PG_DISCARD_STEPS`), on hiqlite the access token is
    /// revoked while the row still names it, then the row deleted.
    async fn discard(&self) -> Result<(), ErrorResponse> {
        match self {
            Self::Token(rt) => RefreshToken::discard(&rt.id).await,
            Self::Device(rt, _) => RefreshTokenDevice::discard(&rt.id).await,
        }
    }

    /// `replacement` is the refresh token of the new token set.
    async fn finish_rotation(&self, replacement: Option<&str>) -> Result<(), ErrorResponse> {
        let end = rotation_end(matches!(self, Self::Device(..)), replacement)?;
        match (self, end) {
            (Self::Token(rt), RotationEnd::Replace(id)) => rt.finish_rotation(id).await,
            (Self::Device(rt, _), RotationEnd::Replace(id)) => rt.finish_rotation(id).await,
            (Self::Device(rt, _), RotationEnd::Consume) => rt.delete_checked().await,
            // `rotation_end` never consumes a `Token`
            (Self::Token(_), RotationEnd::Consume) => Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Missing rotated refresh token",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    const NOW: i64 = 1_700_000_000;

    fn device_rt() -> RefreshTokenDevice {
        RefreshTokenDevice {
            id: "rt1".to_string(),
            device_id: "d1".to_string(),
            user_id: "u1".to_string(),
            nbf: NOW - 10,
            exp: NOW + 60,
            scope: None,
            access_token_jti: None,
        }
    }

    fn token_rt() -> RefreshToken {
        RefreshToken {
            id: "rt1".to_string(),
            user_id: "u1".to_string(),
            nbf: NOW - 10,
            exp: NOW + 60,
            scope: None,
            is_mfa: false,
            session_id: None,
            access_token_jti: None,
            client_id: Some("c1".to_string()),
            client_created_micros: Some(1),
        }
    }

    fn client() -> Client {
        Client {
            id: "c1".to_string(),
            created_micros: Some(1),
            ..Default::default()
        }
    }

    fn err(typ: ErrorResponseType) -> ErrorResponse {
        ErrorResponse::new(typ, "test")
    }

    #[test]
    fn test_check_device_row() {
        assert!(check_device_row(&device_rt(), "d1", "u1", NOW).is_ok());

        let mut rt = device_rt();
        rt.exp = NOW - 1;
        let e = check_device_row(&rt, "d1", "u1", NOW).unwrap_err();
        assert_eq!(e.error, ErrorResponseType::Forbidden);
        assert_eq!(e.message, "Device Refresh Token has expired");

        let e = check_device_row(&device_rt(), "d2", "u1", NOW).unwrap_err();
        assert_eq!(e.error, ErrorResponseType::Forbidden);
        assert_eq!(e.message, "'device_id' does not match");

        let e = check_device_row(&device_rt(), "d1", "u2", NOW).unwrap_err();
        assert_eq!(e.error, ErrorResponseType::Forbidden);
        assert_eq!(e.message, "'user_id' does not match");
    }

    #[test]
    fn test_check_token_row() {
        assert!(check_token_row(&token_rt(), "u1", &client(), NOW).is_ok());
        // legacy rows without a `client_id` match any client
        let mut rt = token_rt();
        rt.client_id = None;
        rt.client_created_micros = None;
        assert!(check_token_row(&rt, "u1", &client(), NOW).is_ok());

        let mut rt = token_rt();
        rt.exp = NOW - 1;
        let e = check_token_row(&rt, "u1", &client(), NOW).unwrap_err();
        assert_eq!(e.error, ErrorResponseType::Forbidden);
        assert_eq!(e.message, "Refresh Token has expired");

        let e = check_token_row(&token_rt(), "u2", &client(), NOW).unwrap_err();
        assert_eq!(e.error, ErrorResponseType::Forbidden);
        assert_eq!(e.message, "'user_id' does not match");

        let mut rt = token_rt();
        rt.client_id = Some("c2".to_string());
        let e = check_token_row(&rt, "u1", &client(), NOW).unwrap_err();
        assert_eq!(e.error, ErrorResponseType::Forbidden);
        assert_eq!(e.message, "'client_id' does not match");

        // issued for a deleted client with the same id
        let mut rt = token_rt();
        rt.client_created_micros = Some(2);
        let e = check_token_row(&rt, "u1", &client(), NOW).unwrap_err();
        assert_eq!(e.error, ErrorResponseType::Unauthorized);
    }

    /// Only a missing device makes the device refresh token invalid. Any other lookup error
    /// (a database or connection failure) keeps the token and is returned as is.
    #[tokio::test]
    async fn test_require_device() {
        let discarded = Cell::new(0);
        let discard = || {
            discarded.set(discarded.get() + 1);
            async { err(ErrorResponseType::Unauthorized) }
        };
        assert_eq!(require_device(Ok(1), discard).await.unwrap(), 1);
        assert_eq!(discarded.get(), 0);

        for typ in [ErrorResponseType::Database, ErrorResponseType::Connection] {
            let discard = || {
                discarded.set(discarded.get() + 1);
                async { err(ErrorResponseType::Unauthorized) }
            };
            let e = require_device::<(), _, _>(Err(err(typ.clone())), discard)
                .await
                .unwrap_err();
            assert_eq!(e.error, typ);
            assert_eq!(discarded.get(), 0, "{typ:?} must keep the token");
        }

        let discard = || {
            discarded.set(discarded.get() + 1);
            async { err(ErrorResponseType::Unauthorized) }
        };
        let e = require_device::<(), _, _>(Err(err(ErrorResponseType::NotFound)), discard)
            .await
            .unwrap_err();
        assert_eq!(e.error, ErrorResponseType::Unauthorized);
        assert_eq!(discarded.get(), 1);
    }

    /// A failed delete after a failed check or issuance must not mask the original error.
    #[tokio::test]
    async fn test_delete_after_keeps_the_cause() {
        let deleted = Cell::new(false);
        let delete = async {
            deleted.set(true);
            Ok(())
        };
        let e = delete_after(delete, err(ErrorResponseType::Internal)).await;
        assert_eq!(e.error, ErrorResponseType::Internal);
        assert!(deleted.get());

        let delete = async { Err(err(ErrorResponseType::Database)) };
        let e = delete_after(delete, err(ErrorResponseType::Internal)).await;
        assert_eq!(e.error, ErrorResponseType::Internal);
    }

    #[test]
    fn test_rotation_end() {
        let pad = "x".repeat(10);
        let id = "v".repeat(REFRESH_TOKEN_VALIDATION_LEN);
        let token = format!("{pad}{id}");

        // device arms: replaced like a regular token, or consumed without a replacement
        assert_eq!(
            rotation_end(true, Some(&token)).unwrap(),
            RotationEnd::Replace(&id)
        );
        assert_eq!(rotation_end(true, None).unwrap(), RotationEnd::Consume);
        // token arms: a replacement is required
        assert_eq!(
            rotation_end(false, Some(&token)).unwrap(),
            RotationEnd::Replace(&id)
        );
        let e = rotation_end(false, None).unwrap_err();
        assert_eq!(e.error, ErrorResponseType::Internal);
        // a replacement shorter than its validation id
        for device in [true, false] {
            let e = rotation_end(device, Some("short")).unwrap_err();
            assert_eq!(e.error, ErrorResponseType::Internal);
        }
    }

    /// A device refresh token is exempt from the consent check: issued with `consent_exempt`,
    /// or before that claim existed with a signed `did`. The rotated token carries no `did`,
    /// but `consent_exempt` from this check, so it stays exempt.
    #[test]
    fn test_device_refresh_consent_exempt() {
        let claims_json = |extra: &str| {
            format!(
                r#"{{"iat":1,"nbf":1,"exp":2,"iss":"iss","aud":"c1","typ":"Refresh","azp":"c1",
                "uid":"u1"{extra}}}"#
            )
        };
        let check = |extra: &str| {
            let json = claims_json(extra);
            let claims: JwtRefreshClaims = serde_json::from_str(&json).unwrap();
            claims_consent_check(&claims)
        };

        // a device token
        assert_eq!(
            check(r#","did":"d1","consent_exempt":true"#),
            ConsentCheck::Exempt
        );
        // a device token from before `consent_exempt`
        assert_eq!(check(r#","did":"d1""#), ConsentCheck::Exempt);
        // its rotation: no `did`, `consent_exempt` kept
        assert_eq!(check(r#","consent_exempt":true"#), ConsentCheck::Exempt);
        // an authorization_code token
        assert_eq!(check(""), ConsentCheck::Required);
    }
}
