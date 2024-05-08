use crate::token_set::{
    AtHash, AuthCodeFlow, DeviceCodeFlow, DpopFingerprint, TokenNonce, TokenScopes, TokenSet,
};
use actix_web::http::header;
use actix_web::http::header::{HeaderMap, HeaderName, HeaderValue};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use cryptr::{EncKeys, EncValue};
use jwt_simple::algorithms::{
    EdDSAKeyPairLike, EdDSAPublicKeyLike, RSAKeyPairLike, RSAPublicKeyLike,
};
use jwt_simple::claims;
use jwt_simple::prelude::*;
use rauthy_common::constants::{
    CACHE_NAME_12HR, CACHE_NAME_LOGIN_DELAY, COOKIE_MFA, DEVICE_GRANT_POLL_INTERVAL,
    DEVICE_GRANT_REFRESH_TOKEN_LIFETIME, ENABLE_SOLID_AUD, ENABLE_WEB_ID, HEADER_DPOP_NONCE,
    IDX_JWKS, IDX_JWK_LATEST, IDX_LOGIN_TIME, REFRESH_TOKEN_LIFETIME, SESSION_LIFETIME,
    SESSION_RENEW_MFA, TOKEN_BEARER, USERINFO_STRICT, WEBAUTHN_REQ_EXP,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::password_hasher::HashPassword;
use rauthy_common::utils::{base64_url_encode, get_client_ip, get_rand, new_store_id};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::auth_codes::AuthCode;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::clients_dyn::ClientDyn;
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::entity::devices::{DeviceAuthCode, DeviceEntity};
use rauthy_models::entity::dpop_proof::DPoPProof;
use rauthy_models::entity::jwk::{Jwk, JwkKeyPair, JwkKeyPairAlg};
use rauthy_models::entity::refresh_tokens::RefreshToken;
use rauthy_models::entity::refresh_tokens_devices::RefreshTokenDevice;
use rauthy_models::entity::scopes::Scope;
use rauthy_models::entity::sessions::{Session, SessionState};
use rauthy_models::entity::users::{AccountType, User};
use rauthy_models::entity::users_values::UserValues;
use rauthy_models::entity::webauthn::{WebauthnCookie, WebauthnLoginReq};
use rauthy_models::entity::webids::WebId;
use rauthy_models::events::event::Event;
use rauthy_models::events::ip_blacklist_handler::{IpBlacklistReq, IpFailedLoginCheck};
use rauthy_models::language::Language;
use rauthy_models::request::{LoginRefreshRequest, LoginRequest, LogoutRequest, TokenRequest};
use rauthy_models::response::{OAuth2ErrorResponse, OAuth2ErrorTypeResponse, TokenInfo, Userinfo};
use rauthy_models::templates::{LogoutHtml, TooManyRequestsHtml};
use rauthy_models::{
    sign_jwt, validate_jwt, AddressClaim, AuthStep, AuthStepAwaitWebauthn, AuthStepLoggedIn,
    JktClaim, JwtAccessClaims, JwtAmrValue, JwtCommonClaims, JwtIdClaims, JwtRefreshClaims,
    JwtTokenType,
};
use redhac::cache_del;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_put};
use ring::digest;
use std::borrow::Cow;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;
use tokio::sync::oneshot;
use tracing::{debug, error, info, trace, warn};

/// # Business logic for [POST /oidc/authorize](crate::handlers::post_authorize)
#[tracing::instrument(name = "post_authorize", skip_all, fields(client_id = req_data.client_id, email = req_data.email))]
pub async fn authorize(
    data: &web::Data<AppState>,
    req: &HttpRequest,
    req_data: LoginRequest,
    mut session: Session,
    // the second argument with the error will be 'true' if a login delay should be added
) -> Result<AuthStep, (ErrorResponse, bool)> {
    // This Error must be the same if user does not exist AND passwords do not match to prevent
    // username enumeration
    let mut user = User::find_by_email(data, req_data.email)
        .await
        .map_err(|e| {
            error!("{:?}", e);
            // be careful, that this Err and the one in User::validate_password are exactly the same
            (
                ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    String::from("Invalid user credentials"),
                ),
                false,
            )
        })?;

    let mfa_cookie = if let Ok(c) = WebauthnCookie::parse_validate(&req.cookie(COOKIE_MFA)) {
        if c.email == user.email && user.has_webauthn_enabled() {
            Some(c)
        } else {
            // If a possibly existing mfa cookie does not match the given email, or user has webauthn
            // disabled in the meantime -> ignore the cookie
            None
        }
    } else {
        None
    };

    let account_type = user.account_type();

    // this allows a user without the mfa cookie to login anyway if it is an only passkey account
    // in this case, UV is always enforced, not matter what -> safe to login without cookie
    let user_must_provide_password =
        req_data.password.is_none() && account_type != AccountType::Passkey && mfa_cookie.is_none();
    if user_must_provide_password {
        trace!("No user password has been provided");
        return Err((
            ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                String::from("Invalid user credentials"),
            ),
            false,
        ));
    }

    if account_type == AccountType::New {
        return Err((
            ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                String::from("Invalid user credentials"),
            ),
            // this basically means, if the user did the first login in the UI with just username,
            // do not add any login delay afterwards for a better UX
            !user_must_provide_password,
        ));
    }

    user.check_enabled()
        .map_err(|err| (err, !user_must_provide_password))?;
    user.check_expired()
        .map_err(|err| (err, !user_must_provide_password))?;

    let has_password_been_hashed = if let Some(pwd) = req_data.password {
        match user.validate_password(data, pwd).await {
            Ok(_) => {
                // update user info
                // in case of webauthn login, the info will be updates in the auth finish step
                user.last_login = Some(OffsetDateTime::now_utc().unix_timestamp());
                user.last_failed_login = None;
                user.failed_login_attempts = None;
                user.save(data, None, None)
                    .await
                    .map_err(|err| (err, true))?;
            }
            Err(err) => {
                trace!("Provided user password is invalid");
                return Err((err, true));
            }
        }
        true
    } else {
        false
    };

    // client validations
    let client = Client::find_maybe_ephemeral(data, req_data.client_id)
        .await
        .map_err(|err| (err, !user_must_provide_password))?;
    client
        .validate_mfa(&user)
        .map_err(|err| (err, has_password_been_hashed))?;
    client
        .validate_redirect_uri(&req_data.redirect_uri)
        .map_err(|err| (err, !user_must_provide_password))?;
    client
        .validate_code_challenge(&req_data.code_challenge, &req_data.code_challenge_method)
        .map_err(|err| (err, !user_must_provide_password))?;
    let header_origin = client
        .validate_origin(req, &data.listen_scheme, &data.public_url)
        .map_err(|err| (err, !user_must_provide_password))?;

    // build authorization code
    let code_lifetime = if user.has_webauthn_enabled() {
        client.auth_code_lifetime + *WEBAUTHN_REQ_EXP as i32
    } else {
        client.auth_code_lifetime
    };
    let scopes = client
        .sanitize_login_scopes(&req_data.scopes)
        .map_err(|err| (err, !user_must_provide_password))?;
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
    code.save(data)
        .await
        .map_err(|err| (err, !user_must_provide_password))?;

    // build location header
    let mut loc = format!("{}?code={}", req_data.redirect_uri, code.id);
    if let Some(state) = req_data.state {
        loc = format!("{}&state={}", loc, state);
    };

    // TODO double check that we do not have any problems with the direct webauthn login here
    // TODO should we allow to skip this step if set so in the config?
    // check if we need to validate the 2nd factor
    if user.has_webauthn_enabled() {
        session
            .set_mfa(data, true)
            .await
            .map_err(|err| (err, !user_must_provide_password))?;

        let step = AuthStepAwaitWebauthn {
            has_password_been_hashed,
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
        .save(data)
        .await
        .map_err(|err| (err, !user_must_provide_password))?;

        Ok(AuthStep::AwaitWebauthn(step))
    } else {
        Ok(AuthStep::LoggedIn(AuthStepLoggedIn {
            has_password_been_hashed,
            email: user.email,
            header_loc: (header::LOCATION, HeaderValue::from_str(&loc).unwrap()),
            header_csrf: Session::get_csrf_header(&session.csrf_token),
            header_origin,
        }))
    }
}

