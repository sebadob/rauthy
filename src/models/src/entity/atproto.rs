use std::{future::Future, ops::Deref, sync::Arc};

use actix_web::{
    cookie::Cookie,
    http::header::{self, HeaderValue},
    web, HttpRequest,
};
use atrium_api::{agent::Agent, com::atproto::server::get_session, types::Object};
use atrium_identity::{
    did::{CommonDidResolver, CommonDidResolverConfig, DEFAULT_PLC_DIRECTORY_URL},
    handle::{
        AtprotoHandleResolver, AtprotoHandleResolverConfig, DnsTxtResolver as DnsTxtResolverTrait,
    },
};
use atrium_oauth_client::{
    AtprotoClientMetadata, AuthMethod, AuthorizeOptions, CallbackParams, DefaultHttpClient,
    GrantType, KnownScope, OAuthClient, OAuthClientConfig, OAuthResolverConfig, Scope,
};
use cryptr::utils::secure_random_alnum;
use hickory_resolver::{proto::rr::rdata::TXT, TokioAsyncResolver};
use rauthy_api_types::atproto;
use rauthy_common::constants::{
    COOKIE_UPSTREAM_CALLBACK, PROVIDER_LINK_COOKIE, UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use time::OffsetDateTime;
use tracing::{debug, error};
use utoipa::{PartialSchema, ToSchema};

use crate::{
    api_cookie::ApiCookie,
    app_state::AppState,
    database::DB,
    entity::{
        auth_codes::AuthCode, auth_providers::AuthProviderLinkCookie, clients::Client, users::User,
    },
    AuthStep, AuthStepLoggedIn, ListenScheme,
};

use super::{
    auth_providers::{AuthProviderCallback, AuthProviderType},
    sessions::Session,
};

#[derive(Clone)]
pub struct AtprotoClient(
    Arc<
        OAuthClient<
            DB,
            DB,
            CommonDidResolver<DefaultHttpClient>,
            AtprotoHandleResolver<DnsTxtResolver, DefaultHttpClient>,
        >,
    >,
);

impl std::fmt::Debug for AtprotoClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AtprotoClient").finish()
    }
}

impl AtprotoClient {
    pub fn new(
        listen_scheme: &ListenScheme,
        public_url: &str,
    ) -> Result<Self, atrium_oauth_client::Error> {
        let http_client = Arc::new(DefaultHttpClient::default());

        let listen_scheme = match listen_scheme {
            ListenScheme::Http | ListenScheme::UnixHttp => "http",
            ListenScheme::Https | ListenScheme::HttpHttps | ListenScheme::UnixHttps => "https",
        };

        let client_metadata = AtprotoClientMetadata {
            client_id: format!("{listen_scheme}://{public_url}/auth/v1/atproto/client_metadata"),
            client_uri: String::new(),
            redirect_uris: vec![format!(
                "{listen_scheme}://{public_url}/auth/v1/atproto/callback"
            )],
            token_endpoint_auth_method: AuthMethod::None,
            grant_types: vec![GrantType::AuthorizationCode],
            scopes: vec![Scope::Known(KnownScope::Atproto)],
            jwks_uri: None,
            token_endpoint_auth_signing_alg: None,
        };

        let config = OAuthClientConfig {
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
        };

        OAuthClient::new(config).map(Arc::new).map(AtprotoClient)
    }
}

impl Deref for AtprotoClient {
    type Target = OAuthClient<
        DB,
        DB,
        CommonDidResolver<DefaultHttpClient>,
        AtprotoHandleResolver<DnsTxtResolver, DefaultHttpClient>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

impl ToSchema for DnsTxtResolver {}

impl PartialSchema for DnsTxtResolver {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::RefOr::T(utoipa::openapi::Schema::Object(
            utoipa::openapi::ObjectBuilder::new().build(),
        ))
    }
}

impl DnsTxtResolverTrait for DnsTxtResolver {
    async fn resolve(
        &self,
        query: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let txt_lookup = self.resolver.txt_lookup(query).await?;
        Ok(txt_lookup.iter().map(TXT::to_string).collect())
    }
}

pub trait AtprotoCallback {
    fn login_start<'a>(
        data: &'a web::Data<AppState>,
        payload: atproto::LoginRequest,
    ) -> impl Future<Output = Result<(Cookie<'a>, String, HeaderValue), ErrorResponse>>;

    fn login_finish<'a>(
        data: &'a web::Data<AppState>,
        req: &'a HttpRequest,
        payload: &'a atproto::CallbackRequest,
        session: Session,
    ) -> impl Future<Output = Result<(AuthStep, Cookie<'a>), ErrorResponse>>;
}

