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
    GrantType, KnownScope, OAuthClientConfig, OAuthResolverConfig, Scope,
};
use cryptr::utils::secure_random_alnum;
use rauthy_api_types::atproto;
use rauthy_common::constants::{
        COOKIE_UPSTREAM_CALLBACK, PROVIDER_LINK_COOKIE,
        UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS,
    };
use rauthy_error::{ErrorResponse, ErrorResponseType};
use resolvers::DnsTxtResolver;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
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
    sessions::Session,
};

type OAuthClient = atrium_oauth_client::OAuthClient<
    DB,
    DB,
    CommonDidResolver<DefaultHttpClient>,
    AtprotoHandleResolver<DnsTxtResolver, DefaultHttpClient>,
>;

#[allow(clippy::type_complexity)]
static CLIENT: OnceCell<OAuthClient> = OnceCell::const_new();

async fn init_oauth_client(
    public_url: String,
    redirect_uri: String,
) -> Result<OAuthClient, atrium_oauth_client::Error> {
    let http_client = Arc::new(DefaultHttpClient::default());

    let client_metadata = AtprotoClientMetadata {
        client_id: public_url.clone(),
        client_uri: public_url.clone(),
        redirect_uris: vec![redirect_uri],
        token_endpoint_auth_method: AuthMethod::PrivateKeyJwt,
        grant_types: vec![GrantType::AuthorizationCode, GrantType::RefreshToken],
        scopes: [KnownScope::Atproto]
            .into_iter()
            .map(Scope::Known)
            .collect(),
        jwks_uri: Some(format!("{public_url}/oidc/certs")),
        token_endpoint_auth_signing_alg: Some(String::from("ES256")),
    };

    OAuthClient::new(OAuthClientConfig {
        client_metadata: client_metadata.clone(),
        keys: None,
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
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Callback {}

impl Callback {
    /// returns (encrypted cookie, xsrf token, location header, optional allowed origins)
    pub async fn login_start<'a>(
        data: &'a web::Data<AppState>,
        payload: atproto::LoginRequest,
    ) -> Result<(Cookie<'a>, String, HeaderValue), ErrorResponse> {
        let callback = AuthProviderCallback {
            callback_id: secure_random_alnum(32),
            xsrf_token: secure_random_alnum(32),
            typ: AuthProviderType::Custom,

            req_client_id: String::from("atproto"),
            req_scopes: None,
            req_redirect_uri: payload.redirect_uri.clone(),
            req_state: None,
            req_nonce: None,
            req_code_challenge: None,
            req_code_challenge_method: None,

            provider_id: String::from("atproto"),

            pkce_challenge: payload.pkce_challenge,
        };

        callback.save().await?;

        let client = CLIENT
            .get_or_try_init(|| async move {
                init_oauth_client(data.public_url.clone(), payload.redirect_uri.clone()).await
            })
            .await
            .unwrap();

        let options = AuthorizeOptions {
            state: Some(callback.callback_id.clone()),
            ..Default::default()
        };
        let location = client.authorize(&payload.at_id, options).await.unwrap();

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

        let req_redirect_uri = slf.req_redirect_uri.clone();

        let client = CLIENT
            .get_or_try_init(|| async move {
                init_oauth_client(data.public_url.clone(), req_redirect_uri).await
            })
            .await
            .unwrap();

        // request is valid -> fetch token for the user
        let params = CallbackParams {
            code: payload.code.clone(),
            state: Some(payload.state.clone()),
            iss: payload.iss.clone(),
        };
        // return early if we got any error
        let (oauth_session, _) = client.callback(params).await.unwrap();
        // error!("{}", err);
        // return Err(ErrorResponse::new(ErrorResponseType::Internal, err));

        let token_set = oauth_session.token_set().await.unwrap();

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
            .unwrap();
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

mod resolvers {
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