/// # Business logic for [POST /oidc/authorize/refresh](crate::handlers::post_authorize_refresh)
pub async fn authorize_refresh(
    data: &web::Data<AppState>,
    session: &Session,
    client: Client,
    header_origin: Option<(HeaderName, HeaderValue)>,
    req_data: LoginRefreshRequest,
) -> Result<AuthStep, ErrorResponse> {
    let user_id = session.user_id.as_ref().ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            String::from("No linked user_id for already validated session"),
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
    code.save(data).await?;

    // build location header
    let header_loc = if let Some(s) = req_data.state {
        format!("{}?code={}&state={}", req_data.redirect_uri, code.id, s)
    } else {
        format!("{}?code={}", req_data.redirect_uri, code.id)
    };

    // check if we need to validate the 2nd factor
    if user.has_webauthn_enabled() && *SESSION_RENEW_MFA {
        let step = AuthStepAwaitWebauthn {
            has_password_been_hashed: false,
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
        login_req.save(data).await?;

        Ok(AuthStep::AwaitWebauthn(step))
    } else {
        Ok(AuthStep::LoggedIn(AuthStepLoggedIn {
            has_password_been_hashed: false,
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

/// Builds the access token for a user after all validation has been successful
// too many arguments is not an issue - params cannot be mistaken because of enum wrappers
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub async fn build_access_token(
    user: Option<&User>,
    data: &web::Data<AppState>,
    client: &Client,
    dpop_fingerprint: Option<DpopFingerprint>,
    lifetime: i64,
    scope: Option<TokenScopes>,
    scope_customs: Option<(Vec<&Scope>, &Option<HashMap<String, Vec<u8>>>)>,
    device_code_flow: DeviceCodeFlow,
) -> Result<String, ErrorResponse> {
    let did = match device_code_flow {
        DeviceCodeFlow::Yes(did) => Some(did),
        DeviceCodeFlow::No => None,
    };
    let mut custom_claims = JwtAccessClaims {
        typ: JwtTokenType::Bearer,
        azp: client.id.to_string(),
        scope: scope
            .map(|s| s.0)
            .unwrap_or_else(|| client.default_scopes.clone().replace(',', " ")),
        allowed_origins: None,
        did,
        email: None,
        preferred_username: None,
        roles: None,
        groups: None,
        cnf: dpop_fingerprint.map(|jkt| JktClaim { jkt: jkt.0 }),
        custom: None,
    };

    // add user specific claims if available
    let sub = if let Some(user) = user {
        custom_claims.preferred_username = Some(user.email.clone());
        custom_claims.roles = Some(user.get_roles());

        if custom_claims.scope.contains("email") {
            custom_claims.email = Some(user.email.clone());
        }

        if custom_claims.scope.contains("groups") {
            custom_claims.groups = Some(user.get_groups());
        }

        Some(&user.id)
    } else {
        None
    };

    if let Some((cust, user_attrs)) = scope_customs {
        let user_attrs = user_attrs.as_ref().unwrap();
        let mut attr = HashMap::with_capacity(cust.len());
        for c in cust {
            if let Some(csv) = &c.attr_include_access {
                let scopes = csv.split(',');
                for cust_name in scopes {
                    if let Some(value) = user_attrs.get(cust_name) {
                        let json = serde_json::from_slice(value.as_slice())
                            .expect("Converting cust user id attr to json");
                        attr.insert(cust_name.to_string(), json);
                    };
                }
            }
        }
        if !attr.is_empty() {
            custom_claims.custom = Some(attr);
        }
    }

    let mut claims = Claims::with_custom_claims(
        custom_claims,
        coarsetime::Duration::from_secs(lifetime as u64),
    )
    .with_issuer(data.issuer.clone())
    .with_audience(client.id.to_string());

    if let Some(sub) = sub {
        claims = claims.with_subject(sub);
    }

    sign_access_token(data, claims, client).await
}

/// Builds the id token for a user after all validation has been successful
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub async fn build_id_token(
    user: &User,
    data: &web::Data<AppState>,
    client: &Client,
    dpop_fingerprint: Option<DpopFingerprint>,
    at_hash: AtHash,
    lifetime: i64,
    nonce: Option<TokenNonce>,
    scope: &str,
    scope_customs: Option<(Vec<&Scope>, &Option<HashMap<String, Vec<u8>>>)>,
    auth_code_flow: AuthCodeFlow,
) -> Result<String, ErrorResponse> {
    let now_ts = Utc::now().timestamp();

    // TODO the `auth_time` here is a bit inaccurate currently. The accuracy could be improved
    // with future DB migrations by adding something like a `last_auth` column for each user.
    // It is unclear right now, if we even need it right now.
    let (amr, auth_time) = match user.has_webauthn_enabled() {
        true => {
            if auth_code_flow == AuthCodeFlow::Yes {
                // With active MFA, the auth_time is always 'now', because it must be re-validated each time
                (JwtAmrValue::Mfa.to_string(), now_ts)
            } else {
                (
                    JwtAmrValue::Pwd.to_string(),
                    now_ts - *SESSION_LIFETIME as i64,
                )
            }
        }
        false => {
            if auth_code_flow == AuthCodeFlow::Yes {
                (JwtAmrValue::Pwd.to_string(), now_ts)
            } else {
                (
                    JwtAmrValue::Pwd.to_string(),
                    now_ts - *SESSION_LIFETIME as i64,
                )
            }
        }
    };

    let webid =
        (*ENABLE_WEB_ID && scope.contains("webid")).then(|| WebId::resolve_webid_uri(&user.id));

    let mut custom_claims = JwtIdClaims {
        azp: client.id.clone(),
        typ: JwtTokenType::Id,
        amr: vec![amr],
        auth_time,
        at_hash: at_hash.0,
        preferred_username: user.email.clone(),
        email: None,
        email_verified: None,
        given_name: None,
        family_name: None,
        address: None,
        birthdate: None,
        locale: None,
        phone: None,
        roles: user.get_roles(),
        groups: None,
        cnf: dpop_fingerprint.map(|jkt| JktClaim { jkt: jkt.0 }),
        custom: None,
        webid,
    };

    let mut user_values = None;
    let mut user_values_fetched = false;

    if scope.contains("email") {
        custom_claims.email = Some(user.email.clone());
        custom_claims.email_verified = Some(user.email_verified);
    }

    if scope.contains("profile") {
        custom_claims.given_name = Some(user.given_name.clone());
        custom_claims.family_name = Some(user.family_name.clone());
        custom_claims.locale = Some(user.language.to_string());

        user_values = UserValues::find(data, &user.id).await?;
        user_values_fetched = true;

        if let Some(values) = &user_values {
            if let Some(birthdate) = &values.birthdate {
                custom_claims.birthdate = Some(birthdate.clone());
            }
        }
    }

    if scope.contains("address") {
        if !user_values_fetched {
            user_values = UserValues::find(data, &user.id).await?;
            user_values_fetched = true;
        }

        if let Some(values) = &user_values {
            custom_claims.address = AddressClaim::try_build(user, values);
        }
    }

    if scope.contains("phone") {
        if !user_values_fetched {
            user_values = UserValues::find(data, &user.id).await?;
            // user_values_fetched = true;
        }

        if let Some(values) = &user_values {
            if let Some(phone) = &values.phone {
                custom_claims.phone = Some(phone.clone());
            }
        }
    }

    if scope.contains("groups") {
        custom_claims.groups = Some(user.get_groups());
    }

    if let Some((cust, user_attrs)) = scope_customs {
        let user_attrs = user_attrs.as_ref().unwrap();
        let mut attr = HashMap::with_capacity(cust.len());
        for c in cust {
            if let Some(csv) = &c.attr_include_id {
                let scopes = csv.split(',');
                for cust_name in scopes {
                    if let Some(value) = user_attrs.get(cust_name) {
                        let json = serde_json::from_slice(value.as_slice())
                            .expect("Converting cust user id attr to json");
                        attr.insert(cust_name.to_string(), json);
                    };
                }
            }
        }
        if !attr.is_empty() {
            custom_claims.custom = Some(attr);
        }
    }

    let mut claims = Claims::with_custom_claims(
        custom_claims,
        coarsetime::Duration::from_secs(lifetime as u64),
    )
    .with_subject(user.id.clone())
    .with_issuer(data.issuer.clone());

    // TODO should we maybe always include the "solid" claim here depending on if a webid exists?
    // like it is now, static clients would never include this claim, even though they might need it
    if client.is_ephemeral() && *ENABLE_SOLID_AUD {
        let mut aud = HashSet::with_capacity(2);
        aud.insert("solid".to_string());
        aud.insert(client.id.to_string());
        claims = claims.with_audiences(aud);
    } else {
        claims = claims.with_audience(client.id.to_string());
    }

    if let Some(nonce) = nonce {
        claims = claims.with_nonce(nonce.0);
    }

    sign_id_token(data, claims, client).await
}

/// Builds the refresh token for a user after all validation has been successful
#[allow(clippy::too_many_arguments)]
pub async fn build_refresh_token(
    user: &User,
    data: &web::Data<AppState>,
    dpop_fingerprint: Option<DpopFingerprint>,
    client: &Client,
    access_token_lifetime: i64,
    scope: Option<TokenScopes>,
    is_mfa: bool,
    device_code_flow: DeviceCodeFlow,
) -> Result<String, ErrorResponse> {
    let did = if let DeviceCodeFlow::Yes(device_id) = device_code_flow {
        Some(device_id)
    } else {
        None
    };

    let custom_claims = JwtRefreshClaims {
        azp: client.id.clone(),
        typ: JwtTokenType::Refresh,
        uid: user.id.clone(),
        cnf: dpop_fingerprint.map(|jkt| JktClaim { jkt: jkt.0 }),
        did: did.clone(),
    };

    let nbf = Utc::now().add(chrono::Duration::seconds(access_token_lifetime - 60));
    let nbf_unix = UnixTimeStamp::from_secs(nbf.timestamp() as u64);

    let claims = Claims::with_custom_claims(custom_claims, coarsetime::Duration::from_hours(48))
        .with_issuer(data.issuer.clone())
        .invalid_before(nbf_unix)
        .with_audience(client.id.to_string());

    let token = sign_refresh_token(data, claims).await?;

    // only save the last 50 characters for validation
    let validation_string = String::from(&token).split_off(token.len() - 49);

    if let Some(device_id) = did {
        let exp = nbf.add(chrono::Duration::hours(
            *DEVICE_GRANT_REFRESH_TOKEN_LIFETIME as i64,
        ));
        RefreshTokenDevice::create(
            data,
            validation_string,
            device_id,
            user.id.clone(),
            nbf,
            exp,
            scope.map(|s| s.0),
        )
        .await?;
    } else {
        let exp = nbf.add(chrono::Duration::hours(*REFRESH_TOKEN_LIFETIME as i64));
        RefreshToken::create(
            data,
            validation_string,
            user.id.clone(),
            nbf,
            exp,
            scope.map(|s| s.0),
            is_mfa,
        )
        .await?;
    }

    Ok(token)
}

#[inline(always)]
pub fn get_bearer_token_from_header(headers: &HeaderMap) -> Result<String, ErrorResponse> {
    let bearer = headers.get("Authorization").ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            String::from("Bearer Token missing"),
        )
    });
    if bearer.is_err() {
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            String::from("Authorization header missing"),
        ));
    }

    let head_val = bearer?
        .to_str()
        .map_err(|_| {
            ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                String::from("Malformed Authorization Header. Could not extract token."),
            )
        })?
        .to_string();

    let (p, bearer) = head_val.split_once(' ').ok_or(("ERR", "")).map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            String::from("Malformed Authorization Header. Could not extract token."),
        )
    })?;
    if p.ne(TOKEN_BEARER) || bearer.is_empty() {
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            String::from("No bearer token given"),
        ));
    }
    Ok(bearer.to_string())
}

