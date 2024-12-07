use std::{fmt::Write, sync::Arc};

use actix_web::{
    cookie::Cookie,
    http::header::{self, HeaderValue},
    web, HttpRequest,
};
use atrium_api::types::string::Datetime;
use atrium_identity::{
    did::{CommonDidResolver, CommonDidResolverConfig, DEFAULT_PLC_DIRECTORY_URL},
    handle::{AtprotoHandleResolver, AtprotoHandleResolverConfig},
};
use atrium_oauth_client::{
    AtprotoClientMetadata, AuthMethod, AuthorizeOptions, CallbackParams, DefaultHttpClient,
    GrantType, KnownScope, OAuthClient, OAuthClientConfig, OAuthResolverConfig, Scope,
};
use cryptr::utils::secure_random_alnum;
use jose_jwk::{Jwk, JwkSet};
use rauthy_api_types::{atproto, oidc::JWKSCerts};
use rauthy_common::constants::{
    COOKIE_UPSTREAM_CALLBACK, DEV_MODE, PROVIDER_LINK_COOKIE, UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use resolvers::DnsTxtResolver;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::{
    api_cookie::ApiCookie,
    app_state::AppState,
    database::DB,
    entity::{
        auth_codes::AuthCode,
        auth_providers::{AuthProvider, AuthProviderIdClaims, AuthProviderLinkCookie},
    },
    AuthStep, AuthStepLoggedIn,
};

use super::{
    auth_providers::{AuthProviderCallback, AuthProviderType},
    jwk::JWKS,
    sessions::Session,
};

pub type Atproto = atrium_oauth_client::OAuthClient<
    DB,
    DB,
    CommonDidResolver<DefaultHttpClient>,
    AtprotoHandleResolver<DnsTxtResolver, DefaultHttpClient>,
>;

pub async fn init_oauth_client(
    public_url: String,
    keys: Option<Vec<Jwk>>,
) -> Result<Atproto, ErrorResponse> {
    let http_client = Arc::new(DefaultHttpClient::default());

    let scheme = (!*DEV_MODE).then(|| "https").unwrap_or("http");

    let client_metadata = AtprotoClientMetadata {
        client_id: format!("{scheme}://{public_url}/auth/v1/atproto/client_metadata"),
        client_uri: format!("{scheme}://{public_url}"),
        redirect_uris: vec![format!("{scheme}://{public_url}/auth/v1/atproto/callback")],
        token_endpoint_auth_method: (!*DEV_MODE)
            .then(|| AuthMethod::PrivateKeyJwt)
            .unwrap_or(AuthMethod::None),
        grant_types: vec![GrantType::AuthorizationCode, GrantType::RefreshToken],
        scopes: vec![Scope::Known(KnownScope::Atproto)],
        jwks_uri: None,
        token_endpoint_auth_signing_alg: (!*DEV_MODE).then(|| "S256".to_owned()),
    };

    let config = OAuthClientConfig {
        client_metadata: client_metadata.clone(),
        keys,
        resolver: OAuthResolverConfig {
            did_resolver: CommonDidResolver::new(CommonDidResolverConfig {
                plc_directory_url: DEFAULT_PLC_DIRECTORY_URL.to_string(),
                http_client: http_client.clone(),
            }),
            handle_resolver: AtprotoHandleResolver::new(AtprotoHandleResolverConfig {
                dns_txt_resolver: DnsTxtResolver::default(),
                http_client: http_client.clone(),
            }),
            authorization_server_metadata: Default::default(),
            protected_resource_metadata: Default::default(),
        },
        state_store: DB,
        session_store: DB,
    };

    let client = OAuthClient::new(config).map_err(|error| {
        error!(%error, "failed to initialize oauth client for atproto");

        ErrorResponse::new(
            ErrorResponseType::Internal,
            "failed to initialize oauth client for atproto",
        )
    })?;

    Ok(client)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Callback {}

impl Callback {
    /// returns (encrypted cookie, xsrf token, location header, optional allowed origins)
    pub async fn login_start(
        data: &web::Data<AppState>,
        payload: atproto::LoginRequest,
    ) -> Result<(Cookie<'_>, String, HeaderValue), ErrorResponse> {
        let callback = AuthProviderCallback {
            callback_id: secure_random_alnum(32),
            xsrf_token: secure_random_alnum(32),
            typ: AuthProviderType::Custom,

            req_client_id: String::from("atproto"),
            req_scopes: None,
            req_redirect_uri: payload.redirect_uri.clone(),
            req_state: payload.state.clone(),
            req_nonce: None,
            req_code_challenge: None,
            req_code_challenge_method: None,

            provider_id: String::from("atproto"),

            pkce_challenge: payload.pkce_challenge,
        };

        callback.save().await?;

        if ((!*DEV_MODE) && (data.atproto.read().await).jwks().keys.is_empty()) {
            let jwks = JWKS::find_pk().await?;

            if jwks.keys.is_empty() {
                JWKS::rotate(data).await?;
            }
            let jwks = JWKS::find_pk().await?;

            let Ok(JwkSet { keys: new_keys }) = serde_json::to_string(&JWKSCerts::from(jwks))
                .and_then(|s| serde_json::from_str(&s))
            else {
                panic!("invalid JwkSet");
            };

            *data.atproto.write().await =
                init_oauth_client(data.public_url.clone(), Some(new_keys)).await?;
        }

        let atproto = data.atproto.read().await;

        let options = AuthorizeOptions {
            state: Some(callback.callback_id.clone()),
            ..Default::default()
        };
        let location = atproto
            .authorize(&payload.at_identifier, options)
            .await
            .map_err(|error| {
                error!(%error, "failed to build pushed authorization request for atproto");
                ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "failed to build pushed authorization request for atproto",
                )
            })?;

        let cookie = ApiCookie::build(
            COOKIE_UPSTREAM_CALLBACK,
            &callback.callback_id,
            UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS as i64,
        );

        Ok((
            cookie,
            callback.xsrf_token,
            HeaderValue::from_str(&location).expect("Location HeaderValue to be correct"),
        ))
    }

    /// In case of any error, the callback code will be fully deleted for security reasons.
    pub async fn login_finish<'a>(
        data: &'a web::Data<AppState>,
        req: &'a HttpRequest,
        payload: &'a atproto::CallbackRequest,
        session: Session,
    ) -> Result<(AuthStep, Cookie<'a>), ErrorResponse> {
        // the callback id for the cache should be inside the encrypted cookie
        let callback_id = ApiCookie::from_req(req, COOKIE_UPSTREAM_CALLBACK).ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Missing encrypted callback cookie",
            )
        })?;

        // validate state
        if callback_id != payload.state {
            AuthProviderCallback::delete(callback_id).await?;

            error!("`state` does not match");
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`state` does not match",
            ));
        }
        debug!("callback state is valid");

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
        debug!("callback csrf token is valid");

        let atproto = data.atproto.read().await;

        // request is valid -> fetch token for the user
        let params = CallbackParams {
            code: payload.code.clone(),
            state: Some(payload.state.clone()),
            iss: payload.iss.clone(),
        };
        // return early if we got any error
        let (oauth_session, _) = atproto.callback(params).await.map_err(|error| {
            error!(%error, "failed to complete authorization callback for atproto");
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "failed to complete authorization callback for atproto",
            )
        })?;

        let token_set = oauth_session.token_set().await.map_err(|error| {
            error!(%error, "failed to retrieve token set for atproto");
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "failed to retrieve token set for atproto",
            )
        })?;

        // extract a possibly existing provider link cookie for
        // linking an existing account to a provider
        let link_cookie = ApiCookie::from_req(req, PROVIDER_LINK_COOKIE)
            .and_then(|value| AuthProviderLinkCookie::try_from(value.as_str()).ok());

        let provider = AuthProvider::find(&slf.provider_id).await?;

        let claims = AuthProviderIdClaims::new(token_set.sub.as_ref());
        let (user, _) = claims.validate_update_user(&provider, &link_cookie).await?;

        user.check_enabled()?;
        user.check_expired()?;

        if link_cookie.is_some() {
            // If this is the case, we don't need to validate any further client values.
            // We will not generate a new auth code at all -> this is just a request to federate
            // an existing account. The federation has been done in the step above already.
            return Ok((
                AuthStep::ProviderLink,
                AuthProviderLinkCookie::deletion_cookie(),
            ));
        }

        // ######################################
        // all good, we can generate an auth code

        // authorization code
        let code_lifetime = token_set
            .expires_at
            .as_ref()
            .map(|d| {
                d.as_ref()
                    .signed_duration_since(Datetime::now().as_ref())
                    .num_seconds() as i32
            })
            .ok_or_else(|| {
                error!("token set never expires!");
                ErrorResponse::new(ErrorResponseType::Internal, "token set never expires!")
            })?;
        let scopes = slf.req_scopes.iter().flatten().cloned().collect();
        let code = AuthCode::new(
            user.id.clone(),
            slf.provider_id.clone(),
            Some(session.id.clone()),
            slf.req_code_challenge,
            slf.req_code_challenge_method,
            slf.req_nonce,
            scopes,
            code_lifetime,
        );
        code.save().await?;

        // location header
        let mut loc = format!("{}?code={}", &slf.req_redirect_uri, code.id);
        if let Some(state) = slf.req_state {
            write!(loc, "&state={}", state)?;
        };

        let auth_step = AuthStep::LoggedIn(AuthStepLoggedIn {
            user_id: user.id,
            email: user.email,
            header_loc: (header::LOCATION, HeaderValue::from_str(&loc)?),
            header_csrf: Session::get_csrf_header(&session.csrf_token),
            header_origin: None,
        });

        // callback data deletion cookie
        let cookie = ApiCookie::build(COOKIE_UPSTREAM_CALLBACK, "", 0);
        Ok((auth_step, cookie))
    }
}

pub mod resolvers {
    use std::error::Error;

    use hickory_resolver::{proto::rr::rdata::TXT, TokioAsyncResolver};

    pub struct DnsTxtResolver {
        resolver: TokioAsyncResolver,
    }

    impl Default for DnsTxtResolver {
        fn default() -> Self {
            Self {
                resolver: TokioAsyncResolver::tokio_from_system_conf()
                    .expect("failed to create resolver"),
            }
        }
    }

    impl atrium_identity::handle::DnsTxtResolver for DnsTxtResolver {
        async fn resolve(
            &self,
            query: &str,
        ) -> Result<Vec<String>, Box<dyn Error + Send + Sync + 'static>> {
            let txt_lookup = self.resolver.txt_lookup(query).await?;
            Ok(txt_lookup.iter().map(TXT::to_string).collect())
        }
    }
}
