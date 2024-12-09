use actix_web::{cookie::Cookie, http::header::HeaderValue, web};
use atrium_identity::{
    did::CommonDidResolver,
    handle::{AtprotoHandleResolver, DnsTxtResolver as DnsTxtResolverTrait},
};
use atrium_oauth_client::{AuthorizeOptions, DefaultHttpClient, OAuthClient};
use cryptr::utils::secure_random_alnum;
use hickory_resolver::{proto::rr::rdata::TXT, TokioAsyncResolver};
use rauthy_api_types::atproto;
use rauthy_common::constants::{COOKIE_UPSTREAM_CALLBACK, UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use tracing::error;

use crate::{api_cookie::ApiCookie, app_state::AppState, database::DB};

use super::auth_providers::{AuthProviderCallback, AuthProviderType};

pub type AtprotoClient = OAuthClient<
    DB,
    DB,
    CommonDidResolver<DefaultHttpClient>,
    AtprotoHandleResolver<DnsTxtResolver, DefaultHttpClient>,
>;

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
    async fn login_start(
        data: &web::Data<AppState>,
        payload: atproto::LoginRequest,
    ) -> Result<(Cookie<'_>, String, HeaderValue), ErrorResponse>;
}

impl AtprotoCallback for AuthProviderCallback {
    async fn login_start(
        data: &web::Data<AppState>,
        payload: atproto::LoginRequest,
    ) -> Result<(Cookie<'_>, String, HeaderValue), ErrorResponse> {
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
}