/// Returns the 'userInfo' for the [/oidc/userinfo endpoint](crate::handlers::get_userinfo)<br>
pub async fn get_userinfo(
    data: &web::Data<AppState>,
    req: HttpRequest,
) -> Result<Userinfo, ErrorResponse> {
    // get bearer token
    let bearer = get_bearer_token_from_header(req.headers())?;

    let claims = validate_token::<JwtCommonClaims>(data, &bearer).await?;
    if claims.custom.typ != JwtTokenType::Bearer {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Token Type must be 'Bearer'".to_string(),
        ));
    }

    let scope = claims.custom.scope.unwrap_or_else(|| "openid".to_string());
    let uid = claims.subject.ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            String::from("Token without 'sub' - could not extract the Principal"),
        )
    })?;
    let user = User::find(data, uid).await.map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("user-not-found".to_string()),
            "The user has not been found".to_string(),
        )
    })?;

    // reject the request if user has been disabled, even when the token is still valid
    if !user.enabled || user.check_expired().is_err() {
        return Err(ErrorResponse::new(
            ErrorResponseType::WWWAuthenticate("user-disabled".to_string()),
            "The user has been disabled".to_string(),
        ));
    }

    if *USERINFO_STRICT {
        // if the token has been issued to a device, make sure it still exists and is valid
        if let Some(device_id) = claims.custom.did {
            // just make sure it still exists
            DeviceEntity::find(data, &device_id).await.map_err(|_| {
                ErrorResponse::new(
                    ErrorResponseType::WWWAuthenticate("user-device-not-found".to_string()),
                    "The user device has not been found".to_string(),
                )
            })?;
        }

        // make sure the original client still exists and is enabled
        let client = Client::find(data, claims.custom.azp).await.map_err(|_| {
            ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate("client-not-found".to_string()),
                "The client has not been found".to_string(),
            )
        })?;
        if !client.enabled {
            return Err(ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate("client-disabled".to_string()),
                "The client has been disabled".to_string(),
            ));
        }
    }

    let roles = user.get_roles();
    let groups = scope.contains("groups").then(|| user.get_groups());
    let webid =
        (*ENABLE_WEB_ID && scope.contains("webid")).then(|| WebId::resolve_webid_uri(&user.id));

    let mut userinfo = Userinfo {
        id: user.id.clone(),
        sub: user.id.clone(),
        name: format!("{} {}", &user.given_name, &user.family_name),
        roles,
        mfa_enabled: user.has_webauthn_enabled(),

        // scope: address
        address: None,

        // scope: email
        email: None,
        email_verified: None,

        // scope: groups
        groups,

        // scope: profile
        preferred_username: None,
        given_name: None,
        family_name: None,
        locale: None,
        birthdate: None,

        // scope: phone
        phone: None,

        // scope: webid
        webid,
    };

    if scope.contains("email") {
        userinfo.email = Some(user.email.clone());
        userinfo.email_verified = Some(user.email_verified);
    }

    let mut user_values = None;
    let mut user_values_fetched = false;

    if scope.contains("profile") {
        userinfo.preferred_username = Some(user.email.clone());
        userinfo.given_name = Some(user.given_name.clone());
        userinfo.family_name = Some(user.family_name.clone());
        userinfo.locale = Some(user.language.to_string());

        user_values = UserValues::find(data, &user.id).await?;
        user_values_fetched = true;

        if let Some(values) = &user_values {
            if let Some(birthdate) = &values.birthdate {
                userinfo.birthdate = Some(birthdate.clone());
            }
        }
    }

    if scope.contains("address") {
        if !user_values_fetched {
            user_values = UserValues::find(data, &user.id).await?;
            user_values_fetched = true;
        }

        if let Some(values) = &user_values {
            userinfo.address = AddressClaim::try_build(&user, values);
        }
    }

    if scope.contains("phone") {
        if !user_values_fetched {
            user_values = UserValues::find(data, &user.id).await?;
            // user_values_fetched = true;
        }

        if let Some(values) = &user_values {
            if let Some(phone) = &values.phone {
                userinfo.phone = Some(phone.clone());
            }
        }
    }

    Ok(userinfo)
}