impl AtprotoCallback for AuthProviderCallback {
    async fn login_start<'a>(
        data: &'a web::Data<AppState>,
        payload: atproto::LoginRequest,
    ) -> Result<(Cookie<'a>, String, HeaderValue), ErrorResponse> {
        let slf = Self {
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

        slf.save().await?;

        let options = AuthorizeOptions {
            state: Some(slf.callback_id.clone()),
            ..Default::default()
        };

        match data.atproto.authorize(&payload.at_id, options).await {
            Ok(location) => {
                let cookie = ApiCookie::build(
                    COOKIE_UPSTREAM_CALLBACK,
                    &slf.callback_id,
                    UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS as i64,
                );
                let header =
                    HeaderValue::from_str(&location).expect("Location HeaderValue to be correct");

                Ok((cookie, slf.xsrf_token, header))
            }
            Err(error) => {
                error!(%error, "failed to build pushed authorization request for atproto");

                Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "failed to build pushed authorization request for atproto",
                ))
            }
        }
    }

    async fn login_finish<'a>(
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

        // request is valid -> fetch token for the user
        let params = CallbackParams {
            code: payload.code.clone(),
            state: Some(payload.state.clone()),
            iss: payload.iss.clone(),
        };
        // return early if we got any error
        let (session_manager, _) = data.atproto.callback(params).await.map_err(|error| {
            error!(%error, "failed to complete authorization callback for atproto");

            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "failed to complete authorization callback for atproto",
            )
        })?;

        let agent = Agent::new(session_manager);

        let (did, email, email_confirmed) = match agent.api.com.atproto.server.get_session().await {
            Ok(Object {
                data:
                    get_session::OutputData {
                        did,
                        email,
                        email_confirmed,
                        ..
                    },
                ..
            }) => (did, email, email_confirmed),
            Err(error) => {
                error!(%error, "failed to get session for atproto");

                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    "failed to get session for atproto",
                ));
            }
        };

        let link_cookie = ApiCookie::from_req(req, PROVIDER_LINK_COOKIE)
            .and_then(|value| AuthProviderLinkCookie::try_from(value.as_str()).ok());

        if email.is_none() {
            todo!()
        }

        let claims_user_id = did.to_string();

        let user_opt = match User::find_by_federation("atproto", &claims_user_id).await {
            Ok(user) => {
                debug!(
                    "found already existing user by federation lookup: {:?}",
                    user
                );
                Some(user)
            }
            Err(_) => {
                debug!("did not find already existing user by federation lookup - making sure email does not exist");

                if let Ok(mut user) = User::find_by_email(email.as_ref().unwrap().to_string()).await
                {
                    if let Some(link) = link_cookie.as_ref() {
                        if link.provider_id != "atproto" {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "bad provider_id in link cookie",
                            ));
                        }

                        if link.user_id != user.id {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "bad user_id in link cookie",
                            ));
                        }

                        if link.user_email != user.email {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "Invalid E-Mail",
                            ));
                        }

                        user.auth_provider_id = Some("atproto".to_owned());
                        user.federation_uid = Some(claims_user_id.clone());

                        Some(user)
                    } else {
                        return Err(ErrorResponse::new(ErrorResponseType::Forbidden, format!(
                            "User with email '{}' already exists but is not linked to this provider.",
                            user.email
                        )));
                    }
                } else {
                    None
                }
            }
        };
        debug!("user_opt:\n{:?}", user_opt);

        let now = OffsetDateTime::now_utc().unix_timestamp();
        let user = if let Some(mut user) = user_opt {
            let mut old_email = None;
            let mut forbidden_error = None;

            if user.federation_uid.is_none()
                || user.federation_uid.as_deref() != Some(&claims_user_id)
            {
                forbidden_error = Some("non-federated user or ID mismatch");
            }

            if user.auth_provider_id.as_deref() != Some("atproto") {
                forbidden_error = Some("invalid login from wrong auth provider");
            }

            if let Some(err) = forbidden_error {
                user.last_failed_login = Some(now);
                user.failed_login_attempts =
                    Some(user.failed_login_attempts.unwrap_or_default() + 1);
                user.save(old_email).await?;

                return Err(ErrorResponse::new(ErrorResponseType::Forbidden, err));
            }

            if Some(user.email.as_str()) != email.as_deref() {
                old_email = Some(user.email);
                user.email = email.as_ref().unwrap().to_string();
            }

            user.last_login = Some(now);
            user.last_failed_login = None;
            user.failed_login_attempts = None;

            user.save(old_email).await?;
            user
        } else {
            let new_user = User {
                email: email.as_ref().unwrap().to_string(),
                given_name: "N/A".to_string(),
                family_name: None,
                roles: Default::default(),
                enabled: true,
                email_verified: email_confirmed.unwrap_or(false),
                last_login: Some(now),
                language: Default::default(),
                auth_provider_id: Some("atproto".to_owned()),
                federation_uid: Some(claims_user_id.to_string()),
                ..Default::default()
            };
            User::create_federated(new_user).await?
        };

        user.check_enabled()?;
        user.check_expired()?;

        if link_cookie.is_some() {
            return Ok((
                AuthStep::ProviderLink,
                AuthProviderLinkCookie::deletion_cookie(),
            ));
        }

        let client = Client::default();

        let code = AuthCode::new(
            user.id.clone(),
            "atproto".to_owned(),
            Some(session.id.clone()),
            slf.req_code_challenge,
            slf.req_code_challenge_method,
            slf.req_nonce,
            vec!["atproto".to_owned()],
            client.auth_code_lifetime,
        );
        code.save().await?;

        let auth_step = AuthStep::LoggedIn(AuthStepLoggedIn {
            user_id: user.id,
            email: user.email,
            header_loc: (
                header::LOCATION,
                HeaderValue::from_str(&(format!("{}?code={}", slf.req_redirect_uri, code.id)))?,
            ),
            header_csrf: Session::get_csrf_header(&session.csrf_token),
            header_origin: None,
        });

        let cookie = ApiCookie::build(COOKIE_UPSTREAM_CALLBACK, "", 0);
        Ok((auth_step, cookie))
    }
}
