use crate::{
    database::{Cache, DB},
    entity::auth_providers::{AuthProvider, AuthProviderType},
    rauthy_config::RauthyConfig,
};
use atrium_api::types::string::Did;
use atrium_common::store::Store;
use atrium_identity::{
    did::{CommonDidResolver, CommonDidResolverConfig, DEFAULT_PLC_DIRECTORY_URL},
    handle::{
        AtprotoHandleResolver, AtprotoHandleResolverConfig, DnsTxtResolver as DnsTxtResolverTrait,
    },
};
use atrium_oauth::{
    AtprotoClientMetadata, AuthMethod, DefaultHttpClient, GrantType, KnownScope, OAuthClient,
    OAuthClientConfig, OAuthResolverConfig, Scope,
    store::{
        self,
        session::SessionStore,
        state::{InternalStateData, StateStore},
    },
};
use hickory_resolver::{
    Resolver, TokioResolver, config::ResolverConfig, name_server::TokioConnectionProvider,
    proto::rr::rdata::TXT,
};
use rauthy_api_types::auth_providers::ProviderRequest;
use rauthy_common::constants::{CACHE_TTL_AUTH_PROVIDER_CALLBACK, CACHE_TTL_SESSION};
use rauthy_error::ErrorResponse;
use std::{
    ops::Deref,
    sync::{Arc, OnceLock},
};
use tracing::info;
use utoipa::{PartialSchema, ToSchema};

pub static ATPROTO_CLIENT: OnceLock<Client> = OnceLock::new();

#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub struct Client(
    Arc<
        OAuthClient<
            DB,
            DB,
            CommonDidResolver<DefaultHttpClient>,
            AtprotoHandleResolver<DnsTxtResolver, DefaultHttpClient>,
        >,
    >,
);

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Client").finish()
    }
}

impl Client {
    pub fn get() -> Self {
        ATPROTO_CLIENT
            .get_or_init(|| Client::new().expect("failed to initialize ATProto client"))
            .clone()
    }

    fn new() -> Result<Self, atrium_oauth::Error> {
        let config = RauthyConfig::get();

        let http_client = Arc::new(DefaultHttpClient::default());

        let scopes = vec![
            Scope::Unknown("transition:email".to_owned()),
            Scope::Known(KnownScope::Atproto),
            Scope::Known(KnownScope::TransitionGeneric),
        ];

        let redirect_uris = vec![RauthyConfig::get().provider_callback_uri.clone()];

        let client_id = if config.vars.dev.dev_mode {
            let mut url = reqwest::Url::parse("http://localhost").unwrap();

            for redirect_uri in &redirect_uris {
                url.query_pairs_mut()
                    .append_pair("redirect_uri", redirect_uri);
            }

            if !scopes.is_empty() {
                let scope: Vec<_> = scopes.iter().map(Scope::as_ref).collect();

                url.query_pairs_mut().append_pair("scope", &scope.join(" "));
            }

            url.to_string()
        } else {
            format!(
                "{}/auth/v1/atproto/client_metadata",
                &config.pub_url_with_scheme
            )
        };

        let client_metadata = AtprotoClientMetadata {
            client_id,
            client_uri: None,
            redirect_uris,
            token_endpoint_auth_method: AuthMethod::None,
            grant_types: vec![GrantType::AuthorizationCode],
            scopes,
            jwks_uri: None,
            token_endpoint_auth_signing_alg: None,
        };

        let config = OAuthClientConfig {
            client_metadata,
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

        OAuthClient::new(config).map(Arc::new).map(Client)
    }

    pub async fn init_provider() -> Result<(), ErrorResponse> {
        info!("Initializing ATProto provider");

        let config = RauthyConfig::get();

        let payload = ProviderRequest {
            name: "ATProto".to_owned(),
            typ: AuthProviderType::Custom.into(),
            enabled: true,
            issuer: "atproto".to_owned(),
            authorization_endpoint: String::new(),
            token_endpoint: String::new(),
            userinfo_endpoint: String::new(),
            jwks_endpoint: None,
            use_pkce: true,
            client_secret_basic: false,
            client_secret_post: false,
            client_id: "rauthy".to_owned(),
            client_secret: None,
            scope: String::new(),
            admin_claim_path: None,
            admin_claim_value: None,
            mfa_claim_path: None,
            mfa_claim_value: None,
        };

        match AuthProvider::find_by_iss("atproto".to_owned()).await {
            Ok(provider) if !config.vars.atproto.enable => {
                AuthProvider::delete(&provider.id).await?;
            }
            Err(_) if config.vars.atproto.enable => {
                AuthProvider::create(payload).await?;
            }
            _ => {}
        };

        Ok(())
    }
}

impl Deref for Client {
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
    resolver: TokioResolver,
}

impl Default for DnsTxtResolver {
    fn default() -> Self {
        Self {
            resolver: Resolver::builder_with_config(
                ResolverConfig::default(),
                TokioConnectionProvider::default(),
            )
            .build(),
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

impl Store<String, InternalStateData> for DB {
    type Error = hiqlite::Error;

    async fn get(&self, key: &String) -> Result<Option<InternalStateData>, Self::Error> {
        let Some(value) = Self::hql().get_bytes(Cache::Atproto, key).await? else {
            return Ok(None);
        };

        let state = serde_json::from_slice(&value)?;

        Ok(Some(state))
    }

    async fn set(&self, key: String, value: InternalStateData) -> Result<(), Self::Error> {
        let value = serde_json::to_vec(&value)?;

        Self::hql()
            .put_bytes(Cache::Atproto, key, value, CACHE_TTL_AUTH_PROVIDER_CALLBACK)
            .await
    }

    async fn del(&self, key: &String) -> Result<(), Self::Error> {
        Self::hql().delete(Cache::Atproto, key.to_owned()).await
    }

    async fn clear(&self) -> Result<(), Self::Error> {
        Self::hql().clear_cache(Cache::Atproto).await
    }
}

impl StateStore for DB {}

impl Store<Did, store::session::Session> for DB {
    type Error = hiqlite::Error;

    async fn get(&self, key: &Did) -> Result<Option<store::session::Session>, Self::Error> {
        let Some(value) = Self::hql().get_bytes(Cache::Atproto, key.as_str()).await? else {
            return Ok(None);
        };

        let session = serde_json::from_slice(&value)?;

        Ok(Some(session))
    }

    async fn set(&self, key: Did, value: store::session::Session) -> Result<(), Self::Error> {
        let value = serde_json::to_vec(&value)?;

        Self::hql()
            .put_bytes(Cache::Atproto, key.to_string(), value, CACHE_TTL_SESSION)
            .await
    }

    async fn del(&self, key: &Did) -> Result<(), Self::Error> {
        Self::hql().delete(Cache::Atproto, key.to_string()).await
    }

    async fn clear(&self) -> Result<(), Self::Error> {
        Self::hql().clear_cache(Cache::Atproto).await
    }
}

impl SessionStore for DB {}