/// Returns [TokenInfo](crate::models::response::TokenInfo) for the
/// [/oidc/tokenInfo endpoint](crate::handlers::post_token_info)
pub async fn get_token_info(
    data: &web::Data<AppState>,
    token: &str,
) -> Result<TokenInfo, ErrorResponse> {
    let claims_res = validate_token::<JwtCommonClaims>(data, token).await;
    if claims_res.is_err() {
        return Ok(TokenInfo {
            active: false,
            scope: None,
            client_id: None,
            username: None,
            exp: None,
            cnf: None,
        });
    }

    let claims = claims_res.unwrap();
    // scope does not exist for ID tokens, for all others unwrap is safe
    let scope = claims.custom.scope;
    let client_id = claims.custom.azp;
    let username = claims.subject;
    let exp = claims.expires_at.unwrap().as_secs();
    let cnf = claims.custom.cnf;

    Ok(TokenInfo {
        active: true,
        scope,
        client_id: Some(client_id),
        username,
        exp: Some(exp),
        cnf,
    })
}

/// Main entrance function for returning a whole new [TokenSet](crate::models::response::TokenSet)
pub async fn get_token_set(
    req_data: TokenRequest,
    data: &web::Data<AppState>,
    req: HttpRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    match req_data.grant_type.as_str() {
        "authorization_code" => grant_type_code(data, req, req_data).await,
        "client_credentials" => grant_type_credentials(data, req, req_data).await,
        "password" => grant_type_password(data, req, req_data).await,
        "refresh_token" => grant_type_refresh(data, req, req_data).await,
        _ => Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("Invalid 'grant_type'"),
        )),
    }
}

/// Return a [TokenSet](crate::models::response::TokenSet) for the `authorization_code` flow
#[tracing::instrument(skip_all, fields(client_id = req_data.client_id, username = req_data.username))]
async fn grant_type_code(
    data: &web::Data<AppState>,
    req: HttpRequest,
    req_data: TokenRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    if req_data.code.is_none() {
        warn!("'code' is missing");
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("'code' is missing"),
        ));
    }

    // check the client for external origin and auth flow
    let (client_id, client_secret) = req_data.try_get_client_id_secret(&req)?;
    let client = Client::find_maybe_ephemeral(data, client_id.clone())
        .await
        .map_err(|_| {
            ErrorResponse::new(
                ErrorResponseType::NotFound,
                format!("Client '{}' not found", client_id),
            )
        })?;
    let header_origin = client.validate_origin(&req, &data.listen_scheme, &data.public_url)?;
    if client.confidential {
        let secret = client_secret.ok_or_else(|| {
            warn!("'client_secret' is missing");
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("'client_secret' is missing"),
            )
        })?;
        client.validate_secret(&secret, &req)?;
    }
    client.validate_flow("authorization_code")?;

    // check for DPoP header
    let mut headers = Vec::new();
    let dpop_fingerprint =
        if let Some(proof) = DPoPProof::opt_validated_from(data, &req, &header_origin).await? {
            if let Some(nonce) = &proof.claims.nonce {
                headers.push((
                    HeaderName::from_str(HEADER_DPOP_NONCE).unwrap(),
                    HeaderValue::from_str(nonce).unwrap(),
                ));
            };
            Some(DpopFingerprint(proof.jwk_fingerprint()?))
        } else {
            None
        };
    if let Some(h) = header_origin {
        headers.push(h);
    }

    // get the auth code from the cache
    let idx = req_data.code.as_ref().unwrap().to_owned();
    let code = AuthCode::find(data, idx).await?.ok_or_else(|| {
        warn!(
            "'auth_code' could not be found inside the cache - Host: {}",
            get_client_ip(&req),
        );
        ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "'auth_code' could not be found inside the cache".to_string(),
        )
    })?;
    // validate the auth code
    if code.client_id != client_id {
        let err = format!("Wrong 'code' for client_id '{}'", client_id);
        warn!(err);
        return Err(ErrorResponse::new(ErrorResponseType::Unauthorized, err));
    }
    if code.exp < OffsetDateTime::now_utc().unix_timestamp() {
        warn!("The Authorization Code has expired");
        return Err(ErrorResponse::new(
            ErrorResponseType::SessionExpired,
            String::from("The Authorization Code has expired"),
        ));
    }
    if code.challenge.is_some() {
        if req_data.code_verifier.is_none() {
            warn!("'code_verifier' is missing");
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("'code_verifier' is missing"),
            ));
        }

        if code.challenge_method.as_ref().unwrap().eq("plain") {
            if !code.challenge.eq(&req_data.code_verifier) {
                warn!("'code_verifier' does not match the challenge");
                return Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    String::from("'code_verifier' does not match the challenge"),
                ));
            }
        } else {
            let hash = digest::digest(&digest::SHA256, req_data.code_verifier.unwrap().as_bytes());
            let hash_base64 = base64_url_encode(hash.as_ref());

            if !code.challenge.as_ref().unwrap().eq(&hash_base64) {
                warn!("'code_verifier' does not match the challenge");
                return Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    String::from("'code_verifier' does not match the challenge"),
                ));
            }
        }
    }
    // We will not perform another `redirect_uri` check at this point, like stated in the RFC.
    // It is just unnecessary because of the way Rauthy handles the flow init during GET /authorize.
    //
    // It is impossible to trick a client to be redirected to another `redirect_uri` than the allowed ones,
    // which are all in control by the original client. The `redirect_uri` for Rauthy in the client config
    // is not optional like mentioned in the RFC, but actually mandatory. It is already checked and validated
    // carefully before the user would even see the login prompt.
    //
    // An additional check at this point does not provide any security benefit but only uses resources.

    let user = User::find(data, code.user_id.clone()).await?;
    let token_set = TokenSet::from_user(
        &user,
        data,
        &client,
        dpop_fingerprint,
        code.nonce.clone().map(TokenNonce),
        Some(TokenScopes(code.scopes.join(" "))),
        AuthCodeFlow::Yes,
        DeviceCodeFlow::No,
    )
    .await?;

    // update session metadata
    if code.session_id.is_some() {
        let sid = code.session_id.as_ref().unwrap().clone();
        let mut session = Session::find(data, sid).await?;

        session.last_seen = OffsetDateTime::now_utc().unix_timestamp();
        session.state = SessionState::Auth;
        if let Err(err) = session.validate_user_expiry(&user) {
            code.delete(data).await?;
            return Err(err);
        }
        session.validate_user_expiry(&user)?;
        session.user_id = Some(user.id);
        session.roles = Some(user.roles);
        session.groups = user.groups;
        session.save(data).await?;
    }
    code.delete(data).await?;

    // update timestamp if it is a dynamic client
    if client.is_dynamic() {
        ClientDyn::update_used(data, &client.id).await?;
    }

    Ok((token_set, headers))
}

