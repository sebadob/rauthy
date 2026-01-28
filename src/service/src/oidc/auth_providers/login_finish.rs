use crate::oidc;
use crate::oidc::auth_providers::cust_impls;
use crate::oidc::authorize::AuthorizeData;
use actix_web::HttpRequest;
use actix_web::cookie::Cookie;
use atrium_common::store::Store;
use rauthy_api_types::auth_providers::ProviderCallbackRequest;
use rauthy_common::constants::{
    APPLICATION_JSON, COOKIE_UPSTREAM_CALLBACK, PROVIDER_ATPROTO, PROVIDER_LINK_COOKIE,
};
use rauthy_common::utils::base64_url_encode;
use rauthy_common::{http_client, sha256};
use rauthy_data::AuthStep;
use rauthy_data::api_cookie::ApiCookie;
use rauthy_data::database::DB;
use rauthy_data::entity::atproto;
use rauthy_data::entity::auth_providers::{
    AuthProvider, AuthProviderCallback, AuthProviderIdClaims, AuthProviderLinkCookie,
    AuthProviderType, NewFederatedUserCreated, ProviderMfaLogin,
};
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::sessions::Session;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use tracing::{debug, error};

#[derive(Debug, Deserialize)]
struct AuthProviderTokenSet {
    access_token: Option<String>,
    // token_type: Option<String>,
    id_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Serialize)]
struct OidcCodeRequestParams<'a> {
    client_id: &'a str,
    client_secret: Option<String>,
    code: &'a str,
    code_verifier: Option<&'a str>,
    grant_type: &'static str,
    redirect_uri: &'a str,
}

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
        let atproto = atproto::Client::get();

        let params = atrium_oauth::CallbackParams {
            code: payload.code.clone(),
            state: Some(payload.state.clone()),
            iss: payload.iss_atproto.clone(),
        };
        // return early if we got any error
        let (session_manager, app_state) = atproto.callback(params).await.map_err(|error| {
            error!(%error, "failed to complete authorization callback for ATProto");

            ErrorResponse::new(
                ErrorResponseType::Internal,
                "failed to complete authorization callback for ATProto",
            )
        })?;

        let Some(app_state) = app_state else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "missing callback state for ATProto",
            ));
        };

        if app_state != slf.callback_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "callback state mismatch for ATProto",
            ));
        }

        let agent = atrium_api::agent::Agent::new(session_manager);

        let Some(did) = agent.did().await else {
            panic!("missing DID for ATProto session");
        };

        let Some(session) = DB.get(&did).await.map_err(|error| {
            error!(%error, "failed to get session for ATProto callback");

            ErrorResponse::new(
                ErrorResponseType::Internal,
                "failed to get session for ATProto callback",
            )
        })?
        else {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "failed to complete authorization callback for atproto",
            ));
        };

        let data = match agent.api.com.atproto.server.get_session().await {
            Ok(atrium_api::types::Object { data, .. }) => data,
            Err(error) => {
                error!(%error, "failed to get session for ATProto callback");

                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    "failed to get session for ATProto callback",
                ));
            }
        };

        let claims = AuthProviderIdClaims {
            sub: Some(Value::String(session.token_set.sub.to_string())),
            email: data.email.map(Cow::from),
            email_verified: data.email_confirmed,
            ..Default::default()
        };

        claims.validate_update_user(&provider, &link_cookie).await?
    } else {
        let mut payload = OidcCodeRequestParams {
            // a client MAY add the `client_id`, but it MUST add it when it's public
            client_id: &provider.client_id,
            client_secret: None,
            code: &payload.code,
            code_verifier: provider.use_pkce.then_some(&payload.pkce_verifier),
            grant_type: "authorization_code",
            redirect_uri: &RauthyConfig::get().provider_callback_uri,
        };
        if provider.client_secret_post {
            payload.client_secret = AuthProvider::secret_cleartext(&provider.secret)?;
        }

        let res = {
            let mut builder = http_client()
                .post(&provider.token_endpoint)
                .header(ACCEPT, APPLICATION_JSON);

            if provider.client_secret_basic {
                builder = builder.basic_auth(
                    &provider.client_id,
                    AuthProvider::secret_cleartext(&provider.secret)?,
                )
            }

            builder
        }
        .form(&payload)
        .send()
        .await?;

        let status = res.status().as_u16();
        debug!("POST /token auth provider status: {status}");

        // return early if we got any error
        if !res.status().is_success() {
            let err = match res.text().await {
                Ok(body) => format!(
                    "HTTP {status} during POST {} for upstream auth provider '{}'\n{body}",
                    provider.token_endpoint, provider.client_id
                ),
                Err(_) => format!(
                    "HTTP {status} during POST {} for upstream auth provider '{}' without any body",
                    provider.token_endpoint, provider.client_id
                ),
            };
            error!("{}", err);
            return Err(ErrorResponse::new(ErrorResponseType::Internal, err));
        }

        let ts = match res.json::<AuthProviderTokenSet>().await {
            Ok(ts) => ts,
            Err(err) => {
                let err = format!(
                    "Deserializing /token response from auth provider {}: {err}",
                    provider.client_id
                );
                error!("{err}");
                return Err(ErrorResponse::new(ErrorResponseType::Internal, err));
            }
        };

        if let Some(err) = ts.error {
            let msg = format!(
                "/token request error: {err}: {}",
                ts.error_description.unwrap_or_default()
            );
            error!("{msg}");
            return Err(ErrorResponse::new(ErrorResponseType::Internal, msg));
        }

        // in case of a standard OIDC provider, we only care about the ID token
        if let Some(id_token) = ts.id_token {
            let claims_bytes = AuthProviderIdClaims::self_as_bytes_from_token(&id_token)?;
            let claims = AuthProviderIdClaims::try_from(claims_bytes.as_slice())?;

            claims.validate_update_user(&provider, &link_cookie).await?
        } else if let Some(access_token) = ts.access_token {
            // the id_token only exists, if we actually have an OIDC provider.
            // If we only get an access token, we need to do another request to the
            // userinfo endpoint
            let res = http_client()
                .get(&provider.userinfo_endpoint)
                .header(AUTHORIZATION, format!("Bearer {access_token}"))
                .header(ACCEPT, APPLICATION_JSON)
                .send()
                .await?;

            let status = res.status().as_u16();
            debug!("GET /userinfo auth provider status: {status}");

            let res_bytes = res.bytes().await?;
            let mut claims = AuthProviderIdClaims::try_from(res_bytes.as_ref())?;

            if claims.email.is_none() && provider.typ == AuthProviderType::GitHub {
                cust_impls::get_github_private_email(&access_token, &mut claims).await?;
            }

            claims.validate_update_user(&provider, &link_cookie).await?
        } else {
            let err = "Neither `access_token` nor `id_token` existed";
            error!("{err}");
            return Err(ErrorResponse::new(ErrorResponseType::BadRequest, err));
        }
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
        &session,
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