/// Return a [TokenSet](crate::models::response::TokenSet) for the `client_credentials` flow
#[tracing::instrument(skip_all, fields(client_id = req_data.client_id, username = req_data.username))]
async fn grant_type_credentials(
    data: &web::Data<AppState>,
    req: HttpRequest,
    req_data: TokenRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    if req_data.client_secret.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("'client_secret' is missing"),
        ));
    }

    let (client_id, client_secret) = req_data.try_get_client_id_secret(&req)?;
    let client = Client::find(data, client_id).await?;
    if !client.confidential {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("'client_credentials' flow is allowed for confidential clients only"),
        ));
    }
    if !client.enabled {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("client is disabled"),
        ));
    }
    let secret = client_secret.ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("'client_secret' is missing"),
        )
    })?;
    client.validate_secret(&secret, &req)?;
    client.validate_flow("client_credentials")?;
    let header_origin = client.validate_origin(&req, &data.listen_scheme, &data.public_url)?;

    let mut headers = Vec::new();
    let dpop_fingerprint =
        if let Some(proof) = DPoPProof::opt_validated_from(data, &req, &header_origin).await? {
            if let Some(nonce) = &proof.claims.nonce {
                headers.push((
                    HeaderName::from_str(HEADER_DPOP_NONCE).unwrap(),
                    HeaderValue::from_str(nonce).unwrap(),
                ));
            }
            Some(DpopFingerprint(proof.jwk_fingerprint()?))
        } else {
            None
        };
    // We do not push the origin header, because client credentials should never be used from
    // any browser at all

    // update timestamp if it is a dynamic client
    if client.is_dynamic() {
        ClientDyn::update_used(data, &client.id).await?;
    }

    let ts = TokenSet::for_client_credentials(data, &client, dpop_fingerprint).await?;
    Ok((ts, headers))
}

/// Return a [TokenSet](crate::models::response::TokenSet) for the `device_code` flow
#[tracing::instrument(skip_all, fields(client_id = payload.client_id))]
pub async fn grant_type_device_code(
    data: &web::Data<AppState>,
    peer_ip: Option<String>,
    payload: TokenRequest,
) -> HttpResponse {
    let device_code = match &payload.device_code {
        None => {
            return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
                error: OAuth2ErrorTypeResponse::InvalidRequest,
                error_description: Some(Cow::from("`device_code` is missing")),
            });
        }
        Some(dc) => dc,
    };
    let mut code = match DeviceAuthCode::find_by_device_code(data, device_code).await {
        Ok(Some(code)) => code,
        Ok(None) | Err(_) => {
            return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
                error: OAuth2ErrorTypeResponse::ExpiredToken,
                error_description: Some(Cow::from("invalid `device_code` or request has expired")),
            });
        }
    };

    if Some(code.client_id.as_str()) != payload.client_id.as_deref() {
        return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
            error: OAuth2ErrorTypeResponse::InvalidRequest,
            error_description: Some(Cow::from("Invalid `client_id`")),
        });
    }

    // We need to check the device_code again, because the `find_by_device_code` uses
    // the `user_code` as cache index under the hood for smaller footprints and the
    // ability to find it in both ways without duplicated data.
    if &code.device_code != device_code {
        return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
            error: OAuth2ErrorTypeResponse::UnauthorizedClient,
            error_description: Some(Cow::from("Invalid `device_code`")),
        });
    }

    if code.client_secret != payload.client_secret {
        return HttpResponse::BadRequest().json(OAuth2ErrorResponse {
            error: OAuth2ErrorTypeResponse::UnauthorizedClient,
            error_description: Some(Cow::from("Invalid `client_secret`")),
        });
    }

    debug!("device auth code poll request is valid");
    let mut error = OAuth2ErrorTypeResponse::AuthorizationPending;
    let mut error_description = Cow::default();

    // Check last_poll and make sure interval is being respected.
    // We allow it to be 500ms shorter than specified to not get into
    // possible problems with slightly inaccurate client implementations.
    let now = Utc::now();
    let poll_thres = now
        .sub(chrono::Duration::seconds(
            *DEVICE_GRANT_POLL_INTERVAL as i64,
        ))
        .add(chrono::Duration::milliseconds(500));
    if poll_thres < code.last_poll {
        warn!("device does not respect the poll interval");
        code.warnings += 1;
        if code.warnings >= 3 {
            warn!("deleting device auth code request early because of not respected poll interval");
            error = OAuth2ErrorTypeResponse::AccessDenied;
            error_description = Cow::from("poll interval has not been respected");
            if let Err(err) = code.delete(data).await {
                // this should never happen
                error!("Error deleting DeviceAuthCode from the cache: {:}", err);
            }
        } else {
            error = OAuth2ErrorTypeResponse::SlowDown;
            error_description = Cow::from("must respect the poll interval");
        }
    }

    // check validation
    if let Some(verified_by) = &code.verified_by {
        let user = match User::find(data, verified_by.clone()).await {
            Ok(user) => user,
            Err(err) => {
                // at this point, this should never fail - only if the DB went down in the meantime
                error!("{:?}", err);
                return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::InvalidRequest,
                    error_description: Some(Cow::from(err.to_string())),
                });
            }
        };

        let client = match Client::find(data, code.client_id.clone()).await {
            Ok(client) => client,
            Err(err) => {
                // at this point, this should never fail - only if the DB went down in the meantime
                error!("{:?}", err);
                return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::InvalidRequest,
                    error_description: Some(Cow::from(err.to_string())),
                });
            }
        };

        let access_exp = now.add(chrono::Duration::seconds(
            client.access_token_lifetime as i64,
        ));
        let refresh_exp = if client.refresh_token {
            Some(
                access_exp
                    .add(chrono::Duration::seconds(48 * 3600))
                    .timestamp(),
            )
        } else {
            None
        };

        if let Err(err) = code.delete(data).await {
            // should really never happen - in cache only
            error!("Error deleting DeviceAuthCode: {:?}", err);
        }

        let id = new_store_id();
        let device = DeviceEntity {
            id: id.clone(),
            client_id: code.client_id,
            user_id: Some(user.id.clone()),
            created: now.timestamp(),
            access_exp: access_exp.timestamp(),
            refresh_exp,
            peer_ip: peer_ip.unwrap_or_default(),
            // The very first name will just always be the id.
            // This is a better UX than asking for a custom name each time.
            // TODO add an optional `name` param to the initial device request?
            name: id,
        };
        if let Err(err) = device.insert(data).await {
            error!("{:?}", err);
            return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                error: OAuth2ErrorTypeResponse::InvalidRequest,
                error_description: Some(Cow::from(err.to_string())),
            });
        }
        debug!("New Device has been created: {:?}", device);

        let ts = match TokenSet::from_user(
            &user,
            data,
            &client,
            None,
            None,
            code.scopes.map(TokenScopes),
            AuthCodeFlow::No,
            DeviceCodeFlow::Yes(device.id),
        )
        .await
        {
            Ok(ts) => ts,
            Err(err) => {
                error!("Building Device TokenSet: {:?}", err);
                return HttpResponse::InternalServerError().json(OAuth2ErrorResponse {
                    error: OAuth2ErrorTypeResponse::InvalidRequest,
                    error_description: Some(Cow::from(err.to_string())),
                });
            }
        };

        return HttpResponse::Ok().json(ts);
    }

    code.last_poll = now;
    if let Err(err) = code.save(data).await {
        // this should never happen
        error!("Error saving the DeviceAuthCode: {:?}", err);
    }

    HttpResponse::BadRequest().json(OAuth2ErrorResponse {
        error,
        error_description: Some(error_description),
    })
}

/// Return a [TokenSet](crate::models::response::TokenSet) for the `password` flow
#[tracing::instrument(skip_all, fields(client_id = req_data.client_id, username = req_data.username))]
async fn grant_type_password(
    data: &web::Data<AppState>,
    req: HttpRequest,
    req_data: TokenRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    if req_data.username.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("Missing 'username'"),
        ));
    }
    if req_data.password.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("Missing 'password"),
        ));
    }

    let (client_id, client_secret) = req_data.try_get_client_id_secret(&req)?;
    let email = req_data.username.as_ref().unwrap();
    let password = req_data.password.unwrap();

    let client = Client::find(data, client_id).await?;
    let header_origin = client.validate_origin(&req, &data.listen_scheme, &data.public_url)?;
    if client.confidential {
        let secret = client_secret.ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("Missing 'client_secret'"),
            )
        })?;
        client.validate_secret(&secret, &req)?;
    }
    client.validate_flow("password")?;

    let mut headers = Vec::new();
    let dpop_fingerprint =
        if let Some(proof) = DPoPProof::opt_validated_from(data, &req, &header_origin).await? {
            if let Some(nonce) = &proof.claims.nonce {
                headers.push((
                    HeaderName::from_str(HEADER_DPOP_NONCE).unwrap(),
                    HeaderValue::from_str(nonce).unwrap(),
                ));
            }
            Some(DpopFingerprint(proof.jwk_fingerprint()?))
        } else {
            None
        };
    if let Some(h) = header_origin {
        headers.push(h);
    }

    // This Error must be the same if user does not exist AND passwords do not match to prevent
    // username enumeration
    let mut user = User::find_by_email(data, String::from(email))
        .await
        .map_err(|_| {
            warn!(
                "False login from Host: '{}' with invalid username: '{}'",
                get_client_ip(&req),
                email
            );
            ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                String::from("Invalid user credentials"),
            )
        })?;
    user.check_enabled()?;
    user.check_expired()?;

    match user.validate_password(data, password.clone()).await {
        Ok(_) => {
            user.last_login = Some(OffsetDateTime::now_utc().unix_timestamp());
            user.last_failed_login = None;
            user.failed_login_attempts = None;

            // check if the password hash should be upgraded
            let hash_uptodate = user.is_argon2_uptodate(&data.argon2_params)?;
            if !hash_uptodate {
                info!("Updating Argon2ID params for user '{}'", &user.email);
                let new_hash = HashPassword::hash_password(password).await?;
                // let new_hash = User::new_password_hash(&password, params).await?;
                user.password = Some(new_hash);
            }

            user.save(data, None, None).await?;

            // update timestamp if it is a dynamic client
            if client.is_dynamic() {
                ClientDyn::update_used(data, &client.id).await?;
            }

            let ts = TokenSet::from_user(
                &user,
                data,
                &client,
                dpop_fingerprint,
                None,
                None,
                AuthCodeFlow::No,
                DeviceCodeFlow::No,
            )
            .await?;
            Ok((ts, headers))
        }
        Err(err) => {
            warn!(
                "False Login attempt from Host: '{}' for user: '{}'",
                get_client_ip(&req),
                user.email
            );

            user.last_failed_login = Some(OffsetDateTime::now_utc().unix_timestamp());
            user.failed_login_attempts = Some(&user.failed_login_attempts.unwrap_or(0) + 1);

            user.save(data, None, None).await?;

            // TODO add expo increasing sleeps after failed login attempts here?
            Err(err)
        }
    }
}

/// Return a [TokenSet](crate::models::response::TokenSet) for the `refresh_token` flow
#[tracing::instrument(skip_all, fields(client_id = req_data.client_id, username = req_data.username))]
async fn grant_type_refresh(
    data: &web::Data<AppState>,
    req: HttpRequest,
    req_data: TokenRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    if req_data.refresh_token.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("'refresh_token' is missing"),
        ));
    }
    let (client_id, client_secret) = req_data.try_get_client_id_secret(&req)?;
    let client = Client::find_maybe_ephemeral(data, client_id).await?;

    let header_origin = client.validate_origin(&req, &data.listen_scheme, &data.public_url)?;

    if client.confidential {
        let secret = client_secret.ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("'client_secret' is missing"),
            )
        })?;
        client.validate_secret(&secret, &req)?;
    }

    client.validate_flow("refresh_token")?;

    let refresh_token = req_data.refresh_token.unwrap();

    // validate common refresh token claims first and get the payload
    let (ts, dpop_none) = validate_refresh_token(Some(client), &refresh_token, data, &req).await?;

    let mut headers = Vec::new();
    if let Some(h) = header_origin {
        headers.push(h);
    }
    if let Some(nonce) = dpop_none {
        headers.push((
            HeaderName::from_str(HEADER_DPOP_NONCE).unwrap(),
            HeaderValue::from_str(&nonce).unwrap(),
        ));
    }

    Ok((ts, headers))
}

/**
Handles the login delay.

With every successful login, a new average login time is calculated for how
long it took for a successful login. If a login failed though, the answer will be delayed by the
current average for a successful login, to prevent things like username enumeration.
 */
pub async fn handle_login_delay(
    data: &web::Data<AppState>,
    peer_ip: Option<String>,
    start: Duration,
    cache_config: &redhac::CacheConfig,
    // the bool for Ok() is true is the password has been hashed
    // the bool for Err() means if we need to add a login delay (and none otherwise for better UX)
    res: Result<(HttpResponse, bool), (ErrorResponse, bool)>,
) -> Result<HttpResponse, ErrorResponse> {
    let success_time = cache_get!(
        i64,
        CACHE_NAME_LOGIN_DELAY.to_string(),
        IDX_LOGIN_TIME.to_string(),
        cache_config,
        false
    )
    .await?
    .unwrap_or(2000);

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let delta = end - start;

    match res {
        Ok((resp, has_password_been_hashed)) => {
            // cleanup possibly blacklisted IP
            if let Some(ip) = peer_ip {
                data.tx_ip_blacklist
                    .send_async(IpBlacklistReq::LoginFailedDelete(ip))
                    .await
                    .expect("ip blacklist recv not to be closed");
            } else {
                warn!("No IP in login delay handler - check your reverse proxy setup");
            }

            // only calculate the new median login time base on the full duration incl password hash
            if has_password_been_hashed {
                let new_time = (success_time + delta.as_millis() as i64) / 2;

                cache_put(
                    CACHE_NAME_LOGIN_DELAY.to_string(),
                    IDX_LOGIN_TIME.to_string(),
                    cache_config,
                    &new_time,
                )
                .await?;

                debug!("New login_success_time: {}", new_time);
            }

            Ok(resp)
        }
        Err((err, add_login_delay)) => {
            if !add_login_delay {
                return Err(err);
            }

            let mut failed_logins = 1;

            // check possibly blacklisted IP
            if let Some(ip) = peer_ip.clone() {
                let (tx, rx) = oneshot::channel();
                data.tx_ip_blacklist
                    .send_async(IpBlacklistReq::LoginCheck(IpFailedLoginCheck {
                        ip,
                        increase_counter: true,
                        tx,
                    }))
                    .await
                    .expect("ip blacklist recv not to be closed");

                match rx.await {
                    Ok(res) => {
                        if let Some(counter) = res {
                            failed_logins = counter;
                        }
                    }
                    Err(err) => {
                        error!("oneshot recv error in login delay handler - this should never happen: {:?}", err);
                    }
                }
            } else {
                warn!("No IP in login delay handler - check your reverse proxy setup");
            }

            // event for failed login
            let ip = if let Some(ip) = peer_ip {
                ip
            } else {
                error!("No peer IP for connected client - check your reverse proxy setup!");
                "UNKNOWN".to_string()
            };
            data.tx_events
                .send_async(Event::invalid_login(failed_logins, ip.clone()))
                .await
                .unwrap();

            let sleep_time_median = {
                let time_taken = end.sub(start).as_millis() as u64;
                let mut sleep_time_median = 0;
                let su64 = success_time as u64;
                if time_taken < su64 {
                    sleep_time_median = su64 - time_taken;
                }
                sleep_time_median
            };

            let sleep_time = match failed_logins as u64 {
                // n-th blacklist -> blocks for 24h with each invalid request
                t if t >= 25 => {
                    let not_before = Utc::now().add(chrono::Duration::seconds(86400));
                    let ts = not_before.timestamp();
                    let html = TooManyRequestsHtml::build(&ip, ts);

                    data.tx_events
                        .send_async(Event::ip_blacklisted(not_before, ip.clone()))
                        .await
                        .unwrap();

                    return Err(ErrorResponse::new(
                        ErrorResponseType::TooManyRequests(ts),
                        html,
                    ));
                }

                t if t > 20 => sleep_time_median + t * 20_000,

                // 4th blacklist
                20 => {
                    let not_before = Utc::now().add(chrono::Duration::seconds(3600));
                    let ts = not_before.timestamp();
                    let html = TooManyRequestsHtml::build(&ip, ts);

                    data.tx_events
                        .send_async(Event::ip_blacklisted(not_before, ip.clone()))
                        .await
                        .unwrap();

                    return Err(ErrorResponse::new(
                        ErrorResponseType::TooManyRequests(ts),
                        html,
                    ));
                }

                t if t > 15 => sleep_time_median + t * 15_000,

                // 3rd blacklist
                15 => {
                    let not_before = Utc::now().add(chrono::Duration::seconds(900));
                    let ts = not_before.timestamp();
                    let html = TooManyRequestsHtml::build(&ip, ts);

                    data.tx_events
                        .send_async(Event::ip_blacklisted(not_before, ip.clone()))
                        .await
                        .unwrap();

                    return Err(ErrorResponse::new(
                        ErrorResponseType::TooManyRequests(ts),
                        html,
                    ));
                }

                t if t > 10 => sleep_time_median + t * 10_000,

                // 2nd blacklist
                10 => {
                    let not_before = Utc::now().add(chrono::Duration::seconds(600));
                    let ts = not_before.timestamp();
                    let html = TooManyRequestsHtml::build(&ip, ts);

                    data.tx_events
                        .send_async(Event::ip_blacklisted(not_before, ip.clone()))
                        .await
                        .unwrap();

                    return Err(ErrorResponse::new(
                        ErrorResponseType::TooManyRequests(ts),
                        html,
                    ));
                }

                t if t > 7 => sleep_time_median + t * 5_000,

                // 1st blacklist
                7 => {
                    let not_before = Utc::now().add(chrono::Duration::seconds(60));
                    let ts = not_before.timestamp();
                    let html = TooManyRequestsHtml::build(&ip, ts);

                    data.tx_events
                        .send_async(Event::ip_blacklisted(not_before, ip.clone()))
                        .await
                        .unwrap();

                    return Err(ErrorResponse::new(
                        ErrorResponseType::TooManyRequests(ts),
                        html,
                    ));
                }

                t if t >= 5 => sleep_time_median + t * 3_000,

                t if t >= 3 => sleep_time_median + t * 2_000,

                _ => sleep_time_median,
            };

            debug!("Failed login - sleeping for {}ms now", sleep_time);
            tokio::time::sleep(Duration::from_millis(sleep_time)).await;

            Err(err)
        }
    }
}

/// Returns the Logout HTML Page for [GET /oidc/logout](crate::handlers::get_logout)
pub async fn logout(
    logout_request: LogoutRequest,
    session: &Session,
    data: &web::Data<AppState>,
    lang: &Language,
) -> Result<String, ErrorResponse> {
    let colors = ColorEntity::find_rauthy(data).await?;

    if logout_request.id_token_hint.is_none() {
        return Ok(LogoutHtml::build(&session.csrf_token, false, &colors, lang));
    }

    // check if the provided token hint is a valid
    let token_raw = logout_request.id_token_hint.unwrap();
    let claims = validate_token::<JwtIdClaims>(data, &token_raw).await?;

    // check if it is an ID token
    if JwtTokenType::Id != claims.custom.typ {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("The provided token is not an ID token"),
        ));
    }

    // from here on, the token_hint contains a valid ID token -> skip the logout confirmation
    if logout_request.post_logout_redirect_uri.is_some() {
        // unwrap is safe since the token is valid already
        let client_id = claims.custom.azp;
        let client = Client::find(data, client_id).await?;
        if client.post_logout_redirect_uris.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("Given 'post_logout_redirect_uri' is not allowed"),
            ));
        }

        let target = logout_request.post_logout_redirect_uri.unwrap();
        let uri_vec = client.get_post_logout_uris();
        let valid_redirect = uri_vec.as_ref().unwrap().iter().filter(|uri| {
            if uri.ends_with('*') && target.starts_with(uri.split_once('*').unwrap().0) {
                return true;
            }
            if target.eq(*uri) {
                return true;
            }
            false
        });
        if valid_redirect.count() == 0 {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("Given 'post_logout_redirect_uri' is not allowed"),
            ));
        }
        // redirect uri is valid at this point
    }

    Ok(LogoutHtml::build(&session.csrf_token, true, &colors, lang))
}

// TODO move into entity
/// Rotates and generates a whole new Set of JWKs for signing JWT Tokens
pub async fn rotate_jwks(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
    info!("Starting JWKS rotation");

    // let key = data.enc_keys.get(&data.enc_key_active).unwrap();
    let enc_key_active = &EncKeys::get_static().enc_key_active;

    // RSA256
    let jwk_plain = web::block(|| {
        RS256KeyPair::generate(2048)
            .unwrap()
            .with_key_id(&get_rand(24))
    })
    .await?;
    let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
        .into_bytes()
        .to_vec();
    let entity = Jwk {
        kid: jwk_plain.key_id().as_ref().unwrap().clone(),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
        signature: JwkKeyPairAlg::RS256,
        enc_key_id: enc_key_active.to_string(),
        jwk,
    };
    entity.save(&data.db).await?;

    // RS384
    let jwk_plain = web::block(|| {
        RS384KeyPair::generate(3072)
            .unwrap()
            .with_key_id(&get_rand(24))
    })
    .await?;
    let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
        .into_bytes()
        .to_vec();
    let entity = Jwk {
        kid: jwk_plain.key_id().as_ref().unwrap().clone(),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
        signature: JwkKeyPairAlg::RS384,
        enc_key_id: enc_key_active.to_string(),
        jwk,
    };
    entity.save(&data.db).await?;

    // RSA512
    let jwk_plain = web::block(|| {
        RS512KeyPair::generate(4096)
            .unwrap()
            .with_key_id(&get_rand(24))
    })
    .await?;
    let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
        .into_bytes()
        .to_vec();
    let entity = Jwk {
        kid: jwk_plain.key_id().as_ref().unwrap().clone(),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
        signature: JwkKeyPairAlg::RS512,
        enc_key_id: enc_key_active.to_string(),
        jwk,
    };
    entity.save(&data.db).await?;

    // Ed25519
    let jwk_plain = web::block(|| Ed25519KeyPair::generate().with_key_id(&get_rand(24))).await?;
    let jwk = EncValue::encrypt(jwk_plain.to_der().as_slice())?
        .into_bytes()
        .to_vec();
    let entity = Jwk {
        kid: jwk_plain.key_id().as_ref().unwrap().clone(),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
        signature: JwkKeyPairAlg::EdDSA,
        enc_key_id: enc_key_active.to_string(),
        jwk,
    };
    entity.save(&data.db).await?;

    // clear all latest_jwk from cache
    cache_del(
        CACHE_NAME_12HR.to_string(),
        format!("{}{}", IDX_JWK_LATEST, JwkKeyPairAlg::RS256.as_str()),
        &data.caches.ha_cache_config,
    )
    .await?;
    cache_del(
        CACHE_NAME_12HR.to_string(),
        format!("{}{}", IDX_JWK_LATEST, JwkKeyPairAlg::RS384.as_str()),
        &data.caches.ha_cache_config,
    )
    .await?;
    cache_del(
        CACHE_NAME_12HR.to_string(),
        format!("{}{}", IDX_JWK_LATEST, JwkKeyPairAlg::RS512.as_str()),
        &data.caches.ha_cache_config,
    )
    .await?;
    cache_del(
        CACHE_NAME_12HR.to_string(),
        format!("{}{}", IDX_JWK_LATEST, JwkKeyPairAlg::EdDSA.as_str()),
        &data.caches.ha_cache_config,
    )
    .await?;

    // clear the all_certs / JWKS cache
    cache_del(
        CACHE_NAME_12HR.to_string(),
        IDX_JWKS.to_string(),
        &data.caches.ha_cache_config,
    )
    .await?;

    info!("Finished JWKS rotation");

    data.tx_events
        .send_async(Event::jwks_rotated())
        .await
        .unwrap();

    Ok(())
}

/// Signs an access token
async fn sign_access_token(
    data: &web::Data<AppState>,
    claims: claims::JWTClaims<JwtAccessClaims>,
    client: &Client,
) -> Result<String, ErrorResponse> {
    let key_pair_type = JwkKeyPairAlg::from_str(&client.access_token_alg)?;
    let kp = JwkKeyPair::find_latest(data, &client.access_token_alg, key_pair_type).await?;
    sign_jwt!(kp, claims)
}

/// Signs an id token
async fn sign_id_token(
    data: &web::Data<AppState>,
    claims: claims::JWTClaims<JwtIdClaims>,
    client: &Client,
) -> Result<String, ErrorResponse> {
    let key_pair_type = JwkKeyPairAlg::from_str(&client.id_token_alg)?;
    let kp = JwkKeyPair::find_latest(data, &client.id_token_alg, key_pair_type).await?;
    sign_jwt!(kp, claims)
}

/// Signs a refresh token
async fn sign_refresh_token(
    data: &web::Data<AppState>,
    claims: claims::JWTClaims<JwtRefreshClaims>,
) -> Result<String, ErrorResponse> {
    let alg = String::from("EdDSA");
    let key_pair_type = JwkKeyPairAlg::from_str(&alg)?;
    let kp = JwkKeyPair::find_latest(data, &alg, key_pair_type).await?;
    sign_jwt!(kp, claims)
}

/// Validates request parameters for the authorization and refresh endpoints
pub async fn validate_auth_req_param(
    data: &web::Data<AppState>,
    req: &HttpRequest,
    client_id: &str,
    redirect_uri: &str,
    code_challenge: &Option<String>,
    code_challenge_method: &Option<String>,
) -> Result<(Client, Option<(HeaderName, HeaderValue)>), ErrorResponse> {
    // client exists
    let client = Client::find_maybe_ephemeral(data, String::from(client_id)).await?;

    // allowed origin
    let header = client.validate_origin(req, &data.listen_scheme, &data.public_url)?;

    // allowed redirect uris
    client.validate_redirect_uri(redirect_uri)?;

    // code challenge + method
    if client.challenge.is_some() {
        if code_challenge.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                String::from("'code_challenge' is missing"),
            ));
        } else {
            // 'plain' is the default method to be assumed by the OAuth specification when it is
            // not further specified.
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

// TODO remove handler /refresh and move into grant_type_refresh? -> obsolete since grant_type_refresh?
/// Validates common claims for refresh tokens used in different places
pub async fn validate_refresh_token(
    // when this is some, it will be checked against the 'azp' claim, otherwise skipped and a client
    // will be fetched inside this function
    client_opt: Option<Client>,
    refresh_token: &str,
    data: &web::Data<AppState>,
    req: &HttpRequest,
) -> Result<(TokenSet, Option<String>), ErrorResponse> {
    let options = VerificationOptions {
        // allowed_audiences: Some(HashSet::from_strings(&[&])), // TODO change after making client non-opt
        allowed_issuers: Some(HashSet::from_strings(&[&data.issuer])),
        ..Default::default()
    };

    // extract metadata
    let kid = JwkKeyPair::kid_from_token(refresh_token)?;

    // retrieve jwk for kid
    let kp = JwkKeyPair::find(data, kid).await?;
    let claims: claims::JWTClaims<JwtRefreshClaims> =
        validate_jwt!(JwtRefreshClaims, kp, refresh_token, options)?;

    // validate typ
    if claims.custom.typ != JwtTokenType::Refresh {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("Provided Token is not a valid refresh token"),
        ));
    }

    // get uid
    let uid = claims.custom.uid;

    // get azp / client
    let client = if let Some(c) = client_opt {
        c
    } else {
        Client::find(data, claims.custom.azp.clone()).await?
    };
    if client.id != claims.custom.azp {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            String::from("Invalid 'azp'"),
        ));
    }
    let header_origin = client.validate_origin(req, &data.listen_scheme, &data.public_url)?;

    // validate DPoP proof
    let (dpop_fingerprint, dpop_nonce) = if let Some(cnf) = claims.custom.cnf {
        // if the refresh token contains the 'cnf' header, we must validate the DPoP as well
        if let Some(proof) = DPoPProof::opt_validated_from(data, req, &header_origin).await? {
            let fingerprint = proof.jwk_fingerprint()?;
            if fingerprint != cnf.jkt {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "The refresh token is bound to a missing DPoP proof".to_string(),
                ));
            }
            debug!("DPoP-Bound refresh token accepted");
            (Some(DpopFingerprint(fingerprint)), proof.claims.nonce)
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "The refresh token is bound to a missing DPoP proof".to_string(),
            ));
        }
    } else {
        (None, None)
    };

    let mut user = User::find(data, uid).await?;
    user.check_enabled()?;
    user.check_expired()?;

    // validate that it exists in the db and invalidate it afterward
    let (_, validation_str) = refresh_token.split_at(refresh_token.len() - 49);
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let exp_at_secs = now + data.refresh_grace_time as i64;
    let rt_scope = if let Some(device_id) = &claims.custom.did {
        let mut rt = RefreshTokenDevice::find(data, validation_str).await?;

        if &rt.device_id != device_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "'device_id' does not match".to_string(),
            ));
        }
        if rt.user_id != user.id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "'user_id' does not match".to_string(),
            ));
        }

        if rt.exp > exp_at_secs + 1 {
            rt.exp = exp_at_secs;
            rt.save(data).await?;
        }
        rt.scope
    } else {
        let mut rt = RefreshToken::find(data, validation_str).await?;
        if rt.exp > exp_at_secs + 1 {
            rt.exp = exp_at_secs;
            rt.save(data).await?;
        }
        rt.scope
    };

    // at this point, everything has been validated -> we can issue a new TokenSet safely
    debug!("Refresh Token - all good!");

    // set last login
    user.last_login = Some(OffsetDateTime::now_utc().unix_timestamp());
    user.save(data, None, None).await?;

    let ts = if let Some(s) = rt_scope {
        TokenSet::from_user(
            &user,
            data,
            &client,
            dpop_fingerprint,
            None,
            Some(TokenScopes(s)),
            AuthCodeFlow::No,
            DeviceCodeFlow::No,
        )
        .await
    } else {
        TokenSet::from_user(
            &user,
            data,
            &client,
            dpop_fingerprint,
            None,
            None,
            AuthCodeFlow::No,
            DeviceCodeFlow::No,
        )
        .await
    }?;
    Ok((ts, dpop_nonce))
}

/// Validates a given JWT Access Token
pub async fn validate_token<T: serde::Serialize + for<'de> ::serde::Deserialize<'de>>(
    data: &web::Data<AppState>,
    token: &str,
) -> Result<claims::JWTClaims<T>, ErrorResponse> {
    let options = jwt_simple::prelude::VerificationOptions {
        // allowed_audiences: Some(HashSet::from_strings(&[&])), // TODO
        allowed_issuers: Some(HashSet::from_strings(&[&data.issuer])),
        ..Default::default()
    };

    // extract metadata
    let kid = JwkKeyPair::kid_from_token(token)?;

    // retrieve jwk for kid
    let kp = JwkKeyPair::find(data, kid).await?;
    validate_jwt!(T, kp, token, options)

    // TODO check roles if we add more users / roles
}

#[cfg(test)]
mod tests {}
